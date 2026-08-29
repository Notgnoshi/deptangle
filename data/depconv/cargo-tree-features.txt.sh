#!/bin/sh
# Feature-annotated cargo tree, depth-limited to 2. Path to the deptangle project removed
cd "$(git rev-parse --show-toplevel)" || exit 1
cargo tree -p deptangle-depgraph -e features --depth 2 | sed "s|$HOME/.*/deptangle/|deptangle/|g"
