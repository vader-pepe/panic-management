extern crate bindgen;
extern crate cc;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Couldnt generate bindings!");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldnt generate bindings!");
}
