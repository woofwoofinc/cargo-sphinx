Docker
======
A Docker_ container definition is provided with installations of the tools used
to develop Cargo Sphinx. To use the container, first install Docker if not
already available. Then create the container by running the following build at
the top level of the repository source tree:

::

    docker build -t cargo-sphinx .

.. _Docker: http://docker.io

Docker uses a build cache so it may be necessary to add ``--no-cache=true``
sometimes in order to achieve a complete rebuild.

Once built, an interactive shell can be run in the container using:

::

    docker run -it \
         -v "$(pwd):/cargo-sphinx" \
         --workdir=/cargo-sphinx \
         cargo-sphinx \
         /bin/bash

The current working directory from the host machine, i.e. the Cargo Sphinx
repository, is available as the current directory in the container. So it is
possible to build and test the library as described earlier.

::

    cargo test

To tear down the container and start over, remove the Docker image:

::

    docker rmi -f cargo-sphinx

Docker images can be listed using:

::

    docker images 

