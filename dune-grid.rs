# fix incorrect parent ordering on several merge commits
edit <8025> perl -pi -e "s/^Parents:.*$/Parents: :22908 :21694 :22472 :22093 :22911 :21395/"
edit <7442> perl -pi -e "s/^Parents:.*$/Parents: :21079 :21093/"
edit <8808> perl -pi -e "s/^Parents:.*$/Parents: :25399 :24754 :21395 :25249 :23825 :24539 :21694 :24475 :24469 :24117 :24989/;" -e "s/:25300 :23994 :24098 :24993 :23067 :22472 :24400 :25399/:25300 :23994 :24098 :24993 :23067 :22472 :24400/"

# delete bogus commits created by releases/ -> tags/ rename
delete =C & <7728.1>..<7728.9> obliterate tagback
delete =C & <7568.1>..<7568.7> obliterate tagback

# fix a broken merge link in the vicinity of the broken
# releases/ -> tags/ copy and delete commits
merge <7727>,<7737>

# rename 2.1.0 tag for consistency
tag release-2.1.0 rename v2.1.0

# Due to the split deletion and creation of the releases/ and
# tags/ directories, reposurgeon does not create tags for our
# releases, so we add them manually here.
tagify v1.0 <1.0>
tagify v1.0.1 <1.0.1>
tagify v1.1 <1.1>
tagify v1.1.1 <1.1.1>
tagify v1.2 <1.2>
tagify v1.2.1 <1.2.1>
tagify v1.2.2 <1.2.2>
tagify v2.0 <2.0>
tagify v2.0.1 <2.0.1>

# Add 'v' prefix to remaining version tags
tag 2.1.1 rename v2.1.1
tag 2.2beta1 rename v2.2beta1
tag 2.2beta2 rename v2.2beta2
tag 2.2.0 rename v2.2.0
tag 2.2.1 rename v2.2.1

# create tag for pdelab-course-201203 release
tagify pdelab-course-201203 <pdelab-course-201203>

# Bake SVN revisions into commit messages
edit =C & 1..$ ./add-fossil-ids.py

# remove branch root tags
# some tags appear twice in this list because reposurgeon created
# two copies due to the convoluted repository history
tag 1.0.1-root delete
tag 1.1-root delete
tag 1.1.1-root delete
tag 1.2-root delete
tag 1.2.1-root delete
tag 1.2.2-root delete
tag 2.0-root delete
tag 2.0.1-root delete
tag cmake-root delete
tag develop-r7652-introduce-dune-geometry-root delete
tag develop-r7678-introduce-dune-geometry-root delete
tag feature-vtkwriter-on-all-partition-root delete
tag gg-fix-root delete
tag mn-devel-root delete
tag pdelab-course-201203-root delete
tag release-2.1-root delete
tag release-2.2-root delete
tag release-cmake-2.2-root delete
tag return_geometry_by_value-root delete
tag rk-diss-root delete
tag rk-diss-root delete

# remove tipdelete tags
tag tipdelete-develop-r7652-introduce-dune-geometry delete
tag tipdelete-develop-r7678-introduce-dune-geometry delete
tag tipdelete-gg-fix delete
tag tipdelete-release-2.1.0 delete
tag tipdelete-return_geometry_by_value delete
tag tipdelete-rk-diss delete

# remove tags for empty commit events
tag emptycommit-3964 delete
tag emptycommit-5165 delete
tag emptycommit-5212 delete
tag emptycommit-7435 delete

# fix naming of branches to new, hierarchical scheme
branch 1.0 rename refs/heads/releases/1.0
branch 1.0.1 rename refs/heads/releases/1.0.1
branch 1.1 rename refs/heads/releases/1.1
branch 1.1.1 rename refs/heads/releases/1.1.1
branch 1.2 rename refs/heads/releases/1.2
branch 1.2.1 rename refs/heads/releases/1.2.1
branch 1.2.2 rename refs/heads/releases/1.2.2
branch 2.0 rename refs/heads/releases/2.0
branch 2.0.1 rename refs/heads/releases/2.0.1
branch release-2.1 rename refs/heads/releases/2.1
branch release-2.2 rename refs/heads/releases/2.2
branch release-cmake-2.2 rename refs/heads/releases/2.2-cmake
branch pdelab-course-201203 rename refs/heads/releases/pdelab-course-201203
branch cmake rename refs/heads/feature/cmake
branch develop-r7678-introduce-dune-geometry rename refs/heads/feature/introduce-dune-geometry
branch feature-vtkwriter-on-all-partition rename refs/heads/feature/vtkwriter-on-all-partition
branch gg-fix rename refs/heads/feature/gg-fix
branch return_geometry_by_value rename refs/heads/feature/return-geometry-by-value
branch mn-devel rename refs/heads/p/mnolte/devel
branch rk-diss rename refs/heads/p/robertk/diss
