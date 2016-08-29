Building this Documentation
---------------------------
The RestructuredText format Sphinx_ documentation under ``docs`` can be
compiled using the ``Makefile`` present.

.. code:: bash

    cd docs
    make clean html

.. _Sphinx: http://sphinx-doc.org

See this `RestructuredText Primer`_ for guidance on writing RestructuredText.

.. _RestructuredText Primer: http://sphinx-doc.org/rest.html

The Docker container provides an installation of Python and Sphinx required to
do this build. To make the documentation directly in container without an
intermediate shell, use:

.. code:: bash

    docker run -v "$(pwd):/cargo-sphinx" \
         --workdir=/cargo-sphinx/docs \
         cargo-sphinx \
         make clean html

The compiled document is written to the shared location and is available on the
host machine under ``docs/_build``. It is published to
`woofwoofinc.github.io/cargo-sphinx`_ using `GitHub Pages`_.

.. _woofwoofinc.github.io/cargo-sphinx: https://woofwoofinc.github.io/cargo-sphinx
.. _GitHub Pages: https://pages.github.com
