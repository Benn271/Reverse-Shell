use std::net::TcpListener;
use std::io::{Read, Write};
use std::process::Command;

//bind has to be a valid port to listen on
//A
//DONE
fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	
	for stream in listener.incoming() {
		//connect
		let mut stream = stream.unwrap();
		println!("Connected!");
		
		loop {
			//get command
			let command = get_command();

			//send command
			stream.write_all(command.as_bytes());

			//read output
			let mut buffer = [0; 512];
			stream.read(&mut buffer);
			let output = String::from_utf8_lossy(&buffer[..]);
			println!("");
			println!("{}", output);
			println!("");
		}
	}
}

fn get_command() -> String{
	//println!("Command: ");
	let mut input = String::new();
	std::io::stdin().read_line(&mut input);

	return input;
}