use std::io;

mod bag;
mod bag_exam;

// fn interactive() {
// 	println!("Not implemented yet.");
// }

// fn automatic() {
// 	let mut point_total = 0;
// 	point_total += bag_exam::test1();
// 	point_total += bag_exam::test2();
// 	point_total += bag_exam::test3();
// 	point_total += bag_exam::test4();
// 	point_total += bag_exam::test5();

// 	println!("Total Points: {}", point_total);
// }

// fn main() {
// 	let mut input = String::new();

// 	println!("Enter 'a' for automatic tests.");
// 	println!("Enter 'i' for interactive tests.");
// 	println!("Enter choice: ");

// 	io::stdin().read_line(&mut input).expect("Failed to read line");

// 	match input.as_str() {
// 		"i" => interactive(),
// 		"a" => automatic(),
// 		_ => println!("input: {}", input.as_str())
// 	}
// }
fn main() {
	let mut point_total = 0;
	point_total += bag_exam::test1();
	point_total += bag_exam::test2();
	point_total += bag_exam::test3();
	point_total += bag_exam::test4();
	point_total += bag_exam::test5();

	println!("Total Points: {}", point_total);
}