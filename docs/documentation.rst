.. _documentation:

Documentation
=============
The project documentation under ``docs`` can be compiled using Cargo Sphinx.
Output is placed in ``docs/_build/html``.

::

    cargo sphinx

If this does not work, raise a bug then use the Makefile as fallback.

::

    cd docs
    make clean html

The development container provides an installation of Python and Sphinx which
can be used to build this documentation also. The latest published Cargo Sphinx
is also included.

Build the container as described in :ref:`dev`. Then change to the Cargo Sphinx
root directory and start the container with this directory mounted at
``/cargo-sphinx``.

::

    sudo rkt run \
        --interactive \
        --volume cargo-sphinx,kind=host,source=$(pwd) \
        dev-cargo-sphinx \
        --mount volume=cargo-sphinx,target=/cargo-sphinx

Inside the container, change directory to ``/cargo-sphinx`` and run the build
command.

::

    cargo sphinx

The compiled document is written to the shared location and is available on the
host machine under ``docs/_build/html``.

It is published to `woofwoofinc.github.io/cargo-sphinx`_ using `GitHub Pages`_.

.. _woofwoofinc.github.io/cargo-sphinx: https://woofwoofinc.github.io/cargo-sphinx
.. _GitHub Pages: https://pages.github.com

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
