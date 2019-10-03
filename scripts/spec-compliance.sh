#!/usr/bin/env bash

set -euo pipefail
set -x

shopt -s globstar

cargo build
spec_runner="$(pwd)/target/debug/spec-runner"

run_core_spec() {
  pushd "spec-runner/vendor/spec/" >/dev/null
  $spec_runner ./**/shared/**/*.rb core/"$1"/**/*.rb
  popd >/dev/null
}

run_library_spec() {
  pushd "spec-runner/vendor/spec/library" >/dev/null
  $spec_runner ../**/shared/**/*.rb "$1"/**/*.rb
  popd >/dev/null
}

run_core_spec "array"

