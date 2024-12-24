use web_sys::{console, Document, HtmlCanvasElement };
use wasm_bindgen::JsCast;

pub struct State<'a> {
    canvas: &'a web_sys::HtmlCanvasElement,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration
}


impl<'a> State<'a> {
    pub async fn init(document: &Document) {

        let canvas = document.get_element_by_id("rust_canvas")
            .expect("could not find canvas element!");
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()
            .expect("conversion to HtmlCanvasElement failed!");

        // let size = canvas. ...

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        });

        let surface_target = wgpu::SurfaceTarget::Canvas(canvas);
        let surface = instance.create_surface(surface_target).expect("create surface failed!");

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        console::log_1(&format!("Selected adapter: {:?}", adapter.get_info()).into());

    }
}