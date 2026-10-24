//! Windows only: embed the app icon and version info (VERSIONINFO) into `videobox.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! On every other target this does nothing (`winresource` is only a build-dependency on Windows
//! hosts). A missing resource compiler is a warning, unless `VIDEOBOX_REQUIRE_WINRES=1` (for
//! release builds) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/videobox.ico");
    println!("cargo:rerun-if-env-changed=VIDEOBOX_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    windows::embed_resources();
}

#[cfg(windows)]
mod windows {
    pub fn embed_resources() {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../assets/app-icon/videobox.ico")
            .set("ProductName", "VideoBox")
            .set("FileDescription", "VideoBox video editor")
            .set("LegalCopyright", "Copyright (c) the VideoBox contributors. MIT OR Apache-2.0.")
            .set("OriginalFilename", "videobox.exe")
            .set("InternalName", "videobox");
        if let Err(e) = res.compile() {
            if std::env::var_os("VIDEOBOX_REQUIRE_WINRES").is_some() {
                panic!("embedding Windows resources failed: {e}");
            }
            println!("cargo:warning=videobox.exe built without icon/version resources: {e}");
        }
    }
}

/// Cross-compiling for Windows from macOS or Linux: `winresource` is not available on this host.
#[cfg(not(windows))]
mod windows {
    pub fn embed_resources() {
        println!("cargo:warning=videobox.exe built without icon/version resources (cross-compiled from a non-Windows host)");
    }
}
