#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.
# Source https://github.com/casey/just/blob/master/justfile

# choose a recipe interactively
default:
  just --choose

#
# VARIABLES
#

docker_image := "treeleaf"
docker_container_name := 'treeleaf'
username := 'lloydlobo'

alias t := test

alias c := check

alias r := run

bt := '0'

export RUST_BACKTRACE := bt

log := "warn"

export JUST_LOG := log

test:
  cargo test

ci: build-book
  cargo test --all
  cargo clippy --all --all-targets
  cargo fmt --all -- --check
  ./bin/forbid
  cargo update --locked --package just

fuzz:
  cargo +nightly fuzz run fuzz-compiler

run:
  cargo run

# only run tests matching PATTERN
filter PATTERN:
  cargo test {{PATTERN}}

build:
  cargo build

fmt:
  cargo fmt --all

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"

man:
  cargo build --features help4help2man
  help2man \
    --name 'save and run commands' \
    --manual 'Just Manual' \
    --no-info \
    target/debug/just \
    > man/just.1

view-man: man
  man man/just.1

# add git log messages to changelog
update-changelog:
  git log --pretty='format:- %s' >> CHANGELOG.md

update-contributors:
  cargo run --release --package update-contributors

check: fmt clippy test forbid
  #!/usr/bin/env bash
  set -euxo pipefail
  git diff --no-ext-diff --quiet --exit-code
  VERSION=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  grep "^\[$VERSION\]" CHANGELOG.md

# publish current GitHub master branch
publish:
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  git clone git@github.com:casey/just.git tmp/release
  cd tmp/release
  ! grep '<sup>master</sup>' README.md
  VERSION=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  git tag -a $VERSION -m "Release $VERSION"
  git push origin $VERSION
  cargo publish
  cd ../..
  rm -rf tmp/release

readme-version-notes:
  grep '<sup>master</sup>' README.md

push: check
  ! git branch | grep '* master'
  git push github

pr: push
  gh pr create --web

# clean up feature branch BRANCH
done BRANCH=`git rev-parse --abbrev-ref HEAD`:
  git checkout master
  git diff --no-ext-diff --quiet --exit-code
  git pull --rebase github master
  git diff --no-ext-diff --quiet --exit-code {{BRANCH}}
  git branch -D {{BRANCH}}

# install just from crates.io
install:
  cargo install -f just

# install development dependencies
install-dev-deps:
  rustup install nightly
  rustup update nightly
  cargo +nightly install cargo-fuzz
  cargo install cargo-check
  cargo install cargo-watch
  cargo install mdbook mdbook-linkcheck

# install system development dependencies with homebrew
install-dev-deps-homebrew:
  brew install help2man

# everyone's favorite animate paper clip
clippy:
  cargo clippy --all --all-targets --all-features

forbid:
  ./bin/forbid

# count non-empty lines of code
sloc:
  @cat src/*.rs | sed '/^\s*$/d' | wc -l

replace FROM TO:
  sd '{{FROM}}' '{{TO}}' src/*.rs

test-quine:
  cargo run -- quine

# make a quine, compile it, and verify it
quine:
  mkdir -p tmp
  @echo '{{quine-text}}' > tmp/gen0.c
  cc tmp/gen0.c -o tmp/gen0
  ./tmp/gen0 > tmp/gen1.c
  cc tmp/gen1.c -o tmp/gen1
  ./tmp/gen1 > tmp/gen2.c
  diff tmp/gen1.c tmp/gen2.c
  rm -r tmp
  @echo 'It was a quine!'

quine-text := '
  int printf(const char*, ...);

  int main() {
    char *s =
      "int printf(const char*, ...);"
      "int main() {"
      "   char *s = %c%s%c;"
      "  printf(s, 34, s, 34);"
      "  return 0;"
      "}";
    printf(s, 34, s, 34);
    return 0;
  }
'

render-readme:
  #!/usr/bin/env ruby
  require 'github/markup'
  $rendered = GitHub::Markup.render("README.adoc", File.read("README.adoc"))
  File.write('tmp/README.html', $rendered)

watch-readme:
  just render-readme
  fswatch -ro README.adoc | xargs -n1 -I{} just render-readme

generate-completions:
  ./bin/generate-completions

test-completions:
  ./tests/completions/just.bash

build-book:
  cargo run --package generate-book
  mdbook build book/en
  mdbook build book/zh

convert-integration-test test:
  cargo expand --test integration {{test}} | \
    sed \
    -E \
    -e 's/#\[cfg\(test\)\]/#\[test\]/' \
    -e 's/^ *let test = //' \
    -e 's/^ *test[.]/./' \
    -e 's/;$//' \
    -e 's/crate::test::Test/Test/' \
    -e 's/\.run\(\)/.run();/'

# run all polyglot recipes
polyglot: _python _js _perl _sh _ruby
# (recipes that start with `_` are hidden from --list)

_python:
  #!/usr/bin/env python3
  print('Hello from python!')

_js:
  #!/usr/bin/env node
  console.log('Greetings from JavaScript!')

_perl:
  #!/usr/bin/env perl
  print "Larry Wall says Hi!\n";

_sh:
  #!/usr/bin/env sh
  hello='Yo'
  echo "$hello from a shell script!"

_ruby:
  #!/usr/bin/env ruby
  puts "Hello from ruby!"

# Print working directory, for demonstration purposes!
pwd:
  echo {{invocation_directory()}}

# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :


##############################################
##                 DOCKER                   ##
##############################################

#
# BUILD
#

docker-build:
  docker build -t {{docker_image}} .

#
# DEV
#

# run cmd container in detatched mode at 8080
docker-run:
  docker run -dp 8080:3030 --rm --name {{docker_container_name}} {{docker_image}}

# Run the binary interactively in the terminal `$ ./infinityper`
docker-run-entrypoint:
  docker run -it --rm --name {{docker_container_name}} --entrypoint bin/bash {{docker_image}}

# fetches container logs & follows log output
docker-logs:
  docker logs -f {{docker_container_name}}

# stop container `sample1`
docker-stop:
  docker stop {{docker_container_name}}

