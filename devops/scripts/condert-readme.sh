#!/usr/bin/env bash

for f in ./*/*/Cargo.toml; do
  echo $(dirname "$f")
  asciidoctor -r asciidoctor-mdpp -b mdpp $(dirname "$f")/readme.adoc -o $(dirname "$f")/README.md
done
