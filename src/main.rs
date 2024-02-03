extern crate clap;
use clap::{Arg,Command};
extern crate log;
use log::{info,error};
extern crate daemonize;
use daemonize::Daemonize;

use std::process;

extern crate p910nd;
use p910nd::logger;

fn main()
{
	let matches = Command::new("P910nd")
		.version(env!("CARGO_PKG_VERSION"))
		.author("https://github.com/kenyapcomau/p910nd-rust")
		.about("Non-spooling printer daemon")
		.arg(
		 	Arg::new("bidir")
				.short('b')
				.long("bidir")
				.action(clap::ArgAction::SetTrue)
				.help("Bidirectional communication"),
		)
		.arg(
			Arg::new("debug")
				.short('d')
				.long("debug")
				.action(clap::ArgAction::SetTrue)
				.help("Log to stderr"),
		)
		.arg(
			Arg::new("device")
				.short('f')
				.long("device")
				.value_parser(clap::builder::NonEmptyStringValueParser::new())
				.action(clap::ArgAction::Set)
				.default_value("/dev/usb/lp0")
				.value_name("DEVICE")
				.help("Device to spool to"),
		)
		.arg(
			Arg::new("bindaddr")
				.short('i')
				.long("bindaddr")
				.value_parser(clap::builder::NonEmptyStringValueParser::new())
				.action(clap::ArgAction::Set)
				.default_value("0.0.0.0")
				.value_name("BINDADDR")
				.help("IP address to bind to"),
		)
		.arg(
			Arg::new("printer")
				.value_parser(clap::value_parser!(u32).range(0..9))
				.default_value("0")
				.value_name("PRINTER_NUMBER")
				.help("Printer number"),
		)
		.get_matches();

	let bidir = matches.get_flag("bidir");
	let debug = matches.get_flag("debug");
	let device = matches.get_one("device").unwrap();
	let bindaddr = matches.get_one("bindaddr").unwrap();
	let pnumber: u32 = *matches.get_one("printer").expect("required");

	logger::log_init(debug);

	info!("Run as server");
	if !debug {
		match Daemonize::new().start() {
			Ok(_) => { },
			Err(e) => { error!("Error {}", e); },
		};
	};
	if let Err(e) = p910nd::server(pnumber, &device, bidir, bindaddr) {
		error!("{}", e);
		process::exit(1);
	}
}
