#! /bin/bash

set -eu

source build-emscripten.sh

export BRANCH=gh-pages
export DEPLOY_DIR=gh-pages
export DEPLOY_REPO=origin

echo "Cloning ${BRANCH} into ${DEPLOY_DIR}/..."

rm -rf ${DEPLOY_DIR}
git clone . -b ${BRANCH} ${DEPLOY_DIR}

echo "Copying files from ${OUTPUT_DIR}/ to ${DEPLOY_DIR}/..."

cd ${DEPLOY_DIR}
git rm *
cp ../${OUTPUT_DIR}/* .
git add *.html *.js *.wasm

echo "Committing changed files to git..."
git commit -m "Update deployment."
git push

echo "Pushing ${BRANCH} to ${DEPLOY_REPO}..."
cd ..
git push ${DEPLOY_REPO} ${BRANCH}
