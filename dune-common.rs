# fix incorrect parent ordering on several merge commits
edit <6357> perl -pi -e "s/^Parents:.*$/Parents: :17173 :17190/"
edit <7131> perl -pi -e "s/^Parents:.*$/Parents: :19150 :18304 :18368 :19084 :18769 :19144 :18490 :19244 :18361 :18706 :18662/;" -e "s/:18456 :19082 :19150/:18456 :19082/"
edit <6048> perl -pi -e "s/^Parents:.*$/Parents: :16352 :16401 :16255 :16179 :16297 :16215 :16147 :16302 :16323 :16310 :16331/;" -e "s/:16245 :16239 :16261 :16352/:16245 :16239 :16261/"

# delete bogus commits created by releases/ -> tags/ rename
delete =C & <6524.1>..<6524.9> obliterate tagback

# rename 2.1.0 tag for consistency
tag release-2.1.0 rename v2.1.0

# There is no branch for this release in the repository, so
# we just turn the root tag into the actual release tag
tag pdelab-course-201203-root rename pdelab-course-201203

# delete and recreate all tags that existed at the time of the
# releases/ -> tags/ rename to fix their metadata
tag 1.0 delete
tagify v1.0 <1.0>
tag 1.0.1 delete
tagify v1.0.1 <1.0.1>
tag 1.1 delete
tagify v1.1 <1.1>
tag 1.1.1 delete
tagify v1.1.1 <1.1.1>
tag 1.2 delete
tagify v1.2 <1.2>
tag 1.2.1 delete
tagify v1.2.1 <1.2.1>
tag 1.2.2 delete
tagify v1.2.2 <1.2.2>

# reposurgeon only creates release branches for these two
# releases - add corresponding tags
tagify v2.0 <2.0>
tagify v2.0.1 <2.0.1>

# reposurgeon mis-tags the 2.2.0 release. Clean that up
tag 2.2.0 delete
tagify v2.2.0 <6787>

# rename remaining tags to add 'v' prefix
tag 2.1.1 rename v2.1.1
tag 2.2beta1 rename v2.2beta1
tag 2.2beta2 rename v2.2beta2
tag 2.2.1 rename v2.2.1

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
tag develop-r6467-introduce-dune-geometry-root delete
tag develop-r6501-introduce-dune-geometry-root delete
tag mn-devel-root delete
tag release-2.1-root delete
tag release-2.2-root delete
tag release-cmake-2.2-root delete
tag rk-diss-root delete
tag rk-diss-root delete
tag ldflags-transition-root delete

# remove tipdelete tags
tag tipdelete-rk-diss delete
tag tipdelete-ldflags-transition delete
tag tipdelete-release-2.1.0 delete
tag tipdelete-develop-r6467-introduce-dune-geometry delete
tag tipdelete-develop-r6501-introduce-dune-geometry delete

# remove tags for empty commit events
tag emptycommit-5066 delete
tag emptycommit-6348 delete
tag emptycommit-6632 delete

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
branch cmake rename refs/heads/feature/cmake
branch develop-r6501-introduce-dune-geometry rename refs/heads/feature/introduce-dune-geometry
branch ldflags-transition rename refs/heads/feature/ldlfags-transition
branch mn-devel rename refs/heads/p/mnolte/devel
branch rk-diss rename refs/heads/p/robertk/diss
