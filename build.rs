// IgH EtherCAT Master Rust Bindings. Copyright (C) 2019 A. Stuart Donnan
// Please see README.md for complete usage conditions.

extern crate bindgen;

use std::env;
use std::path::PathBuf;

/**
 * Set ETHERLAB_PREFIX in your environment to use an Etherlab install prefix that is not the
 * default (/opt)
 */

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_path = PathBuf::from(env::var("ETHERLAB_PREFIX")
                                 .unwrap_or("/opt".to_owned()));

    println!("cargo:rustc-link-search={}", 
             lib_path
                .join("etherlab/lib")
                .to_str()
                .unwrap()
             );
    println!("cargo:rustc-link-lib=ethercat");

    let bindings = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .header("wrapper.h")
        .clang_arg(format!("-I{}", 
                           lib_path
                                .join("etherlab/include")
                                .to_str()
                                .unwrap()
                          )
                   )
        .whitelist_function("ecrt_.*")
        .whitelist_type("ec_.*")
        .default_enum_style(
            bindgen::EnumVariation::ModuleConsts
        )
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("generated-bindings.rs"))
        .expect("Couldn't write bindings!")
}
