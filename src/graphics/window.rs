use std::rc::Rc;
use glium::glutin;
use winit::{event_loop::EventLoop, monitor::VideoMode, window::Fullscreen};

use crate::graphics::vulkan::VulkanWindowBuilder;

use super::{opengl::{OpenglWindow, OpenglWindowBuilder}, vulkan::VulkanWindow};

pub enum WindowMode {
    WindowedFullscreen,
    Fullscreen,
    Normal([u32; 2]),
    Borderless([u32; 2])
}

#[derive(Copy, Clone)]
pub enum Version {
    OpenGL(u8, u8),
    Vulkan(u32, u32, u32)
}

pub enum ResizeSide {
    Left,
    Right,
    Top,
    Bottom
}

pub enum ResizeType {
    Specific(Vec<ResizeSide>),
    Vertical,
    Horizontal,
    All,
    None
}

pub enum WindowMove {
    CenterBox([u32; 2]), /// width and height
    Box([u32; 2], [u32; 2]), /// position x and y, width and height
    SideMargin([u32; 4]), ///  left, right, top, bottom
    Full,
    None
}

struct WindowConfig {
    title: String,
    window_mode: WindowMode,
    resizable: ResizeType,
    movable: WindowMove,
    resolution: [u32; 2],
    version: Version
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        return WindowConfig {
            title: "New Window".to_string(),
            window_mode: WindowMode::Normal([500, 600]),
            resizable: ResizeType::All,
            movable: WindowMove::SideMargin([0, 0, 0, 50]),
            resolution: [1920, 1080],
            version: Version::OpenGL(4, 6)
        };
    }
}

pub trait WindowBuilder {
    fn build<T>(&mut self, event_loop: &winit::event_loop::EventLoop<T>) -> (winit::window::Window, glium::Display<glutin::surface::WindowSurface>);
    fn get_winit(&self) -> winit::window::WindowBuilder;
    fn set_winit(&mut self, winit_builder: winit::window::WindowBuilder);
}

pub trait AnyWindow {
    fn start(&self);
    fn render(&self);
}

pub struct Window {
    pub window_mode: WindowMode,
    pub winit_window: Rc<winit::window::Window>,
    pub display: glium::backend::glutin::Display<glutin::surface::WindowSurface>
}

enum AnyWindowBuilder {
    OpenGL(OpenglWindowBuilder),
    Vulkan(VulkanWindowBuilder)
}

impl AnyWindowBuilder {
    fn build<T>(&mut self, event_loop: &winit::event_loop::EventLoop<T>) -> (winit::window::Window, glium::Display<glutin::surface::WindowSurface>) {
        match self {
            AnyWindowBuilder::OpenGL(window_builder) => window_builder.build(event_loop),
            AnyWindowBuilder::Vulkan(window_builder) => window_builder.build(event_loop)
        }
    }

    fn get_winit(&self) -> winit::window::WindowBuilder {
        match self {
            AnyWindowBuilder::OpenGL(window_builder) => window_builder.get_winit(),
            AnyWindowBuilder::Vulkan(window_builder) => window_builder.get_winit()
        }
    }

    fn set_winit(&mut self, winit_builder: winit::window::WindowBuilder) {
        match self {
            AnyWindowBuilder::OpenGL(window_builder) => window_builder.set_winit(winit_builder),
            AnyWindowBuilder::Vulkan(window_builder) => window_builder.set_winit(winit_builder)
        }
    }
}


fn create_window_builder(version: Version) -> AnyWindowBuilder {
    match version {
        Version::OpenGL(major, minor) => AnyWindowBuilder::OpenGL(OpenglWindowBuilder {
            winit_builder: winit::window::WindowBuilder::new(),
            version: [major, minor]
        }),
        Version::Vulkan(major, minor, patch) => AnyWindowBuilder::Vulkan(VulkanWindowBuilder {
            winit_builder: winit::window::WindowBuilder::new(),
            version: [major, minor, patch]
        })
    }
}

impl Window {
    pub fn new(config: Option<WindowConfig>) -> Box<dyn AnyWindow> {
        let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
        let config = config.unwrap_or_default();

        let mut builder: AnyWindowBuilder = create_window_builder(config.version);

        let mut winit_builder = builder.get_winit();
        winit_builder = winit_builder.with_title(config.title);
        let (winit_window, display);

        match config.window_mode {
            WindowMode::Fullscreen => {
                (winit_window, display) = builder.build(&event_loop);
                let size = [winit_window.current_monitor().unwrap().size().width,
                winit_window.current_monitor().unwrap().size().height];
                winit_window.set_fullscreen(
                    Some(Fullscreen::Exclusive(Self::get_video_mode(&winit_window, size))));
            },
            WindowMode::WindowedFullscreen => {
                (winit_window, display) = builder.build(&event_loop);
                winit_window.set_fullscreen(Some(Fullscreen::Borderless(None)));
            },
            WindowMode::Normal(size) => {
                builder.set_winit(winit_builder.with_inner_size(winit::dpi::PhysicalSize::new(size[0], size[1])));
                (winit_window, display) = builder.build(&event_loop);
            },
            WindowMode::Borderless(size) => {
                builder.set_winit(winit_builder.with_transparent(true)
                                                              .with_inner_size(winit::dpi::PhysicalSize::new(size[0], size[1]))
                                                              .with_decorations(false)
                                                              .with_resizable(true));
                (winit_window, display) = builder.build(&event_loop);
            }
        }

        let window = Window {
            window_mode: config.window_mode,
            winit_window: Rc::new(winit_window),
            display: display
        };

        let any_window: Box<dyn AnyWindow> = match config.version {
            Version::OpenGL(_, _) => Box::new(OpenglWindow { window: window }),
            Version::Vulkan(_, _, _) => Box::new(VulkanWindow { window: window })
        };

        event_loop.run(move |event, control_flow| {
            match event {
                winit::event::Event::AboutToWait => winit_window.request_redraw(),
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => control_flow.exit(),
                    winit::event::WindowEvent::Resized(window_size) => { display.resize(window_size.into()); },
                    winit::event::WindowEvent::RedrawRequested => {
                        any_window.render();
                    },
                    _ => ()
                }
                _ => ()
            }
        });

        any_window.start();

        return window;
    }

    fn event_loop() {

    }

    fn get_video_mode(window: &winit::window::Window, size: [u32; 2]) -> VideoMode {
        let size = size[0]*size[1];
        let video_modes: Rc<Vec<VideoMode>> = Rc::new(window.current_monitor().unwrap().video_modes().collect());
        /*
        let mut max_distance = 0;
        video_modes.for_each(|mode| {
            let current = mode.size().width*mode.size().height;
            if current > max_distance { max_distance = current };
        });
        max_distance = max_distance-size;
        */
        let mut closest = video_modes.last().unwrap().clone();
        let mut closest_distance: u32 = (i64::from(closest.size().width)*i64::from(closest.size().height)-i64::from(size)).abs().try_into().unwrap();

        for mode in video_modes.to_vec() {
            let current_distance = (i64::from(mode.size().width)*i64::from(mode.size().height)-i64::from(size)).abs().try_into().unwrap();
            println!("{}, {}", mode.size().width, mode.size().height);
            if closest_distance > current_distance {
                closest = mode.clone();
                closest_distance = current_distance;
            }
        }

        return closest;
    }
}