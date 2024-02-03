use log::LevelFilter;
use syslog::{Formatter3164,BasicLogger};

pub fn log_init(debug: bool) -> () {
	if debug {
		stderrlog::new()
			.module(module_path!())
			.verbosity(LevelFilter::Info)
			.init()
			.expect("Stderrlog not initialised");
	} else {
		let logger = syslog::unix(Formatter3164::default()).unwrap();
		log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
			.map(|()| log::set_max_level(LevelFilter::Info)).unwrap_or(())
	}
}
