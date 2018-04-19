mod bag;
use bag::Bag;

fn main() {
	let x = Bag::<f64>::new();
	println!("x: {:?}", x);

	let y = Bag::<f64>::new_with_capacity(3);
	println!("y: {:?}", y);

	let mut z = Bag::<f64>::new_from_bag(&y);
	println!("z: {:?}", z);

	println!("z capacity: {}", z.get_capacity());

	println!("z size: {}", z.size());

	z.ensure_capacity(5);
}