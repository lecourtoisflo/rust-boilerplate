use std::time::Duration;

mod basic {
    use task::TaskDefinition;

    pub struct Basic {}

    impl TaskDefinition for Basic {
        fn init(&mut self) -> Result<(), String> {
            println!("Init basic task done");
            Ok(())
        }
        fn run(&mut self) -> Result<(), String> {
            println!("Run basic task done");
            Ok(())
        }
        fn terminate(&mut self) {
            println!("Terminate basic task done")
        }
    }
}

use basic::Basic;
type BasicTask = task::Task<Basic>;

fn main() {
    let task_def = task::build_safe_sync_ptr(Basic {});
    let running = BasicTask::start(Duration::from_secs(1), String::from("basic"), &task_def);
    std::thread::sleep(Duration::from_secs(5));
    running.stop();
}
