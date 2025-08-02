<center>

## hyperlane-time

[![](https://img.shields.io/crates/v/hyperlane-time.svg)](https://crates.io/crates/hyperlane-time)
[![](https://img.shields.io/crates/d/hyperlane-time.svg)](https://img.shields.io/crates/d/hyperlane-time.svg)
[![](https://docs.rs/hyperlane-time/badge.svg)](https://docs.rs/hyperlane-time)
[![](https://github.com/crates-io-utils/hyperlane-time/workflows/Rust/badge.svg)](https://github.com/crates-io-utils/hyperlane-time/actions?query=workflow:Rust)
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

println!("Current Time: {}", time());
println!("Current Date: {}", date());
println!("GMT Date: {}", gmt());
println!("Timestamp (s): {}", timestamp());
println!("Timestamp (ms): {}", timestamp_millis());
println!("Timestamp (μs): {}", timestamp_micros());
println!("Current Year: {}", year());
println!("Current Month: {}", month());
println!("Current Day: {}", day());
println!("Current Hour: {}", hour());
println!("Current Minute: {}", minute());
println!("Current Second: {}", second());
println!("Current Millis: {}", millis());
println!("Current Micros: {}", micros());
println!("Is Leap Year (1949): {}", is_leap_year(1949));
println!("Calculate Current Time: {:?}", calculate_time());
println!("Compute Date (10000 days): {:?}", compute_date(10000));
println!("Current Time with Millis: {}", time_millis());
println!("Current Time with Micros: {}", time_micros());
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
