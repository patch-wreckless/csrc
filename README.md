# csrc

## Prerequisites

Must have `fzf` installed.

## Installation

Build the binary then use a shell function to invoke it and `cd` to returned directory.

```sh
csrc() {
  local dir
  dir="$(command csrc "$@")" || return
  [ -n "$dir" ] && cd "$dir"
}
```
