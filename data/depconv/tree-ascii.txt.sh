#!/bin/sh
cd "$(git rev-parse --show-toplevel)" || exit 1
# Filesystem tree of deptangle/crates, depth-limited to 3, ASCII charset.
tree --charset=ascii -L 3 crates
