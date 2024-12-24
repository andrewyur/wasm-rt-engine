// use gloo_events::EventListener;
use web_sys::{js_sys, ResizeObserver, ResizeObserverEntry, ResizeObserverSize};
use crate::state::State;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;


pub struct App {
    state: Rc<RefCell<State>>,
}

impl App {
    pub async fn start() -> Self {
        let window = web_sys::window().expect("could not obtain window!");
        let document = window.document().expect("could not obtain result!");
        
        let state = Rc::new(RefCell::new(State::new(&document).await));

        state.borrow_mut().render().unwrap();

        // create resize observer for canvas
        {
            let state_clone = Rc::clone(&state);
            let callback: Closure<dyn FnMut(js_sys::Array)> = Closure::new(move |entries: js_sys::Array| {
                // arbitrarily chose content_box_size here
                let entry: ResizeObserverEntry = entries.at(0).unchecked_into();
                let size: ResizeObserverSize = entry.content_box_size().at(0).unchecked_into();

                let width = size.inline_size() as u32;
                let height = size.block_size() as u32;

                state_clone.borrow_mut().resize(width, height);
            });
            let resize_observer = ResizeObserver::new(callback.as_ref().unchecked_ref()).unwrap();

            // not sure whether this causes a memory leak or not, but i dont ever intend to remove this during the lifetime of the page
            callback.forget();
            resize_observer.observe(state.borrow().canvas());
        }


        // attach render function to requestAnimationFrame
        {
            fn request_animation_frame(f: &Closure<dyn FnMut()>) {
                web_sys::window().expect("no global window exists")
                    .request_animation_frame(f.as_ref().unchecked_ref())
                    .expect("should register `requestAnimationFrame` OK");
            }

            // this is horrible, the borrow checker will not let us call the closure from inside itsself, so we have to do this hokey bs
            let f = Rc::new(RefCell::new(None));
            let g = f.clone();

            let state_clone = Rc::clone(&state);

            *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                state_clone.borrow_mut().render().unwrap_throw();
                request_animation_frame(f.borrow().as_ref().unwrap())
            }) as Box<dyn FnMut()>));

            request_animation_frame(g.borrow().as_ref().unwrap());
            // render_closure.forget(); 
        }

        Self {
            state
        }
    }

}