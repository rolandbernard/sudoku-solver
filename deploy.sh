#!/bin/bash

SRC_DIR=$(realpath $(dirname "$0"))/dist
TMP_DIR=$(realpath $(dirname "$0"))/tmp.gh-pages
BRANCH=gh-pages
GIT=https://github.com/rolandbernard/sudoku-solver
URL=sudoku-solver

trunk build --release --public-url $URL
cp $SRC_DIR/index.html $SRC_DIR/404.html
cp $SRC_DIR/index.html $SRC_DIR/16.html

mkdir -p $TMP_DIR
cd $TMP_DIR
if [ ! -e ".git" ]
then
    git clone $GIT .
fi
if git rev-parse --quiet --verify $BRANCH > /dev/null
then
    git checkout $BRANCH
else
    git checkout --orphan $BRANCH
    rm -f .gitignore
fi
rm -rf *

cp -r $SRC_DIR/* $TMP_DIR/

git add .
git commit -m "site update"
git push -u origin $BRANCH

