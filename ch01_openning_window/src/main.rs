//use glium::backend::glutin;

extern crate glium;


fn main() {
    use glium::{ glutin, Surface };
    
    let events_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    events_loop.run(move |ev, _ , control_flow| {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        let frm_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(frm_time);


        if let glutin::event::Event::WindowEvent { event, .. } = ev {
            if event == glutin::event::WindowEvent::CloseRequested {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
        }
    });

    // let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    // let wb = glium::glutin::window::WindowBuilder::new()
    //     .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024., 768.))
    //     .with_title("winit window");
    // let cb = glium::glutin::ContextBuilder::new();
    // let display = glium::Display::new(wb, cb, &events_loop).unwrap();
    //
    // events_loop.run(move | ev, _ , control_flow |{
    //     let mut target = display.draw();
    //     target.clear_color(0.0, 0.0, 1.0, 1.0);
    //     target.finish().unwrap();
    //
    //     let next_frame_time = std::time::Instant::now() + 
    //         std::time::Duration::from_nanos(16_666_667);
    //
    //     *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    //
    //     if let glium::glutin::event::Event::WindowEvent { event, .. } = ev {
    //         if event == glium::glutin::event::WindowEvent::CloseRequested {
    //             *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
    //         }
    //     }
    // });
}
