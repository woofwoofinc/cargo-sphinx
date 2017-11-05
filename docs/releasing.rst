Releasing
=========

Publishing to Crates.io
-----------------------
Instructions for uploading to the crate repository at `crates.io`_ are
at `doc.crates.io/crates-io.html`_. First login to the site using:

.. _crates.io: http://crates.io
.. _doc.crates.io/crates-io.html: http://doc.crates.io/crates-io.html#publishing-crates

::

    cargo login <token>

Tokens can be found from `crates.io/me`_. To make a release, first clean and
build the package:

.. _crates.io/me: https://crates.io/me

::

    git stash
    cargo clean
    cargo package

Examine the built package under ``target/package/cargo-sphinx-<version>``.
And when happy to publish:

::

    cargo publish

And check out the new update at `crates.io/crates/cargo-sphinx`_.

.. _crates.io/crates/cargo-sphinx: https://crates.io/crates/cargo-sphinx


Publishing the Documentation
----------------------------
Project documentation is published to `woofwoofinc.github.io/cargo-sphinx`_
using `GitHub Pages`_.

.. _woofwoofinc.github.io/cargo-sphinx: https://woofwoofinc.github.io/cargo-sphinx
.. _GitHub Pages: https://pages.github.com

Build and publish the documentation as described in :ref:`documentation`. The
GitHub configuration for this project is to serve documentation from the
``gh-pages`` branch.

::

    cargo sphinx --push

Publishing from the container fails for missing GitHub credentials. In this case
it is possible to run the publication command in the container interactively and
complete it on the host machine. Compile and generate the Git repository to push
in ``docs/_build/html`` by running the following in the container.

::

    cargo sphinx --push

Then on the host, change to ``docs/_build/html`` which is now a new Git
repository with the documentation HTML committed on master. Push this to origin
by specifying the remote.

::

    cd docs/_build/html
    git push -f git@github.com:woofwoofinc/cargo-sphinx.git master:gh-pages
