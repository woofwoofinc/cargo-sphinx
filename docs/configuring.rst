Configuring
-----------

Sphinx Project Setup
^^^^^^^^^^^^^^^^^^^^
The Sphinx source files should be placed in a directory in the same repository
with the ``Cargo.toml`` and source code being documented.

The Sphinx project should also be configured to use a Makefile. The
``sphinx-quickstart`` setup tool includes dialogue for generating a Makefile
which corresponds to the selected Sphinx layout.

For an existing Sphinx project, you may wish to run the ``sphinx-quickstart``
setup tool in a temporary directory retrospectively to create the Makefile.
Alternatively, you can adapt the Makefile for this project which is at
https://github.com/woofwoofinc/cargo-sphinx/blob/master/docs/Makefile.

It is useful to configure the Sphinx project to include the Sphinx extension
for GitHub Pages. This will create ``.nojekyll`` files in the output. This is
required in GitHub Pages to bypass Jekyll processing, otherwise Sphinx files
and directories which start with underscores will be purged. (Jekyll convention
does not copy these to the final site.) 

Either select the ``githubpages`` option in the ``sphinx-quickstart`` dialogue
or include the following in the ``conf.py`` for your Sphinx project.

.. code:: python

    extensions = [
        'sphinx.ext.githubpages',
    ]

Cargo Sphinx has you covered and will create ``.nojekyll`` files in the
generated output if they are not present.


Cargo Sphinx Options
^^^^^^^^^^^^^^^^^^^^
You may choose any directory for the Sphinx source files. Cargo Sphinx uses
`docs` at the top level as the default but can be easily configured in
``Cargo.toml`` or by commandline invocation parameters to use another location.

Options for Cargo Sphinx can be set in ``Cargo.toml`` under the custom section
``package.metadata.sphinx``:

* ``docs-path``: string, location of the project Sphinx documentation files.
  Default "docs".
* ``commit-message``: string, a commit message template for doc import.
  Default "(cargo-sphinx) Generate docs.".
* ``sign-commit``: bool, use GPG to sign git commits. Default false.
* ``push-remote``: string, git remote for push. Default "origin".
* ``push-branch``: string, default branch to push docs. Default "gh-pages".

The following is an example of how this section appears in ``Cargo.toml``.

.. code:: ini

    [package.metadata.sphinx] 
    docs-path = "docs"
    commit-message = "(cargo-sphinx) Generate docs."
    sign-commit = false
    push-remote = "origin"
    push-branch = "gh-pages"

Overrides to defaults and ``Cargo.toml`` configuration can be specified when
running the Cargo Sphinx commandline tool.
