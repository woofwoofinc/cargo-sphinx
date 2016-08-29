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
