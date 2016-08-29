.. _running:

Running
-------
Once installed and configured, you can build your project Sphinx documentation
using:

.. code:: bash

    cargo sphinx

The generated site will be output per the specific Sphinx configuration. Using
defaults for Sphinx and Cargo Sphinx will result in output at
``docs/_build/html``.


Publishing
^^^^^^^^^^ 
.. WARNING::
   This will override your existing ``gh-pages`` branch, use at your own risk.

To push the generated documentation to the GitHub Pages branch include the 
``--push`` option:

.. code:: bash

    cargo sphinx --push

This will generate Sphinx documentation and commit it locally to the
``gh-pages`` branch of the repository. Then the branch is force pushed to the
``origin`` remote. If the origin is GitHub and GitHub Pages are enabled on the
repository then you will be able to view your documentation at
https://YOUR-GITHUB-USERNAME.github.io/YOUR-REPOSITORY-NAME.

Including the ``--dry-run`` option will print all the commands instead of
performing them.

.. code:: bash

    cd docs
    make clean html
    cd -
    touch docs/_build/html/.nojekyll
    cd docs/_build/html
    git init
    cd -
    cd docs/_build/html
    git add .
    cd -
    cd docs/_build/html
    git commit  -am (cargo-sphinx) Generate docs.
    cd -
    cd docs
    git push -f git@github.com:woofwoofinc/cargo-sphinx.git master:gh-pages
    cd -


Commandline Options
^^^^^^^^^^^^^^^^^^^
The following commandline options are supported by Cargo Sphinx. These allow
defaults and ``Cargo.toml`` configurations to be overriden.

* ``-p``, ``--push``: Push generated documentation to the git remote.
* ``-s``, ``--sign``: Sign the git commit.
* ``--commit-message <STRING>``: Specify the commit message for the
  documentation change. Defaults to "(cargo-sphinx) Generate docs." if not
  specified.
* ``--docs-path <STRING>``: Specify the path of the Sphinx documentation to
  build. Defaults to "docs" if not specified or provided in ``Cargo.toml``.
* ``--push-branch <STRING>``: Specify the Git branch to push documentation on.
  Defaults to "gh-pages" if not specified or provided in ``Cargo.toml``.
* ``--push-remote <STRING>``: Specify the Git remote to push to. Defaults to
  "origin" if not specified or provided in ``Cargo.toml``.
* ``--dry-run``: Print commands to execute instead of running.  

The Cargo subcommand tooling also provides the following flags:

* ``--color``: Coloring: auto, always, never.
* ``-h``, ``--help``: Prints help information
* ``-q``, ``--quiet``: Less output printed to stdout.
* ``-V``, ``--version``: Prints version information
* ``-v``, ``--verbose``: Use verbose output.
