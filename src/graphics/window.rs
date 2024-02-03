use std::{num::NonZeroU32, rc::Rc};
use raw_window_handle::HasRawWindowHandle;
use glium::glutin::{self, context::NotCurrentGlContext, display::GlDisplay};
use glutin::display::GetGlDisplay;
use winit::{event_loop::EventLoop, monitor::VideoMode, window::Fullscreen};

pub enum WindowMode {
    WindowedFullscreen,
    Fullscreen,
    Normal([u32; 2]),
    Borderless([u32; 2])
}

pub enum WindowRender {
    OpenGL(u32, u32),
    Vulkan(u32, u32, u32)
}

pub enum WindowConfig {
    Resizable(ResizeType),
    Movable(WindowMove),
    Resolution([f32; 2]),
    Version()
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
    OuterMargin([u32; 4]),
    InnerMargin([u32; 4]),
    Full,
    None
}

pub struct Window {
    pub window_mode: WindowMode,
    pub window: winit::window::Window,
    pub display: glium::backend::glutin::Display<glutin::surface::WindowSurface>,
    pub event_loop: EventLoop<()>
}

pub trait SimpleWindow {
    fn create_window();
    fn run();
}

impl Window {
    pub fn new(title: Option<&str>, window_mode: Option<WindowMode>, config: Option<Vec<WindowConfig>>) -> Window {
        let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
        let mut window_builder = winit::window::WindowBuilder::new();
        window_builder = window_builder
            .with_title(title.map_or_else(|| "New window", |title| title));
        let (window, display);

        let window_mode = window_mode.unwrap_or(WindowMode::Fullscreen);
        
        match window_mode {
            WindowMode::Fullscreen => {
                (window, display) = window_builder.simple_build(&event_loop);
                let size = [window.current_monitor().unwrap().size().width,
                                      window.current_monitor().unwrap().size().height];
                window.set_fullscreen(
                    Some(Fullscreen::Exclusive(Self::get_video_mode(&window, size))));
            },
            WindowMode::WindowedFullscreen => {
                (window, display) = window_builder.simple_build(&event_loop);
                window.set_fullscreen(Some(Fullscreen::Borderless(None)));
            },
            WindowMode::Normal(size) => {
                (window, display) = window_builder.with_inner_size_simple(size[0], size[1])
                                                  .simple_build(&event_loop);
            },
            WindowMode::Borderless(size) => {
                (window, display) = window_builder.with_inner_size_simple(size[0], size[1])
                                                  .with_transparent(true).with_decorations(false)
                                                  .simple_build(&event_loop);
            }
        }

        return Window {
            window_mode: window_mode,
            window: window,
            display: display,
            event_loop: event_loop
        };
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

        println!("selected: {}, {}", closest.size().width, closest.size().height);
        return closest;
    }
}

pub trait SimpleWindowBuilder {
    fn simple_build<T>(self, event_loop: &winit::event_loop::EventLoop<T>) -> (winit::window::Window, glium::Display<glutin::surface::WindowSurface>);
    fn with_inner_size_simple(self, width: u32, height: u32) -> Self;
    fn with_title_simple(self, title: &str) -> Self;
}

impl SimpleWindowBuilder for winit::window::WindowBuilder {
    fn simple_build<T>(self, event_loop: &winit::event_loop::EventLoop<T>) -> (winit::window::Window, glium::Display<glutin::surface::WindowSurface>) {
        let display_builder = glutin_winit::DisplayBuilder::new().with_window_builder(Some(self));
        let config_template_builder = glutin::config::ConfigTemplateBuilder::new();
        let (window, gl_config) = display_builder
            .build(&event_loop, config_template_builder, |mut configs| {
                configs.next().unwrap()
            })
            .unwrap();
        let window = window.unwrap();

        // Now we get the window size to use as the initial size of the Surface
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

    fn with_inner_size_simple(mut self, width: u32, height: u32) -> Self {
        self = self.with_inner_size(winit::dpi::PhysicalSize::new(width, height));
        return self;
    }

    fn with_title_simple(mut self, title: &str) -> Self {
        self = self.with_title(title);
        return self;
    }
}