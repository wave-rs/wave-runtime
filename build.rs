extern crate cmake;

fn main() {
    // Builds the project in the directory located in `libdouble`, installing it
    // into $OUT_DIR
    let dst = cmake::build("wamr-embed");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=wamr");
}