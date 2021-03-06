# fix incorrect parent ordering on merge commits
edit <1793> perl -pi -e "s/^Parents:.*$/Parents: :4788 :4627 :4556 :4719 :4402 :4706 :4439 :4502 :4481 :4595 :4416 :4581/;" -e "s/:3898 :4388 :4421 :4169 :4788/:3898 :4388 :4421 :4169/"


# delete bogus commits created by releases/ -> tags/ rename
delete =C & <1505.1>..<1505.8> obliterate tagback

# rename 2.1.0 tag for consistency
tag release-2.1.0 rename v2.1.0

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

# Add missing tag for 2.0 release
tagify v2.0 <2.0>

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
# some tags appear twice in this list because reposurgeon created them
# twice due to the weird history of the SVN repo
tag 1.0.1-root delete
tag 1.1-root delete
tag 1.1.1-root delete
tag 1.2-root delete
tag 1.2.1-root delete
tag 1.2.2-root delete
tag 2.0-root delete
tag cmake-root delete
tag pdelab-course-201203-root delete
tag release-2.1-root delete
tag release-2.2-root delete
tag speedup_amg_build-root delete
tag amg-systems-root delete
tag release-cmake-2.2-root delete
tag rk-diss-root delete
tag rk-diss-root delete

# remove tipdelete tags
tag tipdelete-rk-diss delete
tag tipdelete-amg-systems delete
tag tipdelete-release-2.1.0 delete
tag tipdelete-mn-devel delete

# remove tags for empty commit events
tag emptycommit-701 delete
tag emptycommit-1325 delete
tag emptycommit-1454 delete
tag emptycommit-1501 delete

# fix naming of branches to new, hierarchical scheme
branch 1.0 rename refs/heads/releases/1.0
branch 1.0.1 rename refs/heads/releases/1.0.1
branch 1.1 rename refs/heads/releases/1.1
branch 1.1.1 rename refs/heads/releases/1.1.1
branch 1.2 rename refs/heads/releases/1.2
branch 1.2.1 rename refs/heads/releases/1.2.1
branch 1.2.2 rename refs/heads/releases/1.2.2
branch 2.0 rename refs/heads/releases/2.0
branch release-2.1 rename refs/heads/releases/2.1
branch release-2.2 rename refs/heads/releases/2.2
branch release-cmake-2.2 rename refs/heads/releases/2.2-cmake
branch pdelab-course-201203 rename refs/heads/releases/pdelab-course-201203
branch cmake rename refs/heads/feature/cmake
branch mn-devel rename refs/heads/p/mnolte/devel
branch rk-diss rename refs/heads/p/robertk/diss
branch speedup_amg_build rename refs/heads/feature/speedup-amg-build
branch dune-istl-amg-novlap rename feature/amg-novlap
