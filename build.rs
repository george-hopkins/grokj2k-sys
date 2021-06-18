fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    pkg_config::Config::new()
        .atleast_version("9.2.0")
        .probe("libgrokj2k")
        .unwrap();
}
