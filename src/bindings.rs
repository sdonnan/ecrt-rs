// IgH EtherCAT Master Rust Bindings. Copyright (C) 2019 A. Stuart Donnan
// Please see README.md for complete usage conditions.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// just load the bindings from the generated file
include!(concat!(env!("OUT_DIR"), "/generated-bindings.rs"));
