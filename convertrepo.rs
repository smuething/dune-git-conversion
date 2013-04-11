# load SVN dump file, apply author map
script loadrepo.rs $1

# run cleanup script
script $1.rs

# remove existing indentation logs
!rm -f $1-indent-errors.log
!rm -f $1-indent-ignored.log

# run reindent script if necessary
script $2.rs $1

# create Git repository
prefer git
rebuild $1.git
