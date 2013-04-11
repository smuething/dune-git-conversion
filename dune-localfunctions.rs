# There is no branch for this release in the repository, so
# we just turn the root tag into the actual release tag
tag pdelab-course-201203-root rename pdelab-course-201203

# delete bogus commits created by releases/ -> tags/ rename
delete =C & <1023.1>..<1023.2> obliterate tagback

# fix ancestry of 1.2snapshot branch
merge <98>,<112>

# rename 2.1.0 tag for consistency
tag release-2.1.0 rename 2.1.0

# reposurgeon only creates release branches for these two
# releases - add corresponding tags
tagify 2.0 <2.0>
tagify 2.0.1 <2.0.1>

# Bake SVN revisions into commit messages
edit =C & 1..$ ./add-fossil-ids.py

# remove branch root tags
tag 2.0-root delete
tag 2.0.1-root delete
tag cmake-root delete
tag develop-r978-introduce-dune-geometry-root delete
tag develop-r1006-introduce-dune-geometry-root delete
tag mn-devel-root delete
tag release-2.1-root delete
tag release-2.2-root delete

# remove tipdelete tags
tag tipdelete-release-2.1.0 delete
tag tipdelete-develop-r978-introduce-dune-geometry delete
tag tipdelete-develop-r1006-introduce-dune-geometry delete
tag tipdelete-mn-devel delete

# remove tags for empty commit events
tag emptycommit-6 delete
tag emptycommit-14 delete
tag emptycommit-17 delete
tag emptycommit-19 delete
tag emptycommit-99 delete
tag emptycommit-258 delete
tag emptycommit-696 delete
tag emptycommit-697 delete
tag emptycommit-698 delete
tag emptycommit-722 delete
tag emptycommit-791 delete
tag emptycommit-958 delete
tag emptycommit-969 delete

# fix naming of branches to new, hierarchical scheme
branch 1.2snapshot rename refs/heads/releases/1.2snapshot
branch 2.0 rename refs/heads/releases/2.0
branch 2.0.1 rename refs/heads/releases/2.0.1
branch release-2.1 rename refs/heads/releases/2.1
branch release-2.2 rename refs/heads/releases/2.2
branch cmake rename refs/heads/feature/cmake
branch develop-r1006-introduce-dune-geometry rename refs/heads/feature/introduce-dune-geometry

# remove mn-devel branch, there were never any commits to it
branch mn-devel delete
