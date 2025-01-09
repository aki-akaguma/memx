// build.rs

fn main() {
    //
    if rustversion::cfg!(before(1.77)) {
        //println!("cargo:warning=before v1.77");
        println!("cargo:rustc-cfg=has_not_static_mut_refs");
    }
}
