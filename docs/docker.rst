Docker
------
A Docker_ container definition is provided with installations of the tools used
to develop Cargo Sphinx. To use the container, first install Docker if not
already available and start a Docker terminal. Then create the container by
running the following build at the top level of the repository source tree:

.. code:: bash

    docker build -t cargo-sphinx .

.. _Docker: http://docker.io

Docker uses a build cache so it may be necessary to add ``--no-cache=true``
sometimes in order to achieve a complete rebuild.

Once built, an interactive shell can be run in the container using:

.. code:: bash

    docker run -it \
         -v "$(pwd):/cargo-sphinx" \
         --workdir=/cargo-sphinx \
         cargo-sphinx \
         /bin/bash

The current working directory from the host machine is available as the current
directory in the container so it is possible to build and test the library as
described earlier.

.. code:: bash

    cargo test
