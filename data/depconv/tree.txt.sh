#!/bin/sh
cd "$(git rev-parse --show-toplevel)" || exit 1
# Filesystem tree of deptangle/crates, depth-limited to 3.
tree -L 3 crates
