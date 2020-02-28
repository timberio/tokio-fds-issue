use tokio_compat::runtime::Runtime;

fn main() {
    for _ in 0..10240 {
        println!("{}", tokio_fd_leak::get_fds());
        let _ = Runtime::new().unwrap();
    }
}
