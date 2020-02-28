use tokio02::runtime::Builder;

fn main() {
    for _ in 0..10240 {
        println!("{}", tokio_fd_leak::get_fds());
        let _ = Builder::new().enable_all().build().unwrap();
    }
}
