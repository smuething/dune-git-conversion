# There are no tags in the SVN repository, so we need to manually
# add any tags we want to have in the Git repository
# tagify v1.0 <1.0>

# Bake SVN revisions into commit messages
edit =C & 1..$ ./add-fossil-ids.py

# remove branch root tags
tag 2.0compatible-root delete
tag release-2.1-root delete
tag release-2.2-root delete

# remove tipdelete tags
tag tipdelete-dune-grid-glue delete

# remove tags for empty commit events
tag emptycommit-21 delete
tag emptycommit-409 delete
tag emptycommit-445 delete

# remove bogus branch 'dune-grid-glue' stuck at the initial commit
branch dune-grid-glue delete

# fix naming of branches to new, hierarchical scheme
branch 2.0compatible rename refs/heads/releases/2.0compatible
branch release-2.1 rename refs/heads/releases/2.1
