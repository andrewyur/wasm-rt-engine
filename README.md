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
- [x] make an app struct
- [ ] restart with bevy

## Dev log

- lifetimes suck!! trying to wrangle lifetimes of references between two different structs is causing me immense pain and frustration
- the webgpu api is not available on default firefox unfortunately, so i could either switch to webgl or just use safari
- using winit is creating a lot of issues for me and i realized i dont really need it to make an application catered only to the browser. i will be using web-sys to directly interface with the canvas from now on
- after almost getting the render pipeline working again and failing, i have decided that working on creating a raw wgpu implementation of a renderer has caused me to lose sight of the real goal of this project, which was to fiddle around with raytracing. i will be restarting again, with the bevy game engine and jumping straight into the shader coding and raytracing! exciting times
- nevermind, i was able to get a minimal example working, and might be able to abstract it to the point where it is easily abstractible. i think that building everything from scratch will make it a much cooler project, and this is why wgpu is getting another chance.

## Resources

- <https://rustwasm.github.io/book/>
- <https://rustwasm.github.io/docs/wasm-bindgen/>
- <https://github.com/kwillemsen/wasm_winit_wgpu/tree/main>
- <https://sotrh.github.io/learn-wgpu/#special-thanks-to-these-patrons>
- <https://raytracing.github.io/>
- <https://blog.demofox.org/2020/05/25/casual-shadertoy-path-tracing-1-basic-camera-diffuse-emissive/>
- <https://docs.rs/wgpu/latest/wgpu/struct.Features.html>
- <https://docs.rs/wgpu/latest/wgpu/struct.Limits.html>
- <https://github.com/GuillaumeGomez/egui/blob/3777b8d2741f298eaa1409dc08062902f7541990/crates/eframe/src/web/web_painter_wgpu.rs#L169>
