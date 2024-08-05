fn main() {
    if !cfg!(target_os = "linux") {
        panic!("bindings can only be generated for linux targets")
    }

    let bindings = bindgen::Builder::default()
        .header_contents("bindings.h", "#include<linux/version.h>")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("unable to generate bindings");

    let out_dir = std::path::PathBuf::from(
        std::env::var("OUT_DIR").expect("could not find output directory"),
    );

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("unable to write bindings")
}
