extern crate glium;
const PI: f32 = 3.41458;
use glium::{glutin, implement_vertex, Surface};

#[derive(Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);
fn main() {
    let events_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(1024., 768.));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();
    let radius = 0.5f32;
    let num_points = 4;
    //x = r cos θ//
    //y = r sin θ//
    let delta = (360. / num_points as f32) * (PI / 180.);
    let mut theta = 0f32;

    let shape: Vec<Vertex> = (0..num_points)
        .map(|cc| {
            theta = theta + delta * cc as f32;
            Vertex {
                position: [radius * theta.cos(), radius * theta.sin()],
            }
        })
        .collect();

    println!("{:?}", shape);
    // let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let mut indices = vec![0];

    for i in 1..num_points {
        indices.extend([i, i as u16 + 1, 0u16]);
    }
    indices.pop();
    // let indices: Vec<u16> = (0..=num_points).map(|i| i as u16).collect::<Vec<u16>>();
    //let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let index_buffer =
        glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TriangleFan, &indices)
            .unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
        "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
        "#;
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    events_loop.run(move |ev, _, control_flow| {
        let frm_from_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(frm_from_time);

        match ev {
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
    });
}
