# enable workaround in reposurgeon to avoid crash
# when loading the svn dump
set svn_accept_orphan_branches

# load svn dump and apply author map
script loadrepo.rs dune-grid-dev-howto

# delete bogus commits created by releases/ -> tags/ rename
delete =C & <102.1>..<102.2> obliterate tagback

# delete and recreate all tags that existed at the time of the
# releases/ -> tags/ rename to fix their metadata
tag 1.2 delete
tagify v1.2 <1.2>

# rename 2.1.0 tag for consistency
tag release-2.1.0 rename v2.1.0

# creating missing tag for 2.0 release
tagify v2.0 <2.0>

# creating missing tag for 2.1.1 release
tagify v2.1.1 :260

# Add 'v' prefix to remaining version tags
tag 2.2beta1 rename v2.2beta1
tag 2.2beta2 rename v2.2beta2
tag 2.2.0 rename v2.2.0
tag 2.2.1 rename v2.2.1


# Bake SVN revisions into commit messages
edit =C & 1..$ ./add-fossil-ids.py

# remove branch root tags
tag 1.2-root delete
tag 2.0-root delete
tag release-2.1-root delete
tag release-2.2-root delete

# remove tipdelete tags
tag tipdelete-release-2.1.0 delete

# remove tags for empty commit events
tag emptycommit-23 delete
tag emptycommit-91 delete
tag emptycommit-92 delete

# fix naming of branches to new, hierarchical scheme
branch 1.2 rename refs/heads/releases/1.2
branch 2.0 rename refs/heads/releases/2.0
branch release-2.1 rename refs/heads/releases/2.1
branch release-2.2 rename refs/heads/releases/2.2

# remove existing indentation logs
!rm -f dune-grid-dev-howto-indent-errors.log
!rm -f dune-grid-dev-howto-indent-ignored.log

# reindent files in history
filter =B & 1..$ ./uncrustify-rs.py -d dune-grid-dev-howto-indent-errors.log -i dune-grid-dev-howto-indent-ignored.log -e -m -r %EVENT% %PATHS%

# create Git repository
prefer git
rebuild dune-grid-dev-howto.git
