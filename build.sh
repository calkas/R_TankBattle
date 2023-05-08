#! /bin/bash
echo "Start building..."
cargo build
cp -R ./assets ./target/debug/
echo "Done"