use chrono::DateTime;
use config::Config;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::env;
use chrono::Local;
use chrono::Duration;
use uuid::Uuid;
fn main() {
    let loaded_config = find_config_file();

    create_directory_to_save_backup_if_not_exists(&loaded_config);

    let day_and_time = loaded_config.directory_to_save_backup.to_string() + &Local::now().format("%Y-%m-%d-%H-%M-").to_string();
    let backup_version_name = day_and_time + &Uuid::new_v4().to_string();
    let day_file_path = Path::new(&backup_version_name);
    fs::create_dir(day_file_path).unwrap();

    for directory_to_backup in &loaded_config.directories_to_backup {
        let mut dest = day_file_path.to_path_buf();
        if PathBuf::from(directory_to_backup).is_dir()  {
            dest = day_file_path.to_path_buf().join(PathBuf::from(directory_to_backup).file_name().unwrap());
        }
        match copy(Path::new(directory_to_backup), dest.as_path()) {
            Ok(()) => println!("{} was successfully backed up", directory_to_backup),
            Err(e) => println!("{:?}", e),
        }
    }
    let _ = delete_old_backups(&loaded_config);
}
fn copy(src: &Path, dest: &Path) -> Result<(), std::io::Error> {
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }
    if !src.exists() {
        println!("{} does not exist", src.display());
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }
    if (!src.is_dir()) && (!src.is_file()) && (!src.is_symlink()){
        panic!("damedesu");
    }
    if src.is_file() || src.is_symlink() {
        fs::copy(src, dest.join(src.file_name().unwrap()))?;
        return Ok(());
    }
    if src.is_dir() {
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dir_path = dest.join(entry.file_name());
            if entry_path.is_dir() {
                copy(&entry_path, &dir_path)?;
            } else {
                fs::copy(&entry_path, &dir_path)?;
            }
        }
    }
    Ok(())
}
fn delete_old_backups(loaded_config: &LoadedConfig) -> Result<(), std::io::Error> {
    for backup in fs::read_dir(Path::new(&loaded_config.directory_to_save_backup))? {
        let backup = backup.unwrap();
        let created_at:DateTime<Local> = DateTime::from(backup.metadata().unwrap().created().unwrap());
        let storage_period_days = Duration::days(loaded_config.storage_period_days);
        let now:DateTime<Local> = Local::now();
        if now-storage_period_days > created_at{
            let _ = fs::remove_dir_all(backup.path());
        }
    }
    Ok(())
}
fn find_config_file() ->LoadedConfig{
    let temp_current_dir = env::current_dir().unwrap();
    let _ = env::set_current_dir(env::current_exe().unwrap().parent().unwrap());
    let loaded_config = LoadedConfig::from_config_file();
    let _ = env::set_current_dir(temp_current_dir);
    loaded_config
}
fn create_directory_to_save_backup_if_not_exists(loaded_config:&LoadedConfig) {
    if !(Path::new(&loaded_config.directory_to_save_backup).try_exists().expect("Failed to check is exists directory_to_save_backup")) {
        let _ = fs::create_dir(&loaded_config.directory_to_save_backup);
    }
}
struct LoadedConfig {
    config_file:Config,
    pub storage_period_days:i64,
    pub directory_to_save_backup:String,
    pub directories_to_backup: Vec<String>,
}
impl LoadedConfig {
    pub fn from_config_file() -> LoadedConfig {
        let config_file:Config = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .expect("Failed to load config file");
        LoadedConfig {
            config_file: config_file.clone(),
            storage_period_days: config_file.get_int("storage_period_days").expect("Failed to get storage_period_days from config file"),
            directory_to_save_backup: config_file.get_string("directory_to_save_backup").expect("Failed to get storage_period_days from config file"),
            directories_to_backup: config_file
                .get_array("directories_to_backup")
                .expect("Failed to get directories_to_backup from config file")
                .iter()
                .map(|element| element.kind.to_string())
                .collect()
        }
    }
}