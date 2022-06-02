use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::net::UdpSocket;

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub nginx_log_path: String, // nginx 日志文件路径,默认为 /var/log/nginx/access.log
    pub app_name: String,
    pub access_party: String,
    pub horus_host: String,
}

#[derive(Debug, Serialize)]
struct NginxLogRequest {
    #[serde(rename = "metricsCode")]
    metrics_code: String,
    #[serde(rename = "metricsColumns")]
    metrics_columns: Vec<String>,
    #[serde(rename = "metricsColumnsValue")]
    metrics_columns_value: Vec<Vec<String>>,
}

pub fn read_log(setting: &Setting) {
    // 读取上一次文件读取的位置
    println!("file name is :{} ", setting.nginx_log_path);
    let file = File::open(&setting.nginx_log_path).unwrap();
    let mut fin = BufReader::new(file);
    let mut read_cur = String::from("0");
    let read_res = fs::read_to_string("./read_index");
    if let Ok(file) = read_res {
        println!("{:?}", file);
        read_cur = file;
    }
    // println!("读取到的游标： {}", read_cur);
    fin.seek(SeekFrom::Current(read_cur.parse::<i64>().unwrap()))
        .expect("文件指针移动失败!");
    let mut read_len = 1;
    let mut read_end_pos = read_cur.parse::<u64>().unwrap();
    let mut nginx_logs: Vec<Vec<String>> = Vec::new();
    while read_len != 0 {
        let mut line = String::new();
        // let start_pos = fin.stream_position();
        // if let Result::Ok(pos) = start_pos {
        //     println!("start 指针： {}", pos);
        // }
        read_len = fin.read_line(&mut line).expect("read line error");
        // $msec]-[$remote_addr]-[$http_x_forwarded_for]-[$request_uri]-[$status]-[$http_user_agent]-[$request_time
        // print!("line: {}", line);
        let fields: Vec<&str> = line.split("]-[").collect();
        if fields.len() < 6 {
            continue;
        }
        let nginx_log: Vec<String> = vec![
            String::from(fields[0]).trim().to_string(),
            String::from(fields[1]).trim().to_string(),
            String::from(fields[2]).trim().to_string(),
            String::from(fields[3]).trim().to_string(),
            String::from(fields[4]).trim().to_string(),
            String::from(fields[5]).trim().to_string(),
            String::from(fields[6]).trim().to_string(),
        ];
        nginx_logs.push(nginx_log);
        let end_pos = fin.stream_position();
        if let Result::Ok(pos) = end_pos {
            read_end_pos = pos;
            // println!("end 指针： {}", pos);
        }
    }
    if nginx_logs.len() > 0 {
        let nginx_log_request = NginxLogRequest {
            metrics_code: String::from("nginxLogMonitor"),
            metrics_columns: vec![
                String::from("marking_time"),
                String::from("remote_addr"),
                String::from("http_x_forwarded_for"),
                String::from("request_uri"),
                String::from("status"),
                String::from("http_user_agent"),
                String::from("request_time"),
            ],
            metrics_columns_value: nginx_logs,
        };
        let local_ip = get_local_ip().unwrap().replace(".", "_");
        let url = String::from("http://").to_string()
            + &setting.horus_host
            + "/api/horus/collection/metricsCol?appName="
            + &setting.app_name
            + "&accessParty="
            + &setting.access_party
            + "&instanceId="
            + &local_ip;
        // 发送 horus 请求
        let client = Client::new();
        let resp = client.put(url).json(&vec![nginx_log_request]).send();
        match resp {
            Ok(res) => parse_resp(res),
            Err(err) => println!("请求异常！ {:?}", err),
        }
    } else {
        println!("无新增日志！");
    }
    // 保存文件读取的位置
    let mut cur_file = std::fs::File::create("./read_index").expect("create failed");
    cur_file
        .write_all(read_end_pos.to_string().as_bytes())
        .expect("write failed");
    println!("本次文件读取最后索引： {}", read_end_pos);
}

fn parse_resp(res: reqwest::blocking::Response) {
    if res.status().is_success() {
        println!("resp 返回成功 {:?}", res);
    } else {
        println!("resp 返回失败 {:?}", res);
    }
}

/**
 *  获取本机 IP
 */
pub fn get_local_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return Some(String::from("unknown")),
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return Some(String::from("unknown")),
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return Some(String::from("unknown")),
    };
}
