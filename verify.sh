#!/bin/bash

GITDIR=$1
SVNREPO=$2
SVNDIR=$3

while read GITHEAD SVNPATH;
do
    echo "Comparing Git head $GITHEAD with SVN path $SVNPATH"
    pushd $GITDIR
    git checkout -q $GITHEAD
    popd
    rm -rf $SVNDIR
    svn export -q --ignore-keywords $SVNREPO/$SVNPATH $SVNDIR
    diff -u -x .git -x .gitignore -r $GITDIR $SVNDIR
    rm -rf $SVNDIR
done
