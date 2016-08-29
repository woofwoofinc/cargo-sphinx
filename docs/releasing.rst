Releasing
---------
Instructions for uploading to the crate repository at ``crates.io`` are
at `doc.crates.io/crates-io.html`_. First login to the site using:

.. _doc.crates.io/crates-io.html: http://doc.crates.io/crates-io.html#publishing-crates

.. code:: bash

    cargo login <token>

Token can be found from `crates.io/me`_. To make a release, first clean and
build the package:

.. _crates.io/me: https://crates.io/me

.. code:: bash

    git stash
    cargo clean
    cargo package

Examine the built package under ``target/package/cargo-sphinx-<version>``.
And when happy to publish:

.. code:: bash

    cargo publish

And check out the new update at `crates.io/crates/cargo-sphinx`_.

.. _crates.io/crates/cargo-sphinx: https://crates.io/crates/cargo-sphinx
