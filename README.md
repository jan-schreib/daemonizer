# daemonizer [![Latest Version](https://img.shields.io/crates/v/daemonizer.svg)](https://crates.io/crates/daemonizer/) [![Documentation](https://docs.rs/daemonizer/badge.svg)](https://docs.rs/daemonizer/)

```rust
use daemonizer::*;

fn main() {
    match daemonize("_daemon", "_daemon") {
        Ok(v) => (),
        Err(e) => (),
    }
}
```
