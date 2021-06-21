use std::io;
	
fn main() {
	use std::io::Write;
	
	let mut input = String::new();
	let mut file = std::fs::File::create("data.txt").expect("create file");

	match io::stdin().read_line(&mut input) {
		Ok(n) => {
			println!("{} bytes read", n);
			println!("{}", input);
		}
		Err(error) => println!("error: {}", error),
	}

	file.write_all(input.as_bytes()).expect("write failed");
	println!("Data writen to file");
}
