/// This is an interactive test program for the Bag abstract data type.
/// A menu is displayed to show the user what options they have for testing.
use std::io;
use bag::Bag;

pub fn print_menu() {
	println!("\n\n\tThe following choices are available with 2 bags: ");
	println!("\t\t A  Use the assignment operator to make b1 equal to b2");
	println!("\t\t a  Use the assignment operator to make b2 equal to b1");
	println!("\t\t C  Use the copy constructor to make b1 equal to b2");
	println!("\t\t c  Use the copy constructor to make b2 equal to b1");
	println!("\t\t I  Insert an item into b1");
	println!("\t\t i  Insert an item into b2");
	println!("\t\t R  Erase all of item from b1");
	println!("\t\t r  Erase all of item from b2");
	println!("\t\t X  Erase one of item from b1");
	println!("\t\t x  Erase one of item from b2");
	println!("\t\t O  Display both bags");
	println!("\t\t S  Print the result from the size( ) functions");
	println!("\t\t Q  Quit this test program\n\n");
}

pub fn get_user_command() -> char {
	let mut user_input = String::new();
	
	match io::stdin().read_line(&mut user_input) {
		Ok(_) => {
			match user_input.chars().next() {
				Some(c) => { return c; }
				None => { return '*'; }
			}
		}
		Err(_) => { return '*';}
	}
}

pub fn get_number() -> f64 {
	let mut user_input = String::new();
	println!("\t\tPlease enter a f64 number for the bag: ");
	io::stdin().read_line(&mut user_input).expect("Failed to read from stdin.");
	let trimmed = user_input.trim();
	
	match trimmed.parse::<f64>() {
		Ok(f) => { return f }
		Err(..) => { 
			println!("Incorrect type: expected <f64>.");
			return 0.0
		}
	}
}

pub fn show_bags(b1: &Bag<f64>, b2: &Bag<f64>) {
	// println!("\t\tb1 {}", b1);
	// println!("\t\tb2 {}", b2);
}

pub fn copybag(b: &Bag<f64>) -> Bag<f64> {
	let return_bag = Bag::<f64>::new_from_bag(&b);
	return_bag
}

pub fn interactive_main() {
	let mut b1 = Bag::<f64>::new();
	let mut b2 = Bag::<f64>::new();
	let mut choice = 'z';
	let mut done = false;

	println!("\tI have initialized two empty bags of doubles.");

	while !done {
		print_menu();
		choice = get_user_command();
		match choice {
			'A' => {
				b1 = b2;
			}
			'a' => {
				b2 = b1;
			}
			'C' => {
				b1 = copybag(&b2);
			}
			'c' => {
				b2 = copybag(&b1);
			}
			'S' | 's' => {
				println!("The bags' sizes are {} and {}", b1.size(), b2.size());
			}
			'I' => {
				b1.insert(get_number());
			}
			'i' => {
				b2.insert(get_number());
			}
			'R' => {
				b1.erase(get_number());
			}
			'r' => {
				b2.erase(get_number());
			}
			'X' => {
				b1.erase_one(get_number());
			}
			'x' => {
				b2.erase_one(get_number());
			}
			'O' | 'o' => {
				show_bags(&b1, &b2);
			}
			'Q' | 'q' => {
				println!("Ridicule is the best test of truth.");
				break;
			}
			_ => {
				println!("{} is invalid. Sorry.", choice);
			}
		}
	}
}