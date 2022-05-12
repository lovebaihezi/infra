# My Infra

[toc]

## target

1. app
2. app service
3. deploy service
   a. compile
   b. test
   c. deploy
4. health check
5. data analyze with Deep Learning

## app

### Core

language: Rust
audio: WebRTC
auth: oauth2
web request: GraphQL

#### Core Export lib

1. WASM
2. C-ABI

### Cross

iced

### website

language: Typescript
framework: React
UI: taildwind css + Mui
state manager: swr + recoil
dev system: vite(now) => zigdev(future)
build system: esbuild(now) => rustbuild(future)
test system: vitest + playwright + react-testing-library
deploy system: unknown

### phone

react-native react-native-skia

#### Android

#### IOS

### tui

tui-rs

## app service

### Actix

oauth2, app feature

### lightium

oauth2, Deep Learning, protobuf,

## data analyze with Deep Learning

mlpack, tensorflow, 

## Thanks to:
@article{ensmallen_JMLR_2021,
author  = {Ryan R. Curtin and Marcus Edel and Rahul Ganesh Prabhu and Suryoday Basak and Zhihao Lou and Conrad Sanderson},
title   = {The ensmallen library for flexible numerical optimization},
journal = {Journal of Machine Learning Research},
year    = {2021},
volume  = {22},
number  = {166},
pages   = {1--6},
url     = {http://jmlr.org/papers/v22/20-416.html}
}
