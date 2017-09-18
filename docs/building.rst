Building
========
Install the Rust_ development tools on your system with rustup_ if they are
not already available.

.. _Rust: https://www.rust-lang.org
.. _rustup: https://www.rustup.rs

The following dependencies are also needed to build Cargo Sphinx.

* CMake_: Needed by Cargo tool library dependency.
* Libssl_: Needed by Cargo tool library dependency.

.. _CMake: https://cmake.org
.. _Libssl: https://wiki.openssl.org/index.php/Libssl_API

To build and test the project, use:

::

    cargo test

A development version of the Cargo Sphinx extension can be installed locally
using:

::

    cargo install

(``--force`` is necessary if Cargo Sphinx has previously been installed.)

Run an installed development version to test the Sphinx generation:

::

    cargo sphinx

Use ``--dry-run`` to see the ``--push`` steps.
    
::

    cargo sphinx --push --dry-run
