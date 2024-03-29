use std::time::Instant;
use glium::Surface;
use crate::graphics::{types::{RenderVertex, Rotate}, window::{Window, WindowMode}};

implement_vertex!(RenderVertex, position, texture_coords);

#[derive(Clone, Copy)]
struct Material {
    color_override: [f32; 3]
}

implement_uniform_block!(Material, color_override);

fn get_position(vertices: &[RenderVertex]) -> [f32; 3] {
    let (sum_x, sum_y, sum_z) = vertices.iter().fold((0.0, 0.0, 0.0), |(acc_x, acc_y, acc_z), vertex| {
        (acc_x + vertex.position[0], acc_y + vertex.position[1], acc_z + vertex.position[2])
    });

    let num_vertices = vertices.len() as f32;

    [sum_x / num_vertices, sum_y / num_vertices, sum_z / num_vertices]
}

pub fn start() {
    let window: Window = Window::new(Some("Test"), Some(WindowMode::Borderless([400, 400])), None);
    let (display, window, event_loop) = (window.display, window.window, window.event_loop);

    let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::load(std::io::Cursor::new(std::fs::read(inner_path!("img/opengl_logo.png")).unwrap()), image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let image2 = image::load(std::io::Cursor::new(std::fs::read(inner_path!("img/pngegg.png")).unwrap()), image::ImageFormat::Png).unwrap().to_rgba8();
    let image2_dimensions = image2.dimensions();
    let image2 = glium::texture::RawImage2d::from_raw_rgba_reversed(&image2.into_raw(), image2_dimensions);

    let texture2 = glium::texture::Texture2d::new(&display, image2).unwrap();

    let mut shape = vec![
        RenderVertex { position: [-0.5, -0.5, 0.0], texture_coords: [0.0, 0.0] },
        RenderVertex { position: [-0.5,  0.5, 0.0], texture_coords: [0.0, 1.0] },
        RenderVertex { position: [ 0.5,  0.5, 0.0], texture_coords: [1.0, 1.0] },
        RenderVertex { position: [ 0.5, -0.5, 0.0], texture_coords: [1.0, 0.0] },
    ];
    let offset = [1.5, 0.0];
    let mut shape2 = vec![
        RenderVertex { position: [-0.5+offset[0], -0.5+offset[1], 0.0], texture_coords: [0.0, 0.0] },
        RenderVertex { position: [-0.5+offset[0],  0.5+offset[1], 0.0], texture_coords: [0.0, 1.0] },
        RenderVertex { position: [ 0.5+offset[0],  0.5+offset[1], 0.0], texture_coords: [1.0, 1.0] },
        RenderVertex { position: [ 0.5+offset[0], -0.5+offset[1], 0.0], texture_coords: [1.0, 0.0] },
    ];
    let offset2 = [0.0, 0.5];
    let mut shape3 = vec![
        RenderVertex { position: [-0.5+offset2[0], -0.5+offset2[1], 0.0], texture_coords: [0.0, 0.0] },
        RenderVertex { position: [-0.5+offset2[0],  0.5+offset2[1], 0.0], texture_coords: [0.0, 1.0] },
        RenderVertex { position: [ 0.5+offset2[0],  0.5+offset2[1], 0.0], texture_coords: [1.0, 1.0] },
        RenderVertex { position: [ 0.5+offset2[0], -0.5+offset2[1], 0.0], texture_coords: [1.0, 0.0] },
    ];
    const U32_INDICES: [u16; 6] = [0, 1, 2, 0, 3, 2];

    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &U32_INDICES).unwrap();

    let material = glium::uniforms::UniformBuffer::new(&display, Material { color_override: [1f32, 1f32, 1f32] }).unwrap();
    let material2 = glium::uniforms::UniformBuffer::new(&display, Material { color_override: [1f32, 0f32, 0f32] }).unwrap();
    let material3 = glium::uniforms::UniformBuffer::new(&display, Material { color_override: [-2f32, 0f32, 0f32] }).unwrap();
    
    let vertex_shader = std::fs::read_to_string(inner_path!("shaders/simple.vs")).unwrap();
    let fragment_shader = std::fs::read_to_string(inner_path!("shaders/simple.fs")).unwrap();

    let program = glium::Program::from_source(&display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();
    
    let mut last_frame_update = Instant::now();
    let _ = event_loop.run(move |event, control_flow| {
        match event {
            winit::event::Event::AboutToWait => window.request_redraw(),
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => control_flow.exit(),
                winit::event::WindowEvent::Resized(window_size) => { display.resize(window_size.into()); },
                winit::event::WindowEvent::RedrawRequested => {
                    let rotation = 0.0000005 * last_frame_update.elapsed().as_micros() as f32;
                    last_frame_update = Instant::now();

                    shape.rotate([0.0, rotation, 0.0].into(), get_position(&shape).into());
                    shape2.rotate([0.0, rotation, 0.0].into(), [0.0, 0.0, 0.0].into());
                    shape3.rotate([rotation, 0.0, 0.0].into(), get_position(&shape3).into());
                    
                    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
                    let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();
                    let vertex_buffer3 = glium::VertexBuffer::new(&display, &shape3).unwrap();

                    let uniforms = uniform! {
                        texture_2d: &texture,
                        Material: &material
                    };

                    let uniforms_2 = uniform! {
                        texture_2d: &texture,
                        Material: &material2
                    };

                    let uniforms_3 = uniform! {
                        texture_2d: &texture2,
                        Material: &material3
                    };

                    let mut frame = display.draw();

                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        blend: glium::draw_parameters::Blend::alpha_blending(),
                        .. Default::default()
                    };

                    frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                    let _ = frame.draw(&vertex_buffer, &indices, &program, &uniforms, &params);
                    let _ = frame.draw(&vertex_buffer2, &indices, &program, &uniforms_2, &params);
                    let _ = frame.draw(&vertex_buffer3, &indices, &program, &uniforms_3, &params);
                    frame.finish().unwrap();
                },
                _ => ()
            }
            _ => ()
        }
    });
}