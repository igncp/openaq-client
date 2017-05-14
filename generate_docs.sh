#!/usr/bin/env bash

git merge --no-edit master

rm -rf target

cargo doc --no-deps

rm -rf doc

cp -r target/doc doc

echo '<meta http-equiv=refresh content=0;url=doc/openaq_client>' > index.html
echo '<meta http-equiv=refresh content=0;url=openaq_client>' > doc/index.html
