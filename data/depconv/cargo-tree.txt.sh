#!/bin/sh
cd "$(git rev-parse --show-toplevel)" || exit 1
# Default cargo tree output for deptangle-depgraph crate, with the path to the deptangle project removed
cargo tree -p deptangle-depgraph | sed "s|$HOME/.*/deptangle/|deptangle/|g"
