# load SVN dump file, apply author map
script loadrepo.rs dune-geometry

# There is no branch for this release in the repository, so
# we just turn the root tag into the actual release tag
tag pdelab-course-201203-root rename pdelab-course-201203

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

# remove existing indentation logs
!rm -f dune-geometry-indent-errors.log
!rm -f dune-geometry-indent-ignored.log

# reindent files in history
filter =B & 1..$ ./uncrustify-rs.py -d dune-geometry-indent-errors.log -i dune-geometry-indent-ignored.log -e -m -r %EVENT% %PATHS%

# create Git repository
prefer git
rebuild dune-geometry.git
