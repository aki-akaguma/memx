// build.rs
use rustc_version as rs_v;

fn main() {
    let rt_version = rs_v::version().unwrap();
    //
    if rt_version < rs_v::Version::parse("1.46.0").unwrap() {
        println!("cargo:warning=rustc version should be 1.46.0 or more.");
    }
}
