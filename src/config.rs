use directories::ProjectDirs;
use serde_derive::Deserialize;
use std::fs;

//config.toml where directory and server config details are entered
#[derive(Deserialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u32,
}
impl Config {
    //setting Config feilds to config file feilds
    pub fn set_config() -> Self {
        // get the config directory envirement variable from your Os
        let proj_dirs = ProjectDirs::from("dev", "jg_software", "bolt").unwrap();
        let config_dir = proj_dirs.config_dir();

        /*
            Config enviremonts of the different operationg systems

            Linux:   /home/alice/.config/bolt
            Windows: C:\Users\Alice\AppData\Roaming\jg_software\bolt
            macOS:   /Users/Alice/Library/Application Support/dev.jg_software.bolt
        */
        let config_file = fs::read_to_string(&config_dir.join("config.toml"));

        if !config_file.is_ok() {
            eprintln!("no config.toml exists in your $CONFIG/bolt directory\n");
        }
        // where the Config struct feilds are set to the strings in config.toml file
        let config: Config = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => Self {
                ip: "127.0.0.1".to_string(),
                port: 4000,
            },
        };

        Self {
            ip: config.ip,
            port: config.port,
        }
    }
}
