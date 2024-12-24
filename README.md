# wasm-rt-engine

A real time raytracing engine built with WASM, targeting the browser

## Compatibility

- Browser:
  - works on chrome and edge, works on safari with the webgpu feature enabled
  - does not work on firefox (firefox nightly may work)

## TODO

- [x] set up rust/wasm profile in vscode & nix shell
- [x] set up a crate, window manager & wasm target
- [x] set up a static web server via docker
- [x] get wasm-pack & bindgen working
- [x] winit working on wasm
- [x] dev server without docker
- [x] remove winit
- [ ] make basic wgpu application
- [ ] integrate egui and egui_wgpu
- [ ] follow the rest of the wgpu book
- [ ] camera controls with pointerlock api and gloo
- [ ] slim everything down, remove unused dependencies (try wee_alloc?)

## Dev log

- lifetimes suck!! trying to wrangle lifetimes of references between two different structs is causing me immense pain and frustration
- the webgpu api is not available on default firefox unfortunately, so i could either switch to webgl or just use safari
- using winit is creating a lot of issues for me and i realized i dont really need it to make an application catered only to the browser. i will be using web-sys to directly interface with the canvas from now on

## Resources

- <https://rustwasm.github.io/book/>
- <https://rustwasm.github.io/docs/wasm-bindgen/>
- <https://github.com/kwillemsen/wasm_winit_wgpu/tree/main>
- <https://sotrh.github.io/learn-wgpu/#special-thanks-to-these-patrons>
- <https://raytracing.github.io/>
- <https://blog.demofox.org/2020/05/25/casual-shadertoy-path-tracing-1-basic-camera-diffuse-emissive/>
- <https://docs.rs/wgpu/latest/wgpu/struct.Features.html>
- <https://docs.rs/wgpu/latest/wgpu/struct.Limits.html>
