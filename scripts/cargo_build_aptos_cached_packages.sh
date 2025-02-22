#!/bin/sh

# This script ensures that libra2-cached-packages has been built correctly.
#
# If you want to run this from anywhere in aptos-core, try adding the wrapper
# script to your path:
# https://gist.github.com/banool/e6a2b85e2fff067d3a215cbfaf808032

# Make sure we're in the root of the repo.
if [ ! -d ".github" ]
then
    echo "Please run this from the root of aptos-core!"
    exit 1
fi

# Run in check mode if requested.
CHECK_ARG=""
if [ "$1" = "--check" ]; then
    CHECK_ARG="--check"
fi

# Set appropriate script flags
set -e
set -x

# Ensure that libra2-cached-packages have been built correctly.
unset SKIP_FRAMEWORK_BUILD
cargo build -p libra2-cached-packages
if [ -n "$CHECK_ARG" ]; then
    if [ -n "$(git status --porcelain -uno aptos-move)" ]; then
      git diff
      echo "There are unstaged changes after running 'cargo build -p libra2-cached-packages'! Are you sure libra2-cached-packages is up-to-date?"
      exit 1
    fi
fi
