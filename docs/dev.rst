.. _dev:

Development Tools Container
===========================
The project source comes with a ``dev`` directory which contains a script for
building a rkt Ubuntu container with useful development tools for development.

To build this you must have a system with an installation of rkt and acbuild.
For macOS, the RktMachine_ project provides an xhyve-based VM running CoreOS
with installations of rkt, acbuild, docker2aci, and other useful container
tools.

.. _RktMachine: https://github.com/woofwoofinc/rktmachine


Building
--------
Build the container using the provided build script:

::

    ./dev-cargo-sphinx.acbuild.sh

This will make a ``dev-cargo-sphinx.oci`` in the directory. Convert this to
``dev-cargo-sphinx.aci`` for installation into rkt:

::

    gunzip < dev-cargo-sphinx.oci > dev-cargo-sphinx.oci.tar
    docker2aci dev-cargo-sphinx.oci.tar
    rm dev-cargo-sphinx.oci.tar
    mv dev-cargo-sphinx-latest.aci dev-cargo-sphinx.aci

Install this into rkt:

::

    rkt --insecure-options=image fetch ./dev-cargo-sphinx.aci

This container is intended for interactive use, so to run it with rkt use:

::

    sudo rkt run \
        --interactive \
        --volume cargo-sphinx,kind=host,source=$(pwd) \
        dev-cargo-sphinx \
        --mount volume=cargo-sphinx,target=/cargo-sphinx

The current working directory is available on the container at
``/cargo-sphinx``.
