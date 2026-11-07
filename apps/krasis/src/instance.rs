//! One Krasis at a time: a second start hands its files to the running one and ends.
//!
//! The running Krasis listens on a localhost port named in `<data dir>/instance.port`; a later
//! start sends its file paths there as one JSON line and waits for `ok`.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Duration;

/// A message from a later start.
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Handoff {
    pub files: Vec<PathBuf>,
    pub module: Option<String>,
}

pub enum Start {
    /// Another Krasis took the files.
    Forwarded,
    /// This is the Krasis that runs; later starts arrive on the receiver.
    Primary(Option<Receiver<Handoff>>, Option<Sender<Handoff>>),
}

fn port_file() -> Option<PathBuf> {
    crate::data_dir().map(|d| d.join("instance.port"))
}

/// Hand the files to a running Krasis, or become the running one.
pub fn start(handoff: &Handoff, new_instance: bool) -> Start {
    if !new_instance && let Some(text) = port_file().and_then(|p| std::fs::read_to_string(p).ok()) {
        // "<port> <process id>": the running Krasis may come forward (Windows asks the process
        // the user is working in, this one, to allow it).
        let mut parts = text.split_whitespace();
        let port = parts.next().and_then(|s| s.parse::<u16>().ok());
        if let Some(pid) = parts.next().and_then(|s| s.parse::<u32>().ok()) {
            crate::memory::allow_foreground(pid);
        }
        if let Some(port) = port
            && forward(port, handoff).is_ok()
        {
            return Start::Forwarded;
        }
    }
    match listen() {
        Some((rx, tx)) => Start::Primary(Some(rx), Some(tx)),
        None => Start::Primary(None, None),
    }
}

fn forward(port: u16, handoff: &Handoff) -> std::io::Result<()> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(400))?;
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    let line = serde_json::to_string(handoff).map_err(std::io::Error::other)?;
    stream.write_all(line.as_bytes())?;
    stream.write_all(b"\n")?;
    let mut answer = String::new();
    BufReader::new(stream).read_line(&mut answer)?;
    if answer.trim() == "ok" { Ok(()) } else { Err(std::io::Error::other("no answer")) }
}

fn listen() -> Option<(Receiver<Handoff>, Sender<Handoff>)> {
    let listener = TcpListener::bind(("127.0.0.1", 0)).ok()?;
    let port = listener.local_addr().ok()?.port();
    let path = port_file()?;
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    std::fs::write(&path, format!("{port} {}", std::process::id())).ok()?;
    let (tx, rx) = channel();
    let sender = tx.clone();
    std::thread::Builder::new()
        .name("krasis-instance".into())
        .spawn(move || {
            for stream in listener.incoming().flatten() {
                let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));
                let mut reader = BufReader::new(&stream);
                let mut line = String::new();
                // One line of at most 1 MiB: a later start's file list.
                if reader.by_ref().take(1 << 20).read_line(&mut line).is_err() {
                    continue;
                }
                if let Ok(handoff) = serde_json::from_str::<Handoff>(&line) {
                    let mut s = &stream;
                    let _ = s.write_all(b"ok\n");
                    if tx.send(handoff).is_err() {
                        break;
                    }
                    crate::wake();
                }
            }
        })
        .ok()?;
    Some((rx, sender))
}

/// Forget the port file when this Krasis ends.
pub fn release() {
    if let Some(p) = port_file() {
        let _ = std::fs::remove_file(p);
    }
}

use std::io::Read as _;
