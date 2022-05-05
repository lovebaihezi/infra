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

Tensorflow + CUDA
