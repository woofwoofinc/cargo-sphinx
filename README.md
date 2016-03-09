# cargo release

[![](http://meritbadge.herokuapp.com/cargo-release)](https://crates.io/crates/cargo-release)

This a script standardize release of cargo project for you.

Basically it runs following tasks:

* Check if current working directory is git clean
* Read version from Cargo.toml, remove prerelease extension and commit if necessary
* Run `cargo publish`
* Create a git tag for this version
* Bump version for next development cycle
* `git push`

## Install

`cargo install cargo-release`

## Usage

`cargo release`

### prerequisite

* Your project should be managed by git.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
