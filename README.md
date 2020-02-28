# tokio-compat fd leak issue

- `cargo run --bin tokio02` works
- `cargo run --bin tokio-compat` eats up fds and crashes

Tokio 0.1 is ok too, so the issue must be somewhere at `tokio-compat`.
