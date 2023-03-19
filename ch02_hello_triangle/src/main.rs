use glium::implement_vertex;

extern crate glium;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

fn main() {
    use glium::{glutin, Surface};

    let events_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(1024., 768.))
        .with_title("winit window");

    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    implement_vertex!(Vertex, position);

    let v1 = Vertex {
        position: [-0.5, -0.5],
    };
    let v2 = Vertex {
        position: [0.0, 0.5],
    };
    let v3 = Vertex {
        position: [0.5, -0.25],
    };

    let shape = vec![v1, v2, v3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
        let nxt_frm_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(nxt_frm_time);

        //});

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
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
