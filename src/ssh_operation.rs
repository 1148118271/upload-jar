use std::net::TcpStream;
use ssh2::{Session, Channel};
use crate::config::Config;
use std::io::{Read, Write};
use std::{thread, time};




/// 创建连接
pub fn connection(config: &Config) -> Session {
    println!("开始创建连接......");
    let tcp = TcpStream::connect(format!("{}:{}", &config.connect.ip, &config.connect.port))
        .expect("建立连接异常!");
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&config.connect.username, &config.connect.password)
        .expect("用户或密码不正确,请检查配置文件!");
    println!("连接创建成功......");
    sess
}



/// params:  Session  Config
/// 启动项目
pub fn run(s: &Session, config: &Config) {
    println!("项目启动中......");
    let mut c = s.channel_session().unwrap();
    let f: String;
    if config.flag.is_join {
        f = format!("{}{} {}", &config.path.remote_path, &config.path.file_name, &config.flag.environment)
    } else {
        f = format!("{}{}", &config.path.remote_path, &config.path.file_name)
    }
    c.request_pty("xterm", None, None).unwrap();
    c.shell().unwrap();
    let cmd = format!("nohup java -jar {} > {}{} 2>&1 &\n", &f, &config.path.remote_path, &config.path.out_file_name);
    c.write_all(&cmd.as_bytes())
        .unwrap();
    println!("执行命令 >> {}", &cmd);
    c.flush().unwrap();
    c.send_eof().unwrap();
    let pid = self::close(&mut c, &s, &config);
    println!("通道已关闭, 项目已启动, 进程id为: {}, 请到服务器查看详细日志......", pid);
}





/// 关闭shell通道
/// 关闭条件为进程已启动, 如果进程id未查到 睡眠半秒后继续查询
fn close(c: &mut Channel,
         s: &Session,
         config: &Config) -> String {
    loop {
        match self::get_pid(&s, &config) {
            None => {
                // 启动之后未查到进程
                // 睡眠半秒 继续查询
                thread::sleep(time::Duration::from_millis(500));
                continue;
            }
            Some(pid) => {
                println!("已查到进程 pid为: {}, 开始关闭通道.....", pid);
                // 进程启动成功 关闭shell通道
                c.close().unwrap();
                return pid;
            }
        }
    }
}






/// 获取进程pid
fn get_pid(sess: &Session, config: &Config) -> Option<String> {
    let mut channel = sess.channel_session()
        .expect("打开通道异常!");
    channel.exec(&format!("ps -ef |grep {}", &config.path.file_name))
        .expect("查找进程id异常!");
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    channel.close().unwrap();
    let vec = s.trim().split("\n").collect::<Vec<&str>>();
    let mut jar_str = "";
    if vec.len() > 1 {
        for x in vec {
            if x.contains("java -jar") {
                jar_str = x;
            }
        }
    }
    if !"".eq(jar_str) {
        let v1 = jar_str.split(" ").collect::<Vec<&str>>();
        let mut v2 = Vec::new();
        for x in v1 {
            if x.as_bytes().len() > 0 { v2.push(x) }
        }
        return Some(String::from(v2[1]))
    }
    None
}






/// 杀掉已有的进程 会调用 get_pid() 函数
pub fn kill(s: &Session, config: &Config) {
    let mut c = s.channel_session().unwrap();
    let pid = self::get_pid(&s, &config);
    match pid {
        Some(p) => {
            c.exec(&format!("kill -9 {}", p))
                .unwrap();
            println!("杀掉进程 >>> {}", &p);
        },
        _ => {}
    };
    c.close()
        .unwrap();
}