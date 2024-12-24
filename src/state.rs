use web_sys::{console, Document, HtmlCanvasElement };
use wasm_bindgen::JsCast;

pub struct State {
    canvas: web_sys::HtmlCanvasElement,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    width: u32,
    height: u32,
}


impl State {
    pub async fn new(document: &Document) -> Self{

        let element = document.get_element_by_id("rust_canvas")
            .expect("could not find canvas element!");
        let canvas: HtmlCanvasElement = element.dyn_into::<HtmlCanvasElement>()
            .expect("conversion to HtmlCanvasElement failed!");

        let width = canvas.client_width() as u32;
        let height = canvas.client_height() as u32;

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        });

        // i assume cloning the canvas here is fine, because this is only a reference to the node and not the node itsself
        let surface_target = wgpu::SurfaceTarget::Canvas(canvas.clone());
        let surface = instance.create_surface(surface_target).expect("create surface failed!");

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.expect("could not find an suitable adapter! maybe you dont have a dedicated GPU?");

        console::log_1(&format!("Selected adapter: {:?}", adapter.get_info()).into());
        console::log_1(&format!("adapter features: {:?}", adapter.features()).into());
        
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None, // Trace path
        ).await.unwrap();
        
        console::log_1(&format!("device features: {:?}", device.features()).into());

        console::log_1(&format!("device capabilities: {:?}", surface.get_capabilities(&adapter)).into());
        
        let surface_capabilities = surface.get_capabilities(&adapter);

        let texture_format = surface_capabilities.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or_else(|| {
                console::log_1(&"could not find srgb texture, falling back!".into());
                surface_capabilities.formats[0]
            });

        console::log_1(&format!("texture format: {:?}", texture_format).into());

        let config = wgpu::SurfaceConfiguration {
            format: texture_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            width,
            height,
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Self {
            canvas,
            surface,
            device,
            queue,
            config,
            width, 
            height
        }
    }

    pub fn canvas(&self) -> &HtmlCanvasElement{
        &self.canvas
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        console::log_1(&"resizing!".into());

        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.3,
                            g: 0.2,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}