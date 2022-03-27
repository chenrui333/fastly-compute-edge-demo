# fastly-compute-edge-demo <!-- omit in toc -->

[![main](https://github.com/chenrui333/fastly-compute-edge-demo/actions/workflows/main.yml/badge.svg)](https://github.com/chenrui333/fastly-compute-edge-demo/actions/workflows/main.yml)

- [build/publish](#buildpublish)
- [testing](#testing)

## build/publish

```
fastly compute init
fastly compute build
fastly compute pubilsh
```

## testing

```
$ curl -s -I  https://fastly-compute-edge-demo.edgecompute.app
HTTP/2 200
content-type: text/html; charset=utf-8
x-served-by: cache-ewr18160-EWR
date: Sun, 27 Mar 2022 05:07:42 GMT
```
