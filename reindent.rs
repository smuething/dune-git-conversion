# remove existing indentation logs
!rm -f $1-indent-errors.log
!rm -f $1-indent-ignored.log

# reindent files in history
filter =B & 1..$ ./uncrustify-rs.py -d $1-indent-errors.log -i $1-indent-ignored.log -e -m -r %EVENT% %PATHS%
