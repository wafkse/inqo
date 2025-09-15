use std::path::PathBuf;

use bindgen::Builder;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-link-lib=pam");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=bind/pam.h");

    let bindings = Builder::default()
        .header("bind/pam.h")
        .ctypes_prefix("libc")
        .use_core()
        .opaque_type("pam_handle_t")
        .allowlist_var("PAM_(.+)")
        .allowlist_function("pam_(.+)")
        .blocklist_function("pam_sm_(.+)")
        .emit_builtins()
        .generate()?;

    let target_output = std::env::var("OUT_DIR").map(PathBuf::from)?;

    bindings.write_to_file(target_output.join("pam.rs"))?;

    Ok(())
}
