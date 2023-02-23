use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

// Public interface

/// Task definition trait
pub trait TaskDefinition: 'static {
    fn init(&mut self) -> Result<(), String>;
    fn run(&mut self) -> Result<(), String>;
    fn terminate(&mut self);
}

type TaskHandle = JoinHandle<()>;
type SafeSyncPtr<T> = Arc<Mutex<T>>;

pub fn build_safe_sync_ptr<T>(item: T) -> SafeSyncPtr<T> {
    Arc::new(Mutex::new(item))
}

/// Running task, enabling the user to control the scheduling of the task
pub struct RunningTask {
    handle_: TaskHandle,
    name_: String,
    stopped_requested_: SafeSyncPtr<bool>,
}

impl RunningTask {
    pub fn stop(self) {
        if let Ok(mut stopped_requested) = self.stopped_requested_.lock() {
            *stopped_requested = true;
        } else {
            // Cannot take the lock of the mutex: do nothing
            return;
        }
        self.handle_
            .join()
            .expect((String::from("Fail to join task") + &self.name_).as_str());
    }
}

/// Task class
///
/// The Task class aims to embbed a thread-based periodic task
#[derive(Debug)]
pub struct Task<T>
where
    T: TaskDefinition,
{
    timeout_: Duration,
    name_: String,
    stopped_requested_: SafeSyncPtr<bool>,
    task_definition_: SafeSyncPtr<T>,
}

impl<T: TaskDefinition + std::marker::Send> Task<T> {
    fn new(dur: Duration, name: String, def: SafeSyncPtr<T>) -> Task<T> {
        Task {
            timeout_: dur,
            name_: name,
            stopped_requested_: build_safe_sync_ptr(false),
            task_definition_: def,
        }
    }

    pub fn start(duration: Duration, name: String, def: &SafeSyncPtr<T>) -> RunningTask {
        let task_name = name.clone();
        let new_task = Task::new(duration, name, Arc::clone(&def));
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
        if let Err(error) = runner.lock().unwrap().init() {
            println!("Fail to init {}: {}", &self.name_, &error);
            return;
        }

        loop {
            // custom run
            if let Err(error) = runner.lock().unwrap().run() {
                println!("Fail to run {}: {}", &self.name_, &error);
                break;
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
        runner.lock().unwrap().terminate();
    }
}

// basic unit test
#[cfg(test)]
mod tests {
    use crate::build_safe_sync_ptr;

    #[test]
    fn builder() {
        // test that the builder works
        let built = build_safe_sync_ptr(0);
        if let Err(error) = built.lock() {
            let data_error = error.get_ref();
            println!("error: {data_error}");
        }
        drop(built); // explicit drop
    }
}
