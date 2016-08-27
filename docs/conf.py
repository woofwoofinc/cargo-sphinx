project = u'Cargo GitHub Pages'
copyright = u'2016, Woof Woof, Inc.'
author = u'Woof Woof, Inc.'

version = '0.1.0'
release = '0.1.0'

templates_path = ['_templates']
exclude_patterns = ['_build']

source_suffix = '.rst'
master_doc = 'index'

language = None
pygments_style = 'sphinx'

# -- Options for HTML output ----------------------------------------------

html_theme = 'alabaster'
html_title = project

html_static_path = ['_static']

html_sidebars = {
   '**': ['localtoc.html'],
}

html_use_index = False
html_show_sourcelink = False
html_show_sphinx = False
html_show_copyright = False
