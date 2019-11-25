#!/bin/bash

git remote add upstream https://github.com/learnrusttogether/exercises.git
git fetch upstream
git checkout master
git merge upstream/master
#
