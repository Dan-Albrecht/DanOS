use std::path::Path;

fn main() {
    let local_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    println!("cargo:rerun-if-env-changed=STAGE2_RUST_LOAD_TARGET");

    // The default is here only because we want to just run a compile without
    // needing to know what the correct value is.
    let stage2_load = std::env::var("STAGE2_RUST_LOAD_TARGET").unwrap_or("0x8000".to_string());

    println!("cargo:warning=Not a warning. Just FYI start address is {}.", stage2_load);

    println!(
        "cargo:rustc-link-arg-bins=--defsym=STAGE2_RUST_LOAD_TARGET={}",
        stage2_load);

    println!(
        "cargo:rustc-link-arg-bins=--script={}",
        local_path.join("link.ld").display()
    )
}
