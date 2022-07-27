use std::process::Command;
#[cfg(not(target_os = "windows"))]
fn main() {
    Command::new("packfolder")
        .args(["views","src/resource.rc","-binary"])
        .output()
        .unwrap();
    println!("cargo:rerun-if-changed=/views");
}

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("windows_platform_resource/icon.ico")
        .set_manifest_file("windows_platform_resource/dpi-aware.mainfest");
    res.compile().unwrap();
    Command::new("packfolder")
        .args(["views","src/resource.rc","-binary"])
        .output()
        .unwrap();
    println!("cargo:rerun-if-changed=/views");
}