#!/usr/bin/env bash

//NOTE:  gem install asciidoctor asciidoctor-mdpp
for f in ./*/*/Cargo.toml; do
  echo $(dirname "$f")
  asciidoctor -r asciidoctor-mdpp -b mdpp $(dirname "$f")/readme.adoc -o $(dirname "$f")/README.md
done

asciidoctor -r asciidoctor-mdpp -b mdpp argentum_rest_generator_app/readme.adoc -o argentum_rest_generator_app/README.md
