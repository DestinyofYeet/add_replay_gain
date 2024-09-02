use configparser::ini::Ini;
use std::fs::File;
use std::io::Read;
use std::process::exit;

#[derive(Debug, Clone)]
pub struct Config {
    pub watch_path: String,

    pub metaflac_path: String,
    pub metaflac_flags: String,

    pub mp3gain_path: String,
    pub mp3gain_flags: String,

    pub uptime_url: String,

    pub enable_replay_gain: bool,
    pub enable_remove_comment: bool,
}

impl Config {
    pub fn parse(path: &str) -> Option<Config> {
        let mut file = match File::open(path){
            Err(e) => panic!("Failed to open config file '{}' because: {}", path, e),
            Ok(file) => file
        };

        let mut file_content = String::new();

        match file.read_to_string(&mut file_content){
            Err(e) => panic!("Could not config file contents because {}", e),
            Ok(_) => {

                let mut config = Ini::new();
                match config.read(file_content) {
                    Err(e) => panic!("Failed to parse config file because {}", e),
                    Ok(_) => {}
                };
                
                return Some(Config {
                    watch_path: Self::get_config("DEFAULT", "watch_path", &config),

                    metaflac_path: Self::get_config("FLAC", "metaflac_bin", &config),
                    metaflac_flags: Self::get_config("FLAC", "metaflac_flags", &config),
                    
                    mp3gain_path: Self::get_config("MP3", "mp3gain_bin", &config),
                    mp3gain_flags: Self::get_config("MP3", "mp3gain_flags", &config),

                    uptime_url: Self::get_config("UPTIME", "uptime_url", &config),

                    enable_replay_gain: Self::get_config_bool("ENABLE", "replay_gain", &config).unwrap(),
                    enable_remove_comment: Self::get_config_bool("ENABLE", "remove_comment", &config).unwrap(),
                });
            }
        }
    }

    fn get_config(section: &str, key: &str, config: &Ini) -> String{
        let value = config.get(section, key);
        
        if value.is_none() {
            eprintln!("Could not find key '{}' in section '{}'!", key, section);
            exit(1);
        }
        
        return value.unwrap();
    }

    fn get_config_bool(section: &str, key: &str, config: &Ini) -> Option<bool> {
        let value = config.getbool(section, key);

        if value.is_err() {
            eprintln!("Could not find key '{}' in section '{}'!", key, section);
            exit(1);
        }

        return value.unwrap();
    }
}
