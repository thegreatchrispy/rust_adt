//! This is documentation for the `bag` crate.
//!
//! The bag crate is an abstract data type implementation for the bag data structure.
use std::io;

mod bag;
mod bag_exam;
mod bag_test;

const MANY_TESTS: usize = 5;
const POINTS: [i32; MANY_TESTS + 1] = [100, 32, 12, 12, 32, 12];
const DESCRIPTION: &'static [&'static str] = &[
	"Tests for the Bag Class",
    "Testing insert and the constant member functions",
    "Testing the copy constructor and == methodr",
    "Testing the assignment operator",
    "Testing erase and erase_one functions",
    "Testing += method and non-instance method +"
];

pub fn read_user_input_char() -> char {
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

pub fn run_test(id: i32, message: String, max: i32) -> i32 {
	let mut result = 0;

	println!("\n\nSTART OF TEST {}:", id);
	println!("{} ({} points).", message, max);
	match id {
		1 => result = bag_exam::test1(),
		2 => result = bag_exam::test2(),
		3 => result = bag_exam::test3(),
		4 => result = bag_exam::test4(),
		5 => result = bag_exam::test5(),
		_ => println!("ERROR: Try again.")
	}

	if result > 0 {
		println!("Test {} got {} points out of a possible {}.\n\n", id, result, max);
	}
	else {
		println!("Test {} failed.", id);
		println!("END OF TEST {}.\n\n", id);
	}

	result
}

pub fn main() {
	println!("Running {}", DESCRIPTION[0]);

	println!("Enter 'a' for automatic tests.");
	println!("Enter 'i' for interactive tests.");
	println!("Enter choice: ");

	let user_char = read_user_input_char();

	if user_char == '*' {
		println!("Enter an 'a' or 'i'.");
	}
	
	if user_char == 'a' {
		let mut point_total = 0;
		point_total += run_test(1, DESCRIPTION[1].to_string(), POINTS[1]);
		point_total += run_test(2, DESCRIPTION[2].to_string(), POINTS[2]);
		point_total += run_test(3, DESCRIPTION[3].to_string(), POINTS[3]);
		point_total += run_test(4, DESCRIPTION[4].to_string(), POINTS[4]);
		point_total += run_test(5, DESCRIPTION[5].to_string(), POINTS[5]);

		println!("If you submit your bag to Prof. Haiduk now, you will have");
        println!("{} points out of the {} points from this test program.", point_total, POINTS[0]);
	}

	if user_char == 'i' {
		bag_test::interactive_main();
	}
}