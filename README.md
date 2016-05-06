# cargo release

[![](http://meritbadge.herokuapp.com/cargo-release)](https://crates.io/crates/cargo-release)

This a script standardize release process of cargo project for you.

Basically it runs following tasks:

* Check if current working directory is git clean
* Read version from Cargo.toml, remove prerelease extension, bump
  version and commit if necessary
* Run `cargo publish`
* Generate rustdoc and push to gh-pages optionally
* Create a git tag for this version
* Bump version for next development cycle
* `git push`

## Install

`cargo install cargo-release`

## Usage

`cargo release`

### Prerequisite

* Your project should be managed by git.

### Release level

Use `-l [level]` or `--level [level]` to specify a release level.

* By default, cargo release removes prerelease extension; if there is
no prerelease extension, the current version will be used (0.1.0-pre
-> 0.1.0, 0.1.0 -> 0.1.0)
* If level is `patch` and current version is a prerelease, it behaves
like default; if current version has no extension, it bumps patch
version (0.1.0 -> 0.1.1)
* If level is `minor`, it bumps minor version (0.1.0-pre -> 0.2.0)
* If level is `major`, it bumps major version (0.1.0-pre -> 1.0.0)

### Signing your git commit and tag

Use `--sign` option to GPG sign your release commits and
tags. [Further
information](https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work)

### Upload rust doc to github pages

By using `--upload-doc` option, cargo-release will generate rustdoc
during release process, and commit the doc directory to `gh-pages`
branch. So you can access your rust doc at
https://YOUR-GITHUB-USERNAME.github.io/YOUR-REPOSITORY-NAME/YOUR-CRATE-NAME

Currently only github pages is supported.

#### WARNING

This option will override your existed `gh-pages` branch,
use it at your own risk.

### Tag prefix

For single-crate repository, we will use version number as git tag
name.

For multi-crate repository, the subdirectory name will be used as tag
name. For example, when releasing serde_macros 0.7.0 in serde-rs/serde
repo, a tag named as `serde_macros-0.7.0` will be created.

You can always override this behavior by using `--tag-prefix <prefix>`
option.

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
