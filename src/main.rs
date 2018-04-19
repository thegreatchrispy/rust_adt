mod bag;
mod bag_exam;
use bag::Bag;

fn main() {
	let mut point_total = 0;
	point_total += bag_exam::test1();
	point_total += bag_exam::test2();
	point_total += bag_exam::test3();
	point_total += bag_exam::test4();
	point_total += bag_exam::test5();

	println!("Total Points: {}", point_total);
}