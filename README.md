# fly-accept-encoding
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Determine the best encoding possible from an Accept-Encoding HTTP header.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

**Note:** this is a fork of https://github.com/http-rs/accept-encoding with a few more features.

## Examples
__Basic usage__
```rust
use fly_accept_encoding::Encoding;
use anyhow::Error;
use http::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING};

fn main () -> Result<(), Error> {
  let mut headers = HeaderMap::new();
  headers.insert(ACCEPT_ENCODING, HeaderValue::from_str("gzip, deflate, br")?);

  let encoding = fly_accept_encoding::parse(&headers)?;
  assert_eq!(encoding, Some(Encoding::Gzip));
  Ok(())
}
```

## Installation
```sh
$ cargo add fly-accept-encoding
```

## Safety
This crate uses `#![deny(unsafe_code)]` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
None.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/fly-accept-encoding.svg?style=flat-square
[2]: https://crates.io/crates/fly-accept-encoding
[3]: https://img.shields.io/travis/superfly/accept-encoding/master.svg?style=flat-square
[4]: https://travis-ci.org/superfly/accept-encoding
[5]: https://img.shields.io/crates/d/fly-accept-encoding.svg?style=flat-square
[6]: https://crates.io/crates/fly-accept-encoding
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/fly-accept-encoding

[releases]: https://github.com/superfly/accept-encoding/releases
[contributing]: https://github.com/superfly/accept-encoding/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/superfly/accept-encoding/labels/good%20first%20issue
[help-wanted]: https://github.com/superfly/accept-encoding/labels/help%20wanted
