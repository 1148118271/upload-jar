use ssh2::Session;
use crate::config::Config;
use std::io::{Write, Read};
use std::path::Path;
use std::fs::File;



/// 文件备份
pub fn file_backup(s: &Session, config: &Config) {
    println!("开始备份文件{}{}......", &config.path.remote_path, &config.path.file_name);
    let date_time = chrono::Local::now().to_string();
    let dv = date_time.split(" ").collect::<Vec<&str>>();
    let mut c = s.channel_session()
        .unwrap();
    c.exec(
        &format!
        ("mv {}{} {}{}_{}.bak",
         &config.path.remote_path, &config.path.file_name,
         &config.path.remote_path,  &config.path.file_name, dv[0])
    ).unwrap();
    c.close().unwrap();
    println!("文件{}{}备份完成......", &config.path.remote_path, &config.path.file_name);
}




/// 文件上传
pub fn file_upload(s: &Session, config: &Config) {
    println!("开始上传文件,本地路径为{}{}, 远程路径为{}{}......",
             &config.path.local_path, &config.path.file_name,
             &config.path.remote_path, &config.path.file_name
    );
    let mut actual = Vec::new();
    File::open(config.path.local_path.to_string() + &config.path.file_name)
        .unwrap()
        .read_to_end(&mut actual)
        .unwrap();
    let mut ch = s
        .scp_send(
            &Path::new(&(config.path.remote_path.to_string() + &config.path.file_name)),
            0o644,
            actual.len() as u64,
            None
        ).unwrap();
    println!("文件上传中, 请稍等......");
    ch.write_all(actual.as_slice()).unwrap();
    ch.close().unwrap();
    println!("文件上传完成......");
}




/// 检查文件是否存在
pub fn file_not_null(s: &Session, config: &Config) -> Option<()> {
    println!("检查文件{}{}是否存在......",  &config.path.remote_path, &config.path.file_name);
    let mut c = s.channel_session().unwrap();
    c.exec(&format!("ls -l {}{}", &config.path.remote_path, &config.path.file_name))
        .unwrap();
    let mut info = String::new();
    c.read_to_string(&mut info).unwrap();
    c.close().unwrap();
    if "".eq(&info) {
        println!("文件{}{}不存在......", &config.path.remote_path, &config.path.file_name);
        return None
    }
    println!("文件{}{}存在......", &config.path.remote_path, &config.path.file_name);
    Some(())
}





/// 移除文件
pub fn file_remove(s: &Session, config: &Config) {
    println!("开始删除文件{}{}......",  &config.path.remote_path, &config.path.file_name);
    let mut c = s.channel_session().unwrap();
    c.exec(&format!("rm -rf {}{}", &config.path.remote_path, &config.path.file_name))
        .unwrap();
    c.close().unwrap();
    println!("文件{}{}删除成功......", &config.path.remote_path, &config.path.file_name);
}
