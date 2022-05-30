use reqwest::blocking::Client;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

pub fn read_log(file_name: &String) {
    // 读取上一次文件读取的位置
    println!("file name is :{} ", file_name);
    let file = File::open(file_name).unwrap();
    let mut fin = BufReader::new(file);
    let mut read_cur = String::from("0");
    let read_res = fs::read_to_string("./read_index");
    if let Ok(file) = read_res {
        println!("{:?}", file);
        read_cur = file;
    }
    println!("读取到的游标： {}", read_cur);
    fin.seek(SeekFrom::Current(read_cur.parse::<i64>().unwrap()))
        .expect("文件指针移动失败!");
    let mut read_len = 1;
    let mut read_end_pos = 0;
    while read_len != 0 {
        let mut line = String::new();
        let start_pos = fin.stream_position();
        if let Result::Ok(pos) = start_pos {
            println!("start 指针： {}", pos);
        }
        read_len = fin.read_line(&mut line).expect("read line error");
        print!("line: {}", line);
        let fields: Vec<&str> = line.split(' ').collect();
        for field in &fields {
            println!("{}", field);
        }
        let end_pos = fin.stream_position();
        if let Result::Ok(pos) = end_pos {
            read_end_pos = pos;
            println!("end 指针： {}", pos);
        }
    }
    // 发送 horus 请求
    let client = Client::new();
    let resp = client.post("https://httpbin.org/ip").send();
    match resp {
        Ok(res) => parse_resp(res),
        Err(err) => println!("请求异常！ {:?}", err),
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
