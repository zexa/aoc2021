#!/bin/bash
cp -R ./template $1
file="./$1/Cargo.toml"
gsed -z "s/REPLACE-ME/$1/g" $file -i $file

