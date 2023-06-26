// window.rs

use glium::{self, glutin};

use crate::render::Renderer;
use glam::{Mat4};
use std::sync::{Arc, Mutex};
use crate::input::InputHandler;
use crate::camera::Camera;
use crate::event_manager::{EventManager, CameraEvent, EventHandler, Event};

pub struct InputHandlerWrapper(Arc<Mutex<InputHandler>>);

impl Clone for InputHandlerWrapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl EventHandler for InputHandlerWrapper {
    fn handle_event(&mut self, event: Arc<dyn Event + Send + Sync>) -> Vec<Arc<dyn Event + Send + Sync>> {
        let mut handler = self.0.lock().unwrap();
        handler.handle_event(event)
    }
}

pub fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Rusty Voxel Engine")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

    let cb = glutin::ContextBuilder::new().with_vsync(true);

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let camera = Camera::default();

    // Create a shared InputHandler
    let input_handler = Arc::new(Mutex::new(InputHandler::new(camera.clone())));

    // Wrap InputHandler in a InputHandlerWrapper
    let input_handler_wrapper = InputHandlerWrapper(input_handler.clone());

    // Create the EventManager and register the InputHandler as a handler
    let mut event_manager = EventManager::new();
    event_manager.register_handler::<CameraEvent>(input_handler_wrapper.clone());

    // Create the Renderer TODO: make this do something
    let renderer = Renderer::new(display.clone(), octree);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            glutin::event::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        event_manager.process_event(Arc::new(CameraEvent::KeyboardInput(input)));
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        // Render the scene
        renderer.render(
            Mat4::IDENTITY.to_cols_array_2d(),
            camera.view_matrix().to_cols_array_2d(),
            Mat4::perspective_rh_gl(45.0_f32.to_radians(), 1024.0 / 768.0, 0.1, 100.0).to_cols_array_2d(),
        );
              
    });
    
}