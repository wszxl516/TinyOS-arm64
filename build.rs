fn main() {
    println!("cargo:rerun-if-changed=src/linker.ld");
    println!("cargo:rerun-if-changed=.cargo/config");
}