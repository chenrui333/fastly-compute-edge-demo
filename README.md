# fastly-compute-edge-demo <!-- omit in toc -->

[![main](https://github.com/chenrui333/fastly-compute-edge-demo/actions/workflows/main.yml/badge.svg)](https://github.com/chenrui333/fastly-compute-edge-demo/actions/workflows/main.yml)
[![hurl](https://github.com/chenrui333/fastly-compute-edge-demo/actions/workflows/hurl.yml/badge.svg)](https://github.com/chenrui333/fastly-compute-edge-demo/actions/workflows/hurl.yml)

The goal for this repo is to showcase the fastly-compute-edge workflow:

- Minimal skeleton template which has httpbin backend
- Include fastly commands to demo the build/deploy as well as local testing
- CI/CD pipeline for build/deploy compute@edge package
- CI/CD pipeline for build/deploy terraform service

- [build/publish](#buildpublish)
- [add backend](#add-backend)
- [testing](#testing)
- [local testing](#local-testing)
- [references](#references)
  - [template repos](#template-repos)

## build/publish

```
fastly compute init
fastly compute build
fastly compute pubilsh
```

## add backend

```
$ fastly backend create --version=latest --name=httpbin --address=httpbin.com --port=443 --autoclone

$ fastly backend list --version=latest
SERVICE                 VERSION  NAME        ADDRESS      PORT  COMMENT
3P8d99kmQc6GK2lpLPvtmk  6        httpbin     httpbin.com  443
3P8d99kmQc6GK2lpLPvtmk  6        originless  127.0.0.1    80
```

## testing

```
$ curl -s -I  https://fastly-compute-edge-demo.edgecompute.app
HTTP/2 200
content-type: text/html; charset=utf-8
x-served-by: cache-ewr18160-EWR
date: Sun, 27 Mar 2022 05:07:42 GMT
```

## local testing

```
$ fastly compute serve

$ curl -I http://127.0.0.1:7676/status/500
HTTP/1.1 500 Internal Server Error
```

## references

### template repos

- [fastly/compute-starter-kit-rust-default](https://github.com/fastly/compute-starter-kit-rust-default)
- [fastly/compute-rust-auth](https://github.com/fastly/compute-rust-auth)
- [fastly/compute-starter-kit-rust-beacon-termination](https://github.com/fastly/compute-starter-kit-rust-beacon-termination)
- [fastly/compute-starter-kit-rust-empty](https://github.com/fastly/compute-starter-kit-rust-empty)
- [fastly/compute-starter-kit-rust-static-content](https://github.com/fastly/compute-starter-kit-rust-static-content)
