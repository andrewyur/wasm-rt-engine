use wasm_bindgen::prelude::*;
use web_sys::console;
use winit::{
    application::ApplicationHandler,
    keyboard::{KeyCode, PhysicalKey},
    event::{WindowEvent, KeyEvent, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
    platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys}
};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

// impl App {
//     fn create_window(event_loop: &ActiveEventLoop) -> Window {
//         let window = web_sys::window().expect("get window failed!");
//         let document = window.document().expect("get document failed!");
//         let canvas = document.get_element_by_id("rust_canvas").expect("get canvas failed!");
//         let canvas: web_sys::HtmlCanvasElement = 
//             canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("convert into js type failed!");
//         event_loop.create_window(Window::default_attributes().with_canvas(Some(canvas))).expect("create window failed!")
//     }
// }

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = web_sys::window().expect("get window failed!");
        let document = window.document().expect("get document failed!");
        let canvas = document.get_element_by_id("rust_canvas").expect("get canvas failed!");
        let canvas: web_sys::HtmlCanvasElement = 
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().expect("convert into js type failed!");
        self.window = Some(
            event_loop.create_window(Window::default_attributes().with_canvas(Some(canvas))).expect("create window failed!")
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                console::log_1(&"working!".into());
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    
    console::log_1(&"testing".into());

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let app = App::default();
    let _ = event_loop.spawn_app(app);
}