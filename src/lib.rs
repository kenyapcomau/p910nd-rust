extern crate lockfile;
use lockfile::Lockfile;

use std::str::FromStr;
use std::net::{IpAddr,SocketAddr,TcpListener,TcpStream};
use std::fs::{File,OpenOptions};
use std::io::prelude::*;
use std::{thread, time};

extern crate log;
use log::{trace,debug,info,warn,error};
extern crate syslog;

pub mod logger;

macro_rules! lockpathformat {
//    () => ("/var/lock/subsys/p910{}d")
      () => ("/tmp/p910{}d")
}

const BASEPORT:u32 = 9100;
const PRINTER_RETRY:u64 = 4000;	// milliseconds
const BUFFERSIZE:usize = 8192;

// Copy network data from inputfile (network) to pfile (printer) until EOS
// If bidir, also copy data from printer to network
fn copy_stream(mut conn: &TcpStream, mut pfile: &File) -> Result<(), std::io::Error>
{
	info!("copy_stream");
	let mut buffer = [0u8; BUFFERSIZE];
	loop {
		debug!("reading...");
		let bytes_read = conn.read(&mut buffer)?;
		debug!("{} bytes read", bytes_read);
		if bytes_read == 0 {
			break;
		}
		pfile.write_all(&buffer[0..bytes_read])?;
	}
	Ok(())
}

fn handle_client(stream: &TcpStream, device: &String, bidir: bool) -> Result<(), std::io::Error>
{
	// wait until printer is available
	let pfile = loop {
		match OpenOptions::new().read(bidir).write(true).create(false).truncate(true).open(device) {
		Ok(f) => {
			break f;
			},
		Err(_) => {
			thread::sleep(time::Duration::from_millis(PRINTER_RETRY));
			},
		}
	};
	copy_stream(&stream, &pfile)?;
	pfile.sync_all()?;
	Ok(())
}

pub fn server(pnumber: u32, device: &String, bidir: bool, ba: &String) -> Result<(), std::io::Error>
{
	let bindaddr = IpAddr::from_str(ba).expect(format!("{} not valid bind IP address", ba).as_str());
	let lockfilepath = format!(lockpathformat!(), pnumber);
	let lockfile = Lockfile::create(&lockfilepath).expect(format!("Lockfile {} already present", lockfilepath).as_str());
	let sockaddr = SocketAddr::new(bindaddr, (BASEPORT + pnumber) as u16);
	let listener = TcpListener::bind(sockaddr)?;
	info!("Server listening");
	loop {
		if let Ok((stream, addr)) = listener.accept() {
			info!("new client: {addr:?}");
			if let Err(e) = handle_client(&stream, device, bidir) {
				info!("handle_client: {}", e);
				break;
			}
		} else {
			break;
		}
	};
	lockfile.release()		// or just let it autorelease
}
