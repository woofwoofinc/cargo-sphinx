Travis
======
Cargo Sphinx is continuously integrated on `Travis CI`_.

.. _Travis CI: https://travis-ci.org

Apart from verifying the build and running the tests, Travis also runs
rustfmt_ on new patches and ensures they adhere to our styleguide.

.. _rustfmt: https://github.com/rust-lang-nursery/rustfmt

To update encrypted credentials in the ``.travis.yml`` file, use the
`Travis command line tool`_.

.. _Travis command line tool: https://docs.travis-ci.com/user/encryption-keys

::

    gem install travis

For instance, to update the Slack notification credential:

::

    travis encrypt "woofwoofinc:<credential>" --add notifications.slack
