# Learning Axum Rust Webdev

## Quick_dev commands
- Install cargo-watch
```sh
cargo install cargo-watch
```

- Recompile & run when a change is done une the source code.
```sh
cargo watch -q -c -w src/ -x run
```

- Execute the quick_dev function
```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
