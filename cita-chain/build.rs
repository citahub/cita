use std::env;

use util::build_info::gen_build_info;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    gen_build_info(out_dir.as_ref(), "build_info.rs");
}
