Documentation
-------------
The project documentation under ``docs`` can be compiled using Cargo Sphinx.
Output is placed in ``docs/_build/html``.

.. code:: bash

    cargo sphinx

If this does not work, raise a bug then use the Makefile as fallback.

.. code:: bash

    cd docs
    make clean html

The Docker container provides an installation of Python and Sphinx as well as
the latest Cargo Sphinx published to `crates.io`_. These can be used to build
the documentation also. To make the documentation directly in container withou
an intermediate shell, use:

.. _crates.io: https://crates.io

.. code:: bash

    docker run -v "$(pwd):/cargo-sphinx" \
         --workdir=/cargo-sphinx \
         cargo-sphinx \
         cargo sphinx

The compiled document is written to the shared location and is available on the
host machine under ``docs/_build/html``.

It is published to `woofwoofinc.github.io/cargo-sphinx`_ using `GitHub Pages`_.

.. _woofwoofinc.github.io/cargo-sphinx: https://woofwoofinc.github.io/cargo-sphinx
.. _GitHub Pages: https://pages.github.com

Publishing from the Docker container fails for missing GitHub credentials. In
this case it is possible to run the publication command in the container
interactively and complete it on the host machine. Compile and generate the
Git repository to push in ``docs/_build/html`` by running the following in the
container.

.. code:: bash

    cargo sphinx --push

Then on the host, change to ``docs/_build/html` which is now a new Git
repository with the documentation HTML committed on ``master``. Push this to
``origin`` by specifying the remote.

.. code:: bash

    cd docs/_build/html
    git push -f git@github.com:woofwoofinc/cargo-sphinx.git master:gh-pages
