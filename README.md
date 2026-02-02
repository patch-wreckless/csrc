# csrc

A CLI tool to streamline switching between source directories

## Supported Platforms

- macOS ARM

## Prerequisites

- Must have `fzf` installed.

## Installation

Install the `csrc` binary then use a shell function to invoke it and `cd` to returned directory.

```sh
csrc() {
  local dir
  dir="$(command csrc "$@")" || return
  [ -n "$dir" ] && cd "$dir"
}
```

## Configuration

### Configuration Parameters

`sourceRoot`
: The root directory to search for source directories in. Defaults to the user's `${HOME}` directory.

_NOTE: The cache feature doesn't exist yet, what's docummented is the current plan._

`cache.enabled`
: When `true`, `csrc` maintains a cached list of source directories and their previews. Defaults to `false`.

`cache.directory`
: The directory `csrc` maintains cache data in. Defaults to `${XDG_CACHE_HOME}/csrc` if `XDG_CACHE_HOME` is set, otherwise `${HOME}/cache/csrc/`.

### Specifying Configuration

Configuration can be specified in three ways:

- A YAML configuration file.
- Environment variables.
- Values passed to the `--conf` option.

#### YAML Configuration Files

YAML configuration files can be used to specify configuration values. The YAML structure reflects the [parameter names](#configuration-parameters).

E.g.

```yaml
sourceRoot: ~/src               # sourceRoot
cache:
    enabled: true               # cache.enabled
    directory: ~/.csrc-cache    # cache.directory
```

`csrc` checks the following locations in order and uses the first one that exists as a configuration file. Configuration files are not merged.

| Location | Conditions |
|---|---|
| ${XDG_CONFIG_HOME}/csrc/config.yaml | `XDG_CONFIG_HOME` is set |
| ${XDG_CONFIG_HOME}/csrc.yaml | `XDG_CONFIG_HOME` is set |
| ${HOME}/.config/csrc/config.yaml ||
| ${HOME}/.config/csrc.yaml ||
| ${HOME}/.csrc.yaml ||

#### Environment Variable Configuration

Environment variables can be used to specify configuration values. Values set with environment variables override values set in configuration files.

To specify configuration in an environment variable, convert the [parameter name](#configuration-parameters) to an environment variable name using the following process:

- Replace `.` characters with `__`.
- Convert tokens from camelCase to SCREAMING_SNAKE_CASE.
- Prefix variable name with `CSRC__`

E.g.

```env
CSRC__SOURCE_ROOT=/home/patch-wreckless/src                 # sourceRoot
CSRC__CACHE__ENABLED=true                                   # cache.enabled
CSRC__CACHE__DIRECTORY=/home/patch-wreckless/.csrc-cache    # cache.directory
```

#### CLI Configuration Flags

The CLI supports a `--conf` option that accepts values in the form `{configurationParameterName}={configurationValue}`. Values set with the `--conf` option override values set in environment variables and configuration files.

E.g.

```sh
csrc \
    --conf sourceRoot=~/src \
    --conf cache.enabled=true \
    --conf cache.directory=~/.csrc-cache
```
