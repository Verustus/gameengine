enum WindowType {
    WindowedFullscreen,
    Fullscreen,
    Normal([u32; 2]),
    Borderless([u32; 2])
}

enum WindowConfig {
    Resizable(ResizeType),
    Movable(WindowMove),
}

enum ResizeSide {
    Left,
    Right,
    Top,
    Bottom
}

enum ResizeType {
    Specific(Vec<ResizeSide>),
    Vertical,
    Horizontal,
    All,
    None
}

enum WindowMove {
    OuterMargin([u32; 4]),
    InnerMargin([u32; 4]),
    Full,
    None
}

struct Window {
    border: bool,
    window: winit::window::Window
}

trait Window {
    pub fn create_window();
}

impl Window {
    pub fn new(window_state: WindowState, title: Option<&str>, border: Option<WindowType>, config: Option<Vec<WindowConfig>>) -> Self {
        let event_loop = winit::event_loop::EventLoopBuilder::new().build();
        let mut window_builder = glium::backend::glutin::SimpleWindowBuilder::new();
        window_builder = window_builder.with_title(title.map_or_else(|| "New window", |title| title));
        let (window, display) = window_builder.build(&event_loop);

        match window_state {
            WindowState::Fullscreen(fullscreen) => window.set_fullscreen(Some(Fullscreen::Borderless((None)))),
            WindowState::Size(size) => window.set_inner_size(winit::dpi::LogicalSize::new(size[0], size[1])),
        }

        Self {
            borderless: borderless.map_or_else(|| false, |borderless| borderless),
            window: window
        }
    }
}