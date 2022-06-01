use std::{thread, time};

mod nginx_log_read;

fn main() {
    // 10 秒
    let ten_seconds = time::Duration::from_secs(10);
    loop {
        println!("线程沉睡 10 秒！");
        thread::sleep(ten_seconds);
        println!("沉睡完成！");
        nginx_log_read::read_log(&String::from(
            "/home/archliu/workspace/arch/rust_proj/horus_log_col/access.log",
        ));
    }
}
