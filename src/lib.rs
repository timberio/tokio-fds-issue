pub fn get_fds() -> usize {
    std::fs::read_dir("/proc/self/fd").unwrap().count()
}
