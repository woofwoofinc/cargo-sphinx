Clippy
======
Clippy_ is a Rust linter. Currently it has to be run manually since Cargo
Sphinx targets Rust stable and Clippy requires Rust nightly. Switching versions
is easy with rustup - use the following to lint the repository:

::

    rustup run nightly cargo clippy

.. _Clippy: https://github.com/Manishearth/rust-clippy
