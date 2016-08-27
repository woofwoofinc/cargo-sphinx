[![Build Status](https://travis-ci.org/woofwoofinc/cargo-gh-pages.svg?branch=master)](https://travis-ci.org/woofwoofinc/cargo-gh-pages)
[![Dependency Status](https://dependencyci.com/github/woofwoofinc/cargo-gh-pages/badge)](https://dependencyci.com/github/woofwoofinc/cargo-gh-pages)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://github.com/woofwoofinc/cargo-gh-pages#license)


Cargo GitHub Pages
==================
Forked from the [cargo-release] project by [Ning Sun]. This is the GitHub Pages
functionality without the full release management.

[cargo-release]: https://github.com/sunng87/cargo-release
[Ning Sun]: https://github.com/sunng87


Using Cargo GitHub Pages
------------------------
Install Cargo GitHub Pages in a Git managed Cargo-based Rust project using:

    cargo install cargo-gh-pages

Generate RustDoc and push to GitHub Pages branch using:

    cargo gh-pages --upload-doc

This will generate RustDoc and commit the doc directory to the `gh-pages`
branch of the repository. If GitHub Pages are enabled on the repository then you
will be able to access your RustDoc at
https://YOUR-GITHUB-USERNAME.github.io/YOUR-REPOSITORY-NAME.

WARNING: This will override your existed gh-pages branch, use at your own risk.

Options for Cargo GitHub Pages can be set in `Cargo.toml` under the custom
section `package.metadata.gh-pages`:

* `sign-commit`: bool, use GPG to sign git commits. Default false.
* `push-remote`: string, git remote for push. Default "origin".
* `doc-branch`: string, default branch to push docs. Default "gh-pages".
* `doc-commit-message`: string, a commit message template for doc import.
  Default "(cargo-gh-pages) Generate docs.".

```toml
[package.metadata.gh-pages] 
sign-commit = false
push-remote = "origin"
doc-branch = "gh-pages"
doc-commit-message = "(cargo-gh-pages) Generate docs."
```

Include the `--dry-run` option to print all the commands to execute instead of
performing the generate and upload.

```
$ cargo gh-pages --dry-run
Building and exporting docs.
cargo doc --no-deps
cd target/doc/
git init
cd -
cd target/doc/
git add .
cd -
cd target/doc/
git commit  -am (cargo-gh-pages) Generate docs.
cd -
cd target/doc/
git push -f git@github.com:woofwoofinc/cargo-gh-pages.git master:gh-pages
cd -
```


Developing Cargo GitHub Pages
-----------------------------
Install the [Rust] development tools on your system with [rustup] if they are
not already available. Then build and test the project using:

    cargo test

[Rust]: https://www.rust-lang.org
[rustup]: https://www.rustup.rs

Install a development version of the plugin locally from latest source using:

    cargo install

Then test with a dry run:

    cargo gh-pages --dry-run



See further development documentation in the Sphinx documentation for this
project published at [woofwoofinc.github.io/cargo-gh-pages].

[woofwoofinc.github.io/cargo-gh-pages]: https://woofwoofinc.github.io/cargo-gh-pages


License
-------
This work is dual-licensed under the Apache License, Version 2.0 and under the
MIT Licence.

You may licence this work under the Apache License, Version 2.0.

    Copyright 2016 Ning Sun and tojson_macros contributors

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

Alternatively, you may licence this work under the MIT Licence at your option.

    Copyright (c) 2016 Ning Sun and tojson_macros contributors
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

The licence explainers at [Choose a License] may be helpful. They have 
descriptions for both the [Apache 2.0 Licence] and [MIT Licence] conditions.

[Choose a License]: http://choosealicense.com
[Apache 2.0 Licence]: http://choosealicense.com/licenses/apache-2.0/
[MIT Licence]: http://choosealicense.com/licenses/mit/


Contributing
------------
Please note that this project is released with a [Contributor Code of Conduct].
By participating in this project you agree to abide by its terms. Instances of 
abusive, harassing, or otherwise unacceptable behavior may be reported by
contacting the project team at woofwoofinc@gmail.com.

[Contributor Code of Conduct]: CODE_OF_CONDUCT.md

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
