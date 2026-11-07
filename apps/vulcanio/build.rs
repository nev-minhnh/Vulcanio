//! Windows: the Vulcanio icon and version resource, and the main thread's stack (FigCraft lays out
//! deep component trees on it; 8 MiB as in its own app).
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/vulcanio.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    println!("cargo:rustc-link-arg-bins={}", if msvc { "/STACK:8388608" } else { "-Wl,--stack,8388608" });
    if msvc {
        // PhotoCraft prints PDFs through windows.data.pdf.dll only when asked: load it then.
        println!("cargo:rustc-link-arg-bins=/DELAYLOAD:windows.data.pdf.dll");
        println!("cargo:rustc-link-arg-bins=delayimp.lib");
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/vulcanio.ico")
        .set("ProductName", "Vulcanio")
        .set("FileDescription", "Vulcanio - The all-in-one studio")
        .set("LegalCopyright", "Vulcanio and the Craft apps it hosts: MIT OR Apache-2.0.")
        .set("OriginalFilename", "vulcanio.exe")
        .set("InternalName", "vulcanio");
    if let Err(e) = res.compile() {
        println!("cargo:warning=vulcanio.exe built without icon/version resources: {e}");
    }
}
