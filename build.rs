extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=parted");
    println!("cargo:rustc-link-lib=parted-fs-resize");

    let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .rustified_non_exhaustive_enum("PedDeviceType")
    .rustified_non_exhaustive_enum("PedUnit")
    .rustified_non_exhaustive_enum("_PedDiskFlag")
    .rustified_non_exhaustive_enum("_PedDiskTypeFeature")
    .rustified_non_exhaustive_enum("_PedExceptionOption")
    .rustified_non_exhaustive_enum("_PedExceptionType")
    .rustified_non_exhaustive_enum("_PedPartitionFlag")
    .rustified_non_exhaustive_enum("_PedPartitionType")
    .generate()
    .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
