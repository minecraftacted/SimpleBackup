fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let dest = format!("./target/{}/config.yml", profile);
    std::fs::copy("./resources/config.yml", dest).unwrap();
}