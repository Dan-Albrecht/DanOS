use std::path::Path;

fn main() {
    let local_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Hack to force this always to rebuild as we might change the environment variable and no any of the files
    println!("cargo:rerun-if-changed=BLAH");

    // The default is here only because we want to just run a compile without
    // needing to know what the correct value is.
    let kernel32_load = std::env::var("KERNEL32_LOAD_TARGET").unwrap_or("0".to_string());

    println!("cargo:warning=Not a warning. Just FYI start address is {}.", kernel32_load);

    println!(
        "cargo:rustc-link-arg-bins=--defsym=KERNEL32_LOAD_TARGET={}",
        kernel32_load);

    println!(
        "cargo:rustc-link-arg-bins=--script={}",
        local_path.join("link.ld").display()
    )
}
