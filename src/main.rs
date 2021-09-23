use crate::config::Config;
use std::env;

mod config;
mod ssh_operation;
mod ssh_file;


/// 默认配置文件, 在项目conf文件夹下
const DEF_PATH: &str = "../conf/config.toml";

fn main() {
    let mut path = match env::args().nth(1) {
        None => DEF_PATH.to_owned(),
        Some(t) => t
    };
    let config = Config::init(path);
    let s = ssh_operation::connection(&config);
    match ssh_file::file_not_null(&s, &config) {
        Some(()) => {
            if config.flag.is_backup { ssh_file::file_backup(&s, &config) }
            else { ssh_file::file_remove(&s, &config) }
        }
        _ => {}
    };
    ssh_file::file_upload(&s, &config);
    ssh_operation::kill(&s, &config);
    ssh_operation::run(&s, &config);
}