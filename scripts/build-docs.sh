#!/bin/bash -xe

# Build documentation
cargo doc --no-deps

# Get previous documentation
git clone https://github.com/$TRAVIS_REPO_SLUG --branch gh-pages gh-pages
rm -rf gh-pages/.git
cd gh-pages

# Copy the right directory
if [[ "$TRAVIS_TAG" != "" && "${TRAVIS_OS_NAME}" == "linux" ]]; then
    mv ../target/doc/ $TRAVIS_TAG
    cp _redirect.html $TRAVIS_TAG/index.html
else
    rm -rf latest
    mv ../target/doc/ latest
    cp _redirect.html latest/index.html
fi
