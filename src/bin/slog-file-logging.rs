#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use std::fs::OpenOptions;
use slog::Drain;

fn main() {
   let log_path = "target/your_log_file_path.log";
   let file = OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(log_path)
      .unwrap();

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!());
}