# load SVN dump file, apply author map
script loadrepo.rs dune-grid-howto

# delete bogus commits created by releases/ -> tags/ rename
delete =C & <363.1>..<363.6> obliterate tagback

# There is no branch for this release in the repository, so
# we just turn the root tag into the actual release tag
tag pdelab-course-201203-root rename pdelab-course-201203

# delete bogus commits created by releases/ -> tags/ rename
# delete =C & <1023.1>..<1023.2> obliterate tagback

# rename 2.1.0 tag for consistency
tag release-2.1.0 rename 2.1.0

# delete and recreate all tags that existed at the time of the
# releases/ -> tags/ rename to fix their metadata
tag 1.0 delete
tagify 1.0 <1.0>
tag 1.0.1 delete
tagify 1.0.1 <1.0.1>
tag 1.1 delete
tagify 1.1 <1.1>
tag 1.1.1 delete
tagify 1.1.1 <1.1.1>
tag 1.2 delete
tagify 1.2 <1.2>

# reposurgeon only creates release branches for this release
# releases - add corresponding tag
tagify 2.0 <2.0>

# Bake SVN revisions into commit messages
edit =C & 1..$ ./add-fossil-ids.py

# remove branch root tags
tag 1.0.1-root delete
tag 1.1-root delete
tag 1.1.1-root delete
tag 1.2-root delete
tag 2.0-root delete
tag cmake-root delete
tag mn-devel-root delete
tag release-2.1-root delete
tag release-2.2-root delete

# remove tipdelete tags
tag tipdelete-release-2.1.0 delete
tag tipdelete-mn-devel delete

# remove tags for empty commit events
tag emptycommit-3 delete
tag emptycommit-4 delete
tag emptycommit-5 delete
tag emptycommit-341 delete
tag emptycommit-342 delete

# fix naming of branches to new, hierarchical scheme
branch 1.0 rename refs/heads/releases/1.0
branch 1.0.1 rename refs/heads/releases/1.0.1
branch 1.1 rename refs/heads/releases/1.1
branch 1.1.1 rename refs/heads/releases/1.1.1
branch 1.2 rename refs/heads/releases/1.2
branch 2.0 rename refs/heads/releases/2.0
branch release-2.1 rename refs/heads/releases/2.1
branch release-2.2 rename refs/heads/releases/2.2
branch cmake rename refs/heads/feature/cmake
branch mn-devel rename refs/heads/p/mnolte/devel

# remove existing indentation logs
!rm -f dune-grid-howto-indent-errors.log
!rm -f dune-grid-howto-indent-ignored.log

# reindent files in history
filter =B & 1..$ ./uncrustify-rs.py -d dune-grid-howto-indent-errors.log -i dune-grid-howto-indent-ignored.log -e -m -r %EVENT% %PATHS%

# create Git repository
prefer git
rebuild dune-grid-howto.git
