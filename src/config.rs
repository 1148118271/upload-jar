use serde::{ Serialize, Deserialize };
use std::fs::File;
use std::io::Read;


#[derive(Debug, Serialize, Deserialize)]
pub struct Connect {
    pub ip: String,
    pub port: u16,
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
    pub file_name: String,
    pub out_file_name: String,
    pub local_path: String,
    pub remote_path: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flag {
    pub is_backup: bool,
    pub is_join: bool,
    pub environment: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub connect: Connect,
    pub path: Path,
    pub flag: Flag
}

impl Config {
    pub fn init(path: String) -> Config {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("读取配置文件异常{:?}", e)
        };
        let mut text = String::new();
        match file.read_to_string(&mut text) {
            Err(e) => panic!("读取配置文件异常{:?}", e),
            _ => {}
        }
        let config = match toml::from_str::<Config>(&text) {
            Ok(t) => t,
            Err(e) => panic!("读取配置文件异常{:?}", e)
        };
        config
    }
}
