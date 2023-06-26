//event_manager.rs

//todo implements bitflags to handle flags
use crate::input::InputHandler;
use std::any::{Any, TypeId};
use glam::{Quat, Vec3};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
//use glium::glutin::event::ElementState;

pub trait Event: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}
pub struct EventManager<H: EventHandler + Send + Sync + Clone + 'static> {
    handlers: HashMap<TypeId, Vec<HandlerWrapper<H>>>,
}
pub trait EventHandler: Send + Sync + Clone + 'static {
    fn handle_event(&mut self, event: Arc<dyn Event + Send + Sync>) -> Vec<Arc<dyn Event + Send + Sync>>;
}

impl<H: EventHandler + Send + Sync + Clone + 'static> Clone for HandlerWrapper<H> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
pub struct HandlerWrapper<H: EventHandler + 'static>(Arc<Mutex<H>>);

impl<H: EventHandler + Send + Sync + Clone + 'static> EventHandler for HandlerWrapper<H> {
    fn handle_event(&mut self, event: Arc<dyn Event + Send + Sync>) -> Vec<Arc<dyn Event + Send + Sync>> {
        let mut handler = self.0.lock().unwrap();
        handler.handle_event(event)
    }
}

impl<H: EventHandler + Send + Sync + Clone + 'static> Clone for EventManager<H> {
    fn clone(&self) -> Self {
        let mut new_handlers = HashMap::new();
        for (key, handlers) in self.handlers.iter() {
            let cloned_handlers = handlers
                .iter()
                .map(|handler| handler.clone())
                .collect::<Vec<_>>();
            new_handlers.insert(*key, cloned_handlers);
        }

        Self {
            handlers: new_handlers,
        }
    }
}
impl<H: EventHandler + Send + Sync + Clone + 'static> EventManager<H> {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    pub fn register_handler<E: Event + 'static>(&mut self, handler: H) {
        let type_id = TypeId::of::<E>();
        self.handlers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(HandlerWrapper::<H>(Arc::new(Mutex::new(handler))));
    }

    pub fn process_event(&mut self, event: Arc<dyn Event + Send + Sync>) {
        let type_id = event.as_any().type_id();
        let handlers_to_process = if let Some(handlers) = self.handlers.get(&type_id) {
            handlers
                .iter()
                .map(|handler| handler.0.clone())
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
    
        for handler_mutex in handlers_to_process {
            let event = event.clone();
            let mut handler = handler_mutex.lock().unwrap();
            handler.handle_event(event);
        }
    }    
}    

#[derive(Debug, Clone)]
pub enum CameraEvent {
    UpdatePosition(Vec3),
    UpdateRotation(Quat),
    //KeyboardInput(glium::glutin::event::KeyboardInput),
    UpdateZoom(f32),  
}

impl Event for CameraEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl EventHandler for InputHandler {
    fn handle_event(&mut self, event: Arc<dyn Event + Send + Sync>) -> Vec<Arc<dyn Event + Send + Sync>> {
        let mut new_events = Vec::new();
        if let Some(camera_event) = event.as_any().downcast_ref::<CameraEvent>() {
            let camera_event = camera_event.clone();

            // Clone the camera reference to avoid holding a lock
            let camera_clone = self.get_camera().clone();

            match camera_event {
                CameraEvent::UpdatePosition(delta) => {
                    // Lock the camera only for the duration of the update_position call
                    camera_clone.lock().unwrap().update_position(delta);
                    new_events.push(Arc::new(CameraEvent::UpdatePosition(delta)) as Arc<dyn Event + Send + Sync>);
                },
                CameraEvent::UpdateRotation(delta) => {
                    // Same here
                    camera_clone.lock().unwrap().update_rotation(delta);
                },
                CameraEvent::UpdateZoom(delta) => {
                    // And here
                    camera_clone.lock().unwrap().update_zoom(delta);
                    new_events.push(Arc::new(CameraEvent::UpdateZoom(delta)) as Arc<dyn Event + Send + Sync>);
                },
                /*CameraEvent::KeyboardInput(input) => {
                    // Handle the keyboard input here
                    if let Some(key) = input.virtual_keycode {
                        if input.state == ElementState::Pressed {
                            self.key_down(key);
                        } else {
                            self.key_up(key);
                        }

                        let position_delta = self.handle_keyboard_input(0.1); // Replace 0.1 with your delta time

                        if position_delta != Vec3::ZERO {
                            new_events.extend(self.update_camera(&position_delta, &Quat::IDENTITY));
                        }
                    }
                },*/
            }
        }
        new_events
    }
}
