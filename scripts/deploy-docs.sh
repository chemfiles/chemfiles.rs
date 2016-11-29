#!/bin/bash -xe

# Exit early if we don't want to deploy the docs
if [[ "$TRAVIS_BRANCH" != "master" && "$TRAVIS_TAG" == "" ]]; then
    exit 0
fi

# Build documentation
cargo doc

cd gh-pages
git checkout gh-pages

# Copy the right directory
if [[ "$TRAVIS_BRANCH == master" ]]; then
    rm -rf latest
    mv ../target/doc/ latest
    cp _redirect.html latest/index.html
elif [[ "$TRAVIS_TAG" != "" ]]; then
    mv ../target/doc/ $TRAVIS_TAG
    cp _redirect.html $TRAVIS_TAG/index.html
else
    echo "We should have exited earlier"
    exit 1
fi

git add --all .
# Skip push if there is no change
if git diff --cached --exit-code --quiet; then
    echo "No changes to the output on this push; exiting."
    exit 0
fi

# Git configuration
git config --global user.email "luthaf@luthaf.fr"
git config --global user.name "Travis-CI autobuild"
git config --global push.default simple

# Commit the new doc
git commit -a -m "[AUTO-COMMIT] Documentation update" -m "[ci skip]"
git push origin HEAD:gh-pages
