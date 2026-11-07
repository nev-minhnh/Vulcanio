//! Register Vulcanio for the file types its apps open (per user, Windows): Vulcanio appears in
//! Open With for each type and in Settings › Apps › Default apps, where the user chooses it as
//! the default (Windows lets only the user make that choice).

#[cfg(windows)]
pub fn register() -> Result<(), String> {
    use std::os::windows::process::CommandExt as _;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe = exe.display().to_string();
    let reg = |args: &[&str]| -> Result<(), String> {
        let out = std::process::Command::new("reg").args(args).creation_flags(CREATE_NO_WINDOW).output().map_err(|e| e.to_string())?;
        if out.status.success() { Ok(()) } else { Err(String::from_utf8_lossy(&out.stderr).trim().to_string()) }
    };
    let classes = r"HKCU\Software\Classes";
    let progid = format!(r"{classes}\Vulcanio.Document");
    reg(&["add", &progid, "/ve", "/d", "Vulcanio document", "/f"])?;
    reg(&["add", &format!(r"{progid}\DefaultIcon"), "/ve", "/d", &format!("\"{exe}\",0"), "/f"])?;
    reg(&["add", &format!(r"{progid}\shell\open\command"), "/ve", "/d", &format!("\"{exe}\" \"%1\""), "/f"])?;
    let app_key = format!(r"{classes}\Applications\vulcanio.exe");
    reg(&["add", &app_key, "/v", "FriendlyAppName", "/d", "Vulcanio", "/f"])?;
    reg(&["add", &format!(r"{app_key}\shell\open\command"), "/ve", "/d", &format!("\"{exe}\" \"%1\""), "/f"])?;
    let caps = r"HKCU\Software\Vulcanio\Capabilities";
    reg(&["add", caps, "/v", "ApplicationName", "/d", "Vulcanio", "/f"])?;
    reg(&["add", caps, "/v", "ApplicationDescription", "/d", "Vulcanio - The all-in-one studio", "/f"])?;
    for ext in crate::modules::all_extensions() {
        let dot = format!(".{ext}");
        reg(&["add", &format!(r"{classes}\{dot}\OpenWithProgids"), "/v", "Vulcanio.Document", "/d", "", "/f"])?;
        reg(&["add", &format!(r"{app_key}\SupportedTypes"), "/v", &dot, "/d", "", "/f"])?;
        reg(&["add", &format!(r"{caps}\FileAssociations"), "/v", &dot, "/d", "Vulcanio.Document", "/f"])?;
    }
    reg(&["add", r"HKCU\Software\RegisteredApplications", "/v", "Vulcanio", "/d", r"Software\Vulcanio\Capabilities", "/f"])?;
    Ok(())
}

#[cfg(not(windows))]
pub fn register() -> Result<(), String> {
    Err("File types are registered by the installer on this system.".into())
}
