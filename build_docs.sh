#!/bin/bash

set -o errexit

shopt -s globstar

# build individual documentation
(cd plugin && cargo doc)
(cd runtime && cargo doc)

# remove old docs build
rm -rf build_docs

# build directory structure
mkdir ./build_docs
mkdir ./build_docs/language
mkdir ./build_docs/plugin
mkdir ./build_docs/runtime

# create instruction reference markdown file
(cd doc/insref && cargo run > ../instructionref.md)

# build plugin docs
for f in ./doc/*.md; do
    rustdoc $f -o ./build_docs/language --markdown-no-toc --html-before-content=./doc/pre.html --html-after-content=./doc/post.html --markdown-css=./formatting.css
done
cp ./doc/formatting.css ./build_docs/language/formatting.css

# copy over the docs folders
cp -r ./plugin/target/doc/* ./build_docs/plugin
cp -r ./runtime/target/doc/* ./build_docs/runtime

# hack in javascript
cat ./doc/hack.js >> ./build_docs/plugin/search-index.js
cat ./doc/hack.js >> ./build_docs/runtime/search-index.js

if [ "$1" == "commit" ]; then
    git clone --branch gh-pages --depth 1 "git@github.com:CensoredUsername/dynasm-rs.git" deploy_docs
    cd deploy_docs
    git config user.name "CensoredUsername"
    git config user.email "cens.username@gmail.com"
    cp ../build_docs/* ./ -r
    git add .
    git commit -m "Rebuild docs"
    git push origin gh-pages
    cd ..
    rm deploy_docs -rf
fi

exit
