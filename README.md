# Yogurt Yaml

![](https://github.com/yocurt/yogurt-yaml/workflows/Cargo%20Release/badge.svg)
![](https://github.com/yocurt/yogurt-yaml/workflows/Rust%20Build%20Pipeline/badge.svg?branch=master)
![](https://github.com/yocurt/yogurt-yaml/workflows/Trigger%20Docs%20Update/badge.svg)

This package allows the user to extract yaml from yogurt files, where yogurt files are bassically all files containing a `Identifier[.*]`-like syntax. This package only extracts `ID[.*]`, `REF[.*]`, `ADD[.*]` or `END[.*]`. There will be a cli version with more options and functionality: [yocurt-cli](https://github.com/yocurt/yogurt-cli).

## Usage

There is a lib and a executable, which can be used to extract yaml content specified by `ID[.*]`, `REF[.*]`, `ADD[.*]` or `END[.*]`.

``` md
<!-- file.md -->
# Title

Text in a file.

ID[NAME, attribute: value]

## Next Title

More text

REF[NAME, attribute: value, other_attribute: other_value]
```

returns:

``` yaml
# result.yaml
- {ID: NAME, attribute: value}
- {REF: NAME, attribute: value, other_attribute: other_value}
```

using:

``` bash
 cat file.md | curt-extract > result.yaml
```

## CLI Examples

### Simple Example

It is possible to extract yaml from any file.

``` bash
cat file | curt-extract
```

### More sophisticated Example

Other commandline tools can be used to extend the functionality.

``` bash
cat **/*.adoc | curt-extract | yaml json write - | less
```
