#!/bin/sh
# Use rustup to locally run the same suite of tests as .travis.yml.
# (You should first install/update all versions listed below.)

set -ex

export TRAVIS_RUST_VERSION
for TRAVIS_RUST_VERSION in 1.31.0 1.32.0 1.34.0 1.36.0 stable beta nightly; do
    run="rustup run $TRAVIS_RUST_VERSION"
    $run cargo build --verbose
    $run $PWD/ci/test_full.sh
done
