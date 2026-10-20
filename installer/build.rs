fn main() {
    println!("cargo:rerun-if-changed=windows/encoderbridge-setup.rc");
    println!("cargo:rerun-if-changed=windows/encoderbridge-setup.manifest");
    // Compiled only on Windows, where the build-dependency is present.
    // Linux CI still builds this crate; it just does not embed a resource.
    #[cfg(target_os = "windows")]
    embed_resource::compile("windows/encoderbridge-setup.rc", embed_resource::NONE);
}
