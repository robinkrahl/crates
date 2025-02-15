# TAI64 / TAI64N Timestamps for Rust

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust 1.35+][rustc-image]
[![forbid(unsafe_code)][unsafe-image]][unsafe-link]
[![Build Status][build-image]][build-link]
[![Gitter Chat][gitter-image]][gitter-link]

An implementation of the [TAI64(N)] (*Temps Atomique International*) timestamp
format in Rust.

Supports converting to/from Rust's built-in [SystemTime] type and optionally to
[chrono]'s [DateTime] type when the `"chrono"` feature is enabled.

[Documentation][docs-link]

## Requirements

- Rust 1.35+

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/tai64.svg
[crate-link]: https://crates.io/crates/tai64
[docs-image]: https://docs.rs/tai64/badge.svg
[docs-link]: https://docs.rs/tai64/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/develop/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.35+-blue.svg
[unsafe-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[unsafe-link]: https://internals.rust-lang.org/t/disabling-unsafe-by-default/7988
[build-image]: https://travis-ci.com/iqlusioninc/crates.svg?branch=develop
[build-link]: https://travis-ci.com/iqlusioninc/crates/
[gitter-image]: https://badges.gitter.im/iqlusioninc/community.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[TAI64(N)]: https://cr.yp.to/libtai/tai64.html
[SystemTime]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[chrono]: https://github.com/chronotope/chrono
[DateTime]: https://docs.rs/chrono/0.4.0/chrono/struct.DateTime.html
[LICENSE]: https://github.com/iqlusioninc/crates/blob/develop/LICENSE
