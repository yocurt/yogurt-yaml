# Yogurt Yaml

[![Actions Status](https://github.com/yocurt/yogurt-yaml/workflows/Cargo%20Release/badge.svg)](https://github.com/yocurt/yogurt-yaml/actions?query=workflow%3A%22Cargo+Release%22)
[![Actions Status](https://github.com/yocurt/yogurt-yaml/workflows/Rust%20Build%20Pipeline/badge.svg)](https://github.com/yocurt/yogurt-yaml/actions?query=workflow%3A%22Rust+Build+Pipeline%22)
[![Actions Status](https://github.com/yocurt/yogurt-yaml/workflows/Trigger%20Docs%20Update/badge.svg)](https://github.com/yocurt/yogurt-yaml/actions?query=workflow%3A%22Trigger+Docs+Update%22)
[![Actions Status](https://github.com/yocurt/yogurt-yaml/workflows/Security%20audit/badge.svg)](https://github.com/yocurt/yogurt-yaml/actions?query=workflow%3A%22Security+audit%22)


This package allows the user to extract yaml from yogurt files, where yogurt files are bassically all files containing a `Identifier[.*]`-like syntax. This package only extracts `ID[.*]`, `REF[.*]`, `ADD[.*]` or `END[.*]`. There will be a cli version with more options and functionality: [yogurt-cli](https://github.com/yocurt/yogurt-cli).

## Usage

There is a lib and a executable, which can be used to extract yaml content specified by e.g.: `ID[.*]`, `REF[.*]`, `ADD[.*]` or `END[.*]`. Or via tags e.g.: `#tag: content\n` or `@name`

### Pipe file.md

``` md
# Title

Text in a file.

ID[NAME, attribute: value]

## Next Title

More text

REF[NAME, attribute: value, other_attribute: other_value]
```

### Into curt-extract

``` bash
 cat file.md | curt-extract -b "ID REF" -t "@ #" > result.yaml
```

### Returns result.yaml

``` yaml
- {ID: NAME, attribute: value}
- {REF: NAME, attribute: value, other_attribute: other_value}
```

## CLI Examples

### Simple Example

It is possible to extract yaml from any file.

``` bash
cat file | curt-extract -b ID
```

### More sophisticated Example

Other commandline tools can be used to extend the functionality.

``` bash
cat **/*.adoc | curt-extract -b "ID REF" -t "@ #" | yaml json write - | less
```
