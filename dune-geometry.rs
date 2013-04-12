# fix incorrect parent ordering on merge commits
edit <166> perl -pi -e "s/^Parents:.*$/Parents: :535 :580/"

# There is no branch for this release in the repository, so
# we just turn the root tag into the actual release tag
tag pdelab-course-201203-root rename pdelab-course-201203

# Add 'v' prefix to version tags
tag 2.2beta1 rename v2.2beta1
tag 2.2beta2 rename v2.2beta2
tag 2.2.0 rename v2.2.0
tag 2.2.1 rename v2.2.1

# Bake SVN revisions into commit messages
edit =C & 1..$ ./add-fossil-ids.py

# remove branch root tags
tag cmake-root delete
tag mn-devel-root delete
tag release-2.2-root delete
tag gg-fix-root delete

# remove tipdelete tags
tag tipdelete-gg-fix delete

# fix naming of branches to new, hierarchical scheme
branch release-2.2 rename refs/heads/releases/2.2
branch release-cmake-2.2 rename refs/heads/releases/2.2-cmake
branch cmake rename refs/heads/feature/cmake
branch gg-fix rename refs/heads/feature/gg-fix
branch mn-devel rename refs/heads/p/mnolte/devel
