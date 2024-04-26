fn main() {
    std::fs::copy("./resources/config.yml", "./target/debug/config.yml").unwrap();
}