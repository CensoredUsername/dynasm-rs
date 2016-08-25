#!/bin/bash

# build individual documentation
(cd plugin && cargo doc --no-deps)
(cd runtime && cargo doc --no-deps)

# remove old docs build
rm -rf build_docs

# build directory structure
mkdir ./build_docs
mkdir ./build_docs/language
mkdir ./build_docs/plugin
mkdir ./build_docs/runtime

# build plugin docs
for f in ./plugin/doc/*.md; do
    rustdoc $f -o ./build_docs/language --html-before-content=./plugin/doc/pre.html --html-after-content=./plugin/doc/post.html --markdown-css=./formatting.css
done
cp ./plugin/doc/formatting.css ./build_docs/language/formatting.css

# copy over the docs folders
cp -r ./plugin/target/doc/* ./build_docs/plugin
cp -r ./runtime/target/doc/* ./build_docs/runtime

exit
