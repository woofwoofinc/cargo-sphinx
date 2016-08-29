Installing
----------
Cargo Sphinx is designed for use in Git managed Cargo-based Rust projects. To
install it, use:

.. code:: bash

    cargo install cargo-sphinx

The following dependencies are needed to run Cargo Sphinx.

* Sphinx_: This is the documentation generator tool targeted by Cargo Sphinx.
  Currently Sphinx is the only supported documentation tool.
* Python_: Sphinx is written in Python and requires the interpreter to run.
* Git_: Cargo Sphinx expects to use version control to store the generated
  documentation HTML files. Currently Git is the only supported version
  control. This is not required if the publication features of Cargo Sphinx
  are not being used.
* Cargo_: Cargo is the Rust_ build tool. This project is integrated as a
  subcommand to Cargo.
* Make: An installation of Make is needed as part of the Sphinx build. Sphinx
  can optionally generate a Makefile as part of project creation. Cargo Sphinx
  requires this Makefile to function because user Sphinx directory layouts may
  vary significantly. Relying on the Makefile means that Sphinx layout
  parameters do not need to be specified to Cargo Sphinx.

.. _Sphinx: http://www.sphinx-doc.org
.. _Python: https://www.python.org
.. _Git: https://git-scm.com
.. _Cargo: http://doc.crates.io
.. _Rust: http://rust-lang.org

Optionally, you may like to also install packages for extra Sphinx templates
such as:

* `Read The Docs <https://pypi.python.org/pypi/sphinx_rtd_theme>`_ 
* `Sphinx Bootstrap <https://ryan-roemer.github.io/sphinx-bootstrap-theme>`_
