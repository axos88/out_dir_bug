extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main() {
    includedir_codegen::start("ASSETS")
        .dir("../frontend/dist", Compression::Gzip)
        .build("data.rs")
        .unwrap();
}