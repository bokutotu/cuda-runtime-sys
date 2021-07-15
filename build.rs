extern crate bindgen;

use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .ctypes_prefix("::libc")
        .allowlist_type("^cuda.*")
        .allowlist_type("^surfaceReference")
        .allowlist_type("^textureReference")
        .allowlist_var("^cuda.*")
        .allowlist_function("^cuda.*")
        .default_alias_style(bindgen::AliasVariation::TypeAlias)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .clang_arg("-I")
        .clang_arg("/usr/local/cuda/include".to_string())
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings")/*  */;

    let out_path = PathBuf::from("src/").join("cuda_runtime.rs");
    bindings.write_to_file(out_path).expect("Unable to write");
}