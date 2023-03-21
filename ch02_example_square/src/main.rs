use std::fs;

use glium::{glutin, implement_vertex, Surface};
extern crate glium;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

fn main() {
    let mut events_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(1024., 768.))
        .with_title("winit window");
    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    implement_vertex!(Vertex, position); // println!("{}", frag_s);
    let v1 = Vertex {
        position: [-0.5, 0.5],
    };
    let v2 = Vertex {
        position: [0.5, 0.5],
    };
    let v3 = Vertex {
        position: [0.5, -0.5],
    };
    let v4 = Vertex {
        position: [-0.5, -0.5],
    };

    let shape = vec![v1, v2, v3, v4];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    //let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &[0u16, 1, 2, 0, 2, 3],
    )
    .unwrap();
    //
    let vs = fs::read_to_string("shaders/vertex_shader.glsl").expect("Failed to read file.");
    let frag_s =
        fs::read_to_string("shaders/fragment_shader.glsl").expect("Failed to read fragment shader");

    let program =
        glium::Program::from_source(&display, vs.as_str(), frag_s.as_str(), None).unwrap();

    events_loop.run(move |events, _, control_flow| {
        let time_nxt_frm = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(time_nxt_frm);
        match events {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
        // });
    });
}
