use futures01::Future;
use tokio01::util::FutureExt;
use tokio_compat::runtime::{Builder, Runtime};

pub fn shutdown_on_idle(runtime: Runtime) {
    block_on(
        runtime
            .shutdown_on_idle()
            .timeout(std::time::Duration::from_secs(10)),
    )
    .unwrap()
}

pub fn block_on<F, R, E>(future: F) -> Result<R, E>
where
    F: Send + 'static + Future<Item = R, Error = E>,
    R: Send + 'static,
    E: Send + 'static,
{
    let mut rt = runtime();

    rt.block_on(future)
}

pub fn runtime() -> Runtime {
    Builder::new().core_threads(1).build().unwrap()
}

pub fn get_fds() -> usize {
    std::fs::read_dir("/proc/self/fd").unwrap().count()
}

fn main() {
    for _ in 0..10240 {
        println!("{}", get_fds());
        let rt = Runtime::new().unwrap();
        shutdown_on_idle(rt)
    }
}
