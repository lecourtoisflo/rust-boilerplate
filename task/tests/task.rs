#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use task::{build_safe_sync_ptr, Task, TaskDefinition};

    struct TestTask {
        pub init_ok: bool,
        pub nb_runs: u32,
        pub run_counter: u32,
        pub terminated: bool,
    }

    impl TestTask {
        fn new(init: bool, max_nb_runs: u32) -> TestTask {
            TestTask {
                init_ok: init,
                nb_runs: max_nb_runs,
                run_counter: 0,
                terminated: false,
            }
        }
    }
    impl TaskDefinition for TestTask {
        fn init(&mut self) -> Result<(), String> {
            if !self.init_ok {
                return Err(String::from("Init NOK"));
            }
            Ok(())
        }
        fn run(&mut self) -> Result<(), String> {
            self.run_counter = self.run_counter + 1;
            if self.run_counter >= self.nb_runs {
                return Err(String::from("Run done"));
            }
            Ok(())
        }
        fn terminate(&mut self) {
            self.terminated = true;
        }
    }

    #[test]
    fn nominal() {
        let testtask_ptr = build_safe_sync_ptr(TestTask::new(true, 1));
        let running_task = Task::start(
            Duration::from_secs(1),
            String::from("nominal"),
            &testtask_ptr,
        );
        sleep(Duration::from_secs(3));
        running_task.stop();
        let testtask = testtask_ptr.lock().unwrap();
        assert_eq!(testtask.terminated, true);
        assert_eq!(testtask.run_counter, testtask.nb_runs);
    }

    #[test]
    fn stopping() {
        let testtask_ptr = build_safe_sync_ptr(TestTask::new(true, 10));
        let running_task = Task::start(
            Duration::from_secs(1),
            String::from("stopping"),
            &testtask_ptr,
        );
        sleep(Duration::from_secs(2));
        running_task.stop();
        let testtask = testtask_ptr.lock().unwrap();
        assert_eq!(testtask.terminated, true);
        // due to timing, the counter can equal 2 ou 3
        assert!(testtask.run_counter >= 2);
        assert!(testtask.run_counter < 4);
    }

    #[test]
    fn fail_init() {
        let testtask_ptr = build_safe_sync_ptr(TestTask::new(false, 1));
        let running_task = Task::start(
            Duration::from_secs(1),
            String::from("fail_init"),
            &testtask_ptr,
        );
        sleep(Duration::from_secs(2));
        running_task.stop();
        let testtask = testtask_ptr.lock().unwrap();
        assert_eq!(testtask.terminated, false);
        assert_eq!(testtask.run_counter, 0);
    }
}
