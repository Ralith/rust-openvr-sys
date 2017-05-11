extern crate cmake;

fn main() {
    let mut cfg = cmake::Config::new("openvr");

    // Work around broken cmake build
    #[cfg(windows)]
    cfg.cxxflag("/DWIN32");

    let dst = cfg.build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=openvr_api");

    #[cfg(target_os="linux")]
    println!("cargo:rustc-link-lib=stdc++");
}
