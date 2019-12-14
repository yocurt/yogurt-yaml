# Yogurt Yaml

[![Actions Status](https://github.com/yocurt/yogurt-yaml/workflows/Cargo%20Release/badge.svg)](https://github.com/yocurt/yogurt-yaml/actions)
[![Actions Status](https://github.com/yocurt/yogurt-yaml/workflows/Rust%20Build%20Pipeline/badge.svg)](https://github.com/yocurt/yogurt-yaml/actions)
[![Actions Status](https://github.com/yocurt/yogurt-yaml/workflows/Trigger%20Docs%20Update/badge.svg)](https://github.com/yocurt/yogurt-yaml/actions)

This package allows the user to extract yaml from yogurt files, where yogurt files are bassically all files containing a `Identifier[.*]`-like syntax. This package only extracts `ID[.*]`, `REF[.*]`, `ADD[.*]` or `END[.*]`. There will be a cli version with more options and functionality: [yogurt-cli](https://github.com/yocurt/yogurt-cli).

## Usage

There is a lib and a executable, which can be used to extract yaml content specified by e.g.: `ID[.*]`, `REF[.*]`, `ADD[.*]` or `END[.*]`.

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
 cat file.md | curt-extract -b "ID REF" > result.yaml
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
cat **/*.adoc | curt-extract -b ID | yaml json write - | less
```
