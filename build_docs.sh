#!/bin/bash

set -o errexit

shopt -s globstar

echo "build individual documentation"
(cd plugin && cargo doc --no-deps)
(cd runtime && cargo doc --no-deps)

echo "remove old docs build"
rm -rf build_docs

echo "build directory structure"
mkdir ./build_docs
mkdir ./build_docs/language
mkdir ./build_docs/plugin
mkdir ./build_docs/runtime

echo "create instruction reference markdown file"
(cd doc/insref && cargo update && cargo run -- x64 > ../instructionref_x64.md && cargo run -- aarch64 > ../instructionref_aarch64.md)

echo "build plugin docs"
for f in ./doc/*.md; do
    rustdoc $f -o ./build_docs/language --markdown-no-toc --html-before-content=./doc/pre.html --html-after-content=./doc/post.html --markdown-css=./formatting.css
done
cp ./doc/formatting.css ./build_docs/language/formatting.css

echo "copy over the docs folders"
cp -r ./plugin/target/doc/* ./build_docs/plugin
cp -r ./runtime/target/doc/* ./build_docs/runtime

echo "insert javascript"
cat ./doc/hack.js >> ./build_docs/plugin/search-index.js
cat ./doc/hack.js >> ./build_docs/runtime/search-index.js

echo "copy docs examples to tests"
declare -a examples=("bf-jit" "hello-world" "bf-interpreter")
for EX in "${examples[@]}"
do
    TARGET=$(echo $EX | tr - _)
    if [ -f "./doc/examples/${EX}/src/main.rs" ]; then
        cp "./doc/examples/${EX}/src/main.rs" "./testing/tests/${TARGET}.rs"
        echo -n -e "#[test]\nfn ex_${TARGET}()\n{\n    main();\n}\n" >> \
             "./testing/tests/${TARGET}.rs"
    fi
    if [ -f "./doc/examples/${EX}/src/x64.rs" ]; then
        cp "./doc/examples/${EX}/src/x64.rs" "./testing/tests/${TARGET}_x64.rs"
        echo -n -e "#[cfg(target_arch=\"x64\")]\n#[test]\nfn ex_${TARGET}()\n{\n    main();\n}\n" >> \
             "./testing/tests/${TARGET}_x64.rs"
    fi
    if [ -f "./doc/examples/${EX}/src/aarch64.rs" ]; then
        cp "./doc/examples/${EX}/src/aarch64.rs" "./testing/tests/${TARGET}_aarch64.rs"
        echo -n -e "#[cfg(target_arch=\"aarch64\")]\n#[test]\nfn ex_${TARGET}()\n{\n    main();\n}\n" >> \
             "./testing/tests/${TARGET}_aarch64.rs"
    fi
done

if [ "$1" == "commit" ]; then
    echo "cloning gh-pages into a temporary directory"
    git clone --branch gh-pages --depth 1 "git@github.com:CensoredUsername/dynasm-rs.git" deploy_docs
    git gc
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
