use configparser::ini::Ini;
use std::fs::{File};
use std::io::Read;
use std::process::exit;

pub struct Config {
    pub watch_path: String,

    pub metaflac_path: String,
    pub metaflac_flags: String,

    pub mp3gain_path: String,
    pub mp3gain_flags: String,
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
                
                let watch_path = config.get("DEFAULT", "watch_path");
                
                if watch_path.is_none(){
                    eprintln!("Could not find key 'watch_path' in section 'DEFAULT'!");
                    return None;
                }
                
                let metaflac_path = config.get("FLAC", "metaflac_bin");
                
                if metaflac_path.is_none(){
                    eprintln!("Could not find key 'metaflac_bin' in section 'FLAC'!");
                    return None;
                }
                
                let metaflac_flags = config.get("FLAC", "metaflac_flags");
                
                if metaflac_flags.is_none(){
                    eprintln!("Could not find key 'metaflac_flags' in section 'FLAC'!");
                    return None;
                }

                return Some(Config {
                    watch_path: Self::get_config("DEFAULT", "watch_path", &config),

                    metaflac_path: Self::get_config("FLAC", "metaflac_bin", &config),
                    metaflac_flags: Self::get_config("FLAC", "metaflac_flags", &config),
                    
                    mp3gain_path: Self::get_config("MP3", "mp3gain_bin", &config),
                    mp3gain_flags: Self::get_config("MP3", "mp3gain_flags", &config),
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
    
    
}