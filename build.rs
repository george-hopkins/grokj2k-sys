fn main() {
    pkg_config::Config::new()
        .atleast_version("9.2.0")
        .probe("libgrokj2k")
        .unwrap();
}
