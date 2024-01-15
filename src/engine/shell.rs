use winit::event_loop::EventLoop;

use super::{gl_view::GLView, rasterizer::Rasterizer, task_runner::TaskRunners};

pub struct Shell {
    pub task_runners: TaskRunners,
    pub gl_view: GLView,
    pub rasterizer: Rasterizer,
}

impl Shell {
    pub fn new(el: &EventLoop<()>) -> Self {
        let width = 800.0;
        let height = 600.0;
        let task_runners = TaskRunners::new();
        let gl_view = GLView::new(el, width, height);
        let rasterizer = Rasterizer::new(&gl_view.config, width as i32, height as i32);

        Shell {
            task_runners,
            gl_view,
            rasterizer,
        }
    }
}
