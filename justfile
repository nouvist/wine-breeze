linux_toolchain := env("LINUX_TOOLCHAIN", "musl")

default:
  @just --list

alias b := build
alias t := test
alias r := run
alias rw := run-wrapper

build: build-win32 build-linux
test: test-win32 test-linux

build-win32 profile="release":
  cargo build --target x86_64-pc-windows-gnu --profile={{profile}}

build-linux profile="release":
  cargo build --target x86_64-unknown-linux-{{linux_toolchain}} --profile={{profile}}

test-win32:
  cargo test --target x86_64-pc-windows-gnu

test-linux:
  cargo test --target x86_64-unknown-linux-{{linux_toolchain}}

run *args:
  cargo run --target x86_64-pc-windows-gnu -- {{args}}

run-wrapper *args: (build-linux "dev")
  cargo run --target x86_64-unknown-linux-{{linux_toolchain}} -- {{args}}
