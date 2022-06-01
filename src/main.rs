use nginx_log_read::Setting;
use std::fs;
use std::{thread, time};

mod nginx_log_read;

fn main() {
    // 读取配置 /etc/horus/nginx_log_monitor.conf
    let horus_setting = fs::read_to_string("/etc/horus/nginx_log_monitor.conf");
    let mut horus_config = Setting {
        nginx_log_path: String::from("/var/log/nginx/access.log"),
        app_name: String::from("nginx"),
        access_party: String::from("nginx"),
        horus_host: String::from("127.0.0.1:80"),
    };
    if let Ok(setting) = horus_setting {
        // 读取用户的配置
        let user_config: Setting = serde_json::from_str(&setting).unwrap();
        horus_config.nginx_log_path = user_config.nginx_log_path;
        horus_config.app_name = user_config.app_name;
        horus_config.access_party = user_config.access_party;
        horus_config.horus_host = user_config.horus_host;
    }
    // 10 秒
    let ten_seconds = time::Duration::from_secs(10);
    loop {
        println!("线程沉睡 10 秒！");
        thread::sleep(ten_seconds);
        println!("沉睡完成！");
        nginx_log_read::read_log(&horus_config);
    }
}
