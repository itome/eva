use std::thread;

pub struct TaskRunners {
    pub raster_task_runner: TaskRunner,
    pub ui_task_runner: TaskRunner,
}

impl TaskRunners {
    pub fn new() -> Self {
        TaskRunners {
            raster_task_runner: TaskRunner::new(),
            ui_task_runner: TaskRunner::new(),
        }
    }
}

pub struct TaskRunner {
    tx: std::sync::mpsc::Sender<Box<dyn FnOnce() + Send>>,
}

impl TaskRunner {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<Box<dyn FnOnce() + Send>>();

        thread::spawn(move || {
            while let Ok(task) = rx.recv() {
                task();
            }
        });

        TaskRunner { tx }
    }

    pub fn post_task(
        &mut self,
        task: Box<dyn FnOnce() + Send>,
    ) -> Result<(), std::sync::mpsc::SendError<Box<dyn FnOnce() + Send>>> {
        self.tx.send(task)
    }
}
