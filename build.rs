fn main() {
    std::fs::copy("./resources/SimpleBackup_config.yml", "./target/debug/SimpleBackup_config.yml").unwrap();
    std::fs::copy("./resources/TransferBackup_config.yml", "./target/debug/TransferBackup_config.yml").unwrap();

}