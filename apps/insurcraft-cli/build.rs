//! Windows only: embed version info (VERSIONINFO) into `insurcraft-cli.exe`, so Explorer's
//! Details tab, `(Get-Item insurcraft-cli.exe).VersionInfo` and inventory tools can read the version
//! and publisher, as they can for `insurcraft.exe` (`apps/insurcraft/build.rs`).
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `INSURCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=INSURCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "InsurCraft")
        .set("FileDescription", "InsurCraft command-line tool")
        .set("LegalCopyright", "Copyright (c) the PdfCraft contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "insurcraft-cli.exe")
        .set("InternalName", "insurcraft-cli");
    if let Err(e) = res.compile() {
        if std::env::var_os("INSURCRAFT_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=insurcraft-cli.exe built without version resources: {e}");
    }
}
