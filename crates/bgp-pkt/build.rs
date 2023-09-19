use std::error::Error;
use std::fs::File;
use capi_gen;

fn main() -> Result<(), Box<dyn Error>> {

    let output = File::create("src/capi_autogen.rs")?;
    capi_gen::run("src/**/*.rs", output)
}