// build.rs

fn main() {
    pkg_config::Config::new().probe("libmosquitto").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}