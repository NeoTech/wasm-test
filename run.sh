#!/bin/env bash
echo "this will fail if you do not have SFZ";
wasm-pack build --target web && sfz