use std::{env, time::Duration};

use printer::Printer;

mod printer;

type PrinterTask = task::Task<printer::Printer>;

fn build(mut args: impl Iterator<Item = String>) -> Result<(), &'static str> {
    // skip args[0] the name of the program
    args.next();

    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string"),
    };

    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file path"),
    };

    println!("query={},file_path={}", query, file_path);

    Ok(())
}

fn main() {
    let result = build(env::args());
    match result {
        Ok(()) => println!("OK"),
        Err(err) => println!("ERR: {}", err),
    }

    let printer_task_def = task::build_safe_sync_ptr(Printer::new());
    let running_printer = PrinterTask::start(
        Duration::from_secs(1),
        String::from("printer"),
        &printer_task_def,
    );
    std::thread::sleep(Duration::from_secs(5));
    running_printer.stop();
}
