# deptangle

![lint workflow](https://github.com/Notgnoshi/deptangle/actions/workflows/lint.yml/badge.svg?event=push)
![code coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/Notgnoshi/0e15edf83d41c5b3cace2ff71f4d2f53/raw/deptangle-coverage.json)

Tools to interrogate and detangle dependency graphs

## Table of contents

# Philosophy

Rather than build an infinitely flexible, does-everything-and-more graph toolkit, these are targeted
tools to solve specific dependency graph problems I frequently encounter.

All tools operate on stdin/stdout in addition to files, and are designed to be chained together with
pipes. Any ancillary output is emitted on stderr.
