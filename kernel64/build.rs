use std::path::Path;

fn main() {
    let local_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    // The default is here only because we want to just run a compile without
    // needing to know what the correct value is.
    let kernel64_load = std::env::var("KERNEL64_LOAD_TARGET").unwrap_or("0".to_string());

    println!("cargo:warning=Not a warning. Just FYI start address is {}.", kernel64_load);

    println!(
        "cargo:rustc-link-arg-bins=--defsym=KERNEL64_LOAD_TARGET={}",
        kernel64_load);

    println!(
        "cargo:rustc-link-arg-bins=--script={}",
        local_path.join("link.ld").display()
    )
}
