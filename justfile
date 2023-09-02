#!/usr/bin/env -S just --justfile

_default:
  just --list -u

alias r := ready
alias c := codecov
alias t := test

# Initialize the project by installing all the necessary tools.
# Make sure you have cargo-binstall installed.
# You can download the pre-compiled binary from <https://github.com/cargo-bins/cargo-binstall#installation>
# or install via `cargo install cargo-binstall`
init:
  cargo binstall cargo-nextest cargo-watch cargo-insta typos-cli taplo-cli cargo-llvm-cov tauri-cli -y && \
    cargo install diesel_cli --no-default-features --features "postgres uuid" && \
    pnpm install -C client && \
    pnpm -C client build

# When ready, run the same CI commands
ready:
  pnpm -C client lint-fix
  typos
  cargo fmt
  just check
  just test
  just lint
  git status

# Update our local branch with the remote branch (this is for you to sync the submodules)
update:
  git pull
  git submodule update --init

# Run `cargo watch`
# --no-vcs-ignores: cargo-watch has a bug loading all .gitignores, including the ones listed in .gitignore
# use .ignore file getting the ignore list
watch command:
  cargo watch --no-vcs-ignores -x '{{command}}'

# Format all files
fmt:
  cargo fmt
  taplo format

# Run cargo check
check:
  cargo ck

# Run all the tests
test:
  cargo nextest run

# Lint the whole project
lint:
  cargo lint -- --deny warnings

# Get code coverage
codecov:
  cargo codecov --html

run-server:
  cargo run --bin calendar_server

run-tauri:
  cargo tauri dev

make-migrations name:
  diesel migration generate {{name}}

run-migrations:
  diesel migration run

revert-migrations:
  diesel migration revert
