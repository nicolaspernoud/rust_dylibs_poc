#!/bin/bash

WD="$(
    cd "$(dirname "$0")"
    pwd -P
)"

cd ${WD}

cd dynamic_library
cargo build --release


cp target/release/libdynamic_library.{so,dylib,dll} ../main_project

cd ../main_project
cargo run
