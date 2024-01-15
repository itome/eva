use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

mod engine;

use crate::engine::shell::Shell;

fn main() {
    let el = EventLoop::new().expect("Failed to create event loop");
    let mut shell = Shell::new(&el);

    shell
        .task_runners
        .raster_task_runner
        .post_task(Box::new(|| {
            shell.rasterizer.draw_surface();
        }))
        .expect("Failed to create rasterizer");

    el.run(move |event, window_target| {
        let mut draw_frame = false;

        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => {
                    window_target.exit();
                    return;
                }
                WindowEvent::MouseInput { .. } => {
                    shell.gl_view.window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    draw_frame = true;
                }
                _ => (),
            }
        }

        if draw_frame {
            shell
                .task_runners
                .raster_task_runner
                .post_task(Box::new(|| {
                    shell.rasterizer.draw_surface();
                    shell.gl_view.swap_buffers().unwrap();
                }))
                .expect("Failed to draw frame");
        }
    })
    .expect("run() failed");
}
