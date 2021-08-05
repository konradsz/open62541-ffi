use std::{collections::HashSet, env, path::PathBuf};

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    let dst = cmake::build("open62541");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=open62541");
    println!("cargo:rerun-if-changed=wrapper.h");

    let ignored_macros = IgnoreMacros(vec!["IPPORT_RESERVED".into()].into_iter().collect());

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}/include", dst.display()))
        .header("wrapper.h")
        .parse_callbacks(Box::new(ignored_macros))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
