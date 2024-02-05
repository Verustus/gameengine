use std::num::NonZeroU32;
use raw_window_handle::HasRawWindowHandle;
use glium::glutin::{self, context::NotCurrentGlContext, display::GlDisplay};
use glutin::display::GetGlDisplay;

use super::window::{AnyWindow, Window, WindowBuilder};

pub struct VulkanWindowBuilder {
    pub version: [u32; 3],
    pub winit_builder: winit::window::WindowBuilder
}

impl WindowBuilder for VulkanWindowBuilder {
    fn build<T>(&mut self, event_loop: &winit::event_loop::EventLoop<T>) -> (winit::window::Window, glium::Display<glium::glutin::surface::WindowSurface>) {
        let display_builder = glutin_winit::DisplayBuilder::new().with_window_builder(Some(self.winit_builder));
        let config_template_builder = glutin::config::ConfigTemplateBuilder::new();
        let (window, gl_config) = display_builder
            .build(&event_loop, config_template_builder, |mut configs| {
                configs.next().unwrap()
            })
            .unwrap();
        let window = window.unwrap();

        let (width, height): (u32, u32) = window.inner_size().into();
        let attrs = glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new().build(
            window.raw_window_handle(),
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };
        let context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::OpenGl(
                Some(glutin::context::Version::new(4, 6))
            ))
            .build(Some(window.raw_window_handle()));
        let current_context = Some(unsafe {
            gl_config.display().create_context(&gl_config, &context_attributes).expect("failed to create context")
        }).unwrap().make_current(&surface).unwrap();
        let display = glium::Display::from_context_surface(current_context, surface).unwrap();

        return (window, display);
    }

    fn get_winit(&self) -> winit::window::WindowBuilder { return self.winit_builder; }

    fn set_winit(&mut self, winit_builder: winit::window::WindowBuilder) { self.winit_builder = winit_builder; }
}

pub struct VulkanWindow {
    pub window: Window
}

impl AnyWindow for VulkanWindow {
    fn start(&self) {
        let display = self.window.display;
    }
}