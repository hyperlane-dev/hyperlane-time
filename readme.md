<center>

## hyperlane-time

[![](https://img.shields.io/crates/v/hyperlane-time.svg)](https://crates.io/crates/hyperlane-time)
[![](https://img.shields.io/crates/d/hyperlane-time.svg)](https://img.shields.io/crates/d/hyperlane-time.svg)
[![](https://docs.rs/hyperlane-time/badge.svg)](https://docs.rs/hyperlane-time)
[![](https://github.com/ltpp-universe/hyperlane-time/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/hyperlane-time/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane-time.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane-time/)

[Api Docs](https://docs.rs/hyperlane-time/latest/hyperlane_time/)

> A library for fetching the current time based on the system's locale settings.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane-time
```

## Use

```rust
use hyperlane_time::*;
println!("Now time: {}", current_time());
println!("Now day: {}", current_date());
println!("Now date: {}", current_date_gmt());
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
