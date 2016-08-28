Cargo Sphinx
============

.. toctree::
   :maxdepth: 2


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


Running Clippy Lints
--------------------
Clippy_ is a Rust linter. Currently it has to be run manually since Cargo
Sphinx targets Rust stable and Clippy requires Rust nightly. Switching versions
is easy with ``rustup`` - use the following to lint the repository:

.. code:: bash

    rustup run nightly cargo clippy

.. _Clippy: https://github.com/Manishearth/rust-clippy


Building the Documentation
--------------------------
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


Travis
------
Cargo Sphinx is continuously integrated on `Travis CI`_.

.. _Travis CI: https://travis-ci.org

To update encrypted credentials in the ``.travis.yml`` file, use the
`Travis command line tool`_.

.. _Travis command line tool: https://docs.travis-ci.com/user/encryption-keys

.. code:: bash

    gem install travis

For instance, to update the Slack notification credential:

.. code:: bash

    travis encrypt "woofwoofinc:<credential>" --add notifications.slack


Publishing on ``crates.io``
---------------------------
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


License
-------
This work is dual-licensed under the Apache License, Version 2.0 and under the
MIT Licence.

You may licence this work under the Apache License, Version 2.0.

::

    Copyright 2016 Ning Sun and tojson_macros contributors

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

Alternatively, you may licence this work under the MIT Licence at your option.

::

    Copyright (c) 2016 Ning Sun and tojson_macros contributors
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

The licence explainers at `Choose a License`_ may be helpful. They have 
descriptions for both the `Apache 2.0 Licence`_ and `MIT Licence`_ conditions.

.. _Choose a License: http://choosealicense.com
.. _Apache 2.0 Licence: http://choosealicense.com/licenses/apache-2.0/
.. _MIT Licence: http://choosealicense.com/licenses/mit/


Contributing
------------
Please note that this project is released with a `Contributor Code of Conduct`_.
By participating in this project you agree to abide by its terms. Instances of 
abusive, harassing, or otherwise unacceptable behavior may be reported by
contacting the project team at ``woofwoofinc@gmail.com``.

.. _Contributor Code of Conduct: http://contributor-covenant.org/version/1/4/

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
