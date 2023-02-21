use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

// Public interface

#[derive(Debug)]
pub struct TaskDefinition {
    init: fn() -> Result<(), String>,
    run: fn() -> Result<(), String>,
    terminate: fn() -> Result<(), String>,
}

type TaskMutex = Arc<Mutex<bool>>;
type TaskHandle = JoinHandle<()>;

pub struct RunningTask {
    handle_: TaskHandle,
    name_: String,
    stopped_requested_: TaskMutex,
}

impl RunningTask {
    fn new(handle: TaskHandle, name: String, stopped_requested: TaskMutex) -> RunningTask {
        RunningTask {
            handle_: handle,
            name_: name,
            stopped_requested_: stopped_requested,
        }
    }

    pub fn stop(self) {
        if let Ok(mut stopped_requested) = self.stopped_requested_.lock() {
            *stopped_requested = true;
            self.handle_
                .join()
                .expect((String::from("Fail to join task") + &self.name_).as_str());
        }
    }
}

#[derive(Debug)]
pub struct Task {
    timeout_: Duration,
    name_: String,
    stopped_requested_: TaskMutex,
    task_definition_: TaskDefinition,
}

impl Task {
    fn new(dur: Duration, name: String, def: TaskDefinition) -> Task {
        Task {
            timeout_: dur,
            name_: name,
            stopped_requested_: Arc::new(Mutex::new(false)),
            task_definition_: def,
        }
    }

    pub fn start(duration: Duration, name: String, def: TaskDefinition) -> RunningTask {
        let task_name = name.clone();
        let new_task = Task::new(duration, name, def);
        let stopped_requested_attr = new_task.stopped_requested_.clone();
        let handle = thread::spawn(move || new_task.thread_run());
        RunningTask {
            handle_: handle,
            name_: task_name,
            stopped_requested_: stopped_requested_attr,
        }
    }

    fn thread_run(self) {
        let runner = self.task_definition_;

        // custom init task
        if let Err(error) = (runner.init)() {
            println!("Fail to init {}: {}", &self.name_, &error);
            return;
        }

        loop {
            // custom run
            if let Err(error) = (runner.run)() {
                println!("Fail to run {}: {}", &self.name_, &error);
                return;
            }

            thread::sleep(self.timeout_);
            {
                let stopped_requested = self.stopped_requested_.lock().unwrap();
                if *stopped_requested {
                    break;
                }
            }
        }

        // custom terminate
        if let Err(error) = (runner.terminate)() {
            println!("Fail to run {}: {}", &self.name_, &error);
            return;
        };
    }
}
