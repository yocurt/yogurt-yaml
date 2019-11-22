# Yogurt Yaml

![](https://github.com/yocurt/yogurt-yaml/workflows/Cargo%20Release/badge.svg)
![](https://github.com/yocurt/yogurt-yaml/workflows/Rust%20Build%20Pipeline/badge.svg?branch=master)
![](https://github.com/yocurt/yogurt-yaml/workflows/Trigger%20Docs%20Update/badge.svg)

Get yaml from yogurt files

## Usage

There is a lib and a executable, which can be used.

## CLI Examples

### Simple Example

``` bash
cat file | curt -p
```

### More sophisticated Example

``` bash
cat **/*.adoc | curt -p | yaml json write - | less
```
