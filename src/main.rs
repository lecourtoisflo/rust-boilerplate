use std::time::Duration;

use printer::Printer;

mod printer;

type PrinterTask = task::Task<printer::Printer>;

fn main() {
    let printer_task_def = task::build_safe_sync_ptr(Printer::new());
    let running_printer = PrinterTask::start(
        Duration::from_secs(1),
        String::from("printer"),
        &printer_task_def,
    );
    std::thread::sleep(Duration::from_secs(5));
    running_printer.stop();
}
