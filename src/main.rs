mod nginx_log_read;

fn main() {
    nginx_log_read::read_log(&String::from("/var/log/nginx/access.log"));
}
