#[derive(Debug, Clone)]
struct Bag<i64> {
	data: Vec<i64>,
	used: i64
}

// Compare a bag with another bag for equality
impl PartialEq for Bag<i64> {
	fn eq(&self, other: &Bag<i64>) -> bool {
		let mut isEqual = true;

		if self.used != other.used {
			isEqual = false;
		}

		for i in 0..other.data.len() {
			if self.data[i] != other.data[i] {
				isEqual = false;
			}
		}

		isEqual
	}
}

impl<T> Bag<T> {
	// No argument constructor
	fn new() -> Bag<i64> {
		Bag::<i64> {
			data: vec![0; 1],
			used: 0
		}
	}

	// Constructor given an initial capacity
	fn new_with_capacity(initial_capacity: usize) -> Bag<i64> {
		if initial_capacity < 1 {
			//panic
		}

		// Needs to divide initial_capacity by 2 if memory is exceeded
		Bag::<i64> {
			data: vec![0; initial_capacity],
			used: 0
		}
	}

	// Constructor given a bag
	fn new_from_bag(source: Bag<i64>) -> Bag<i64> {
		// Needs to check if source is null
		let mut dest = vec![0;source.used as usize];

		for i in 0..source.data.len() {
			dest[i] = source.data[i];
		}

		Bag::<i64> {
			used: source.used,
			data: dest.clone()
		}
	}

	// Clone already implemented

	// Get the size of the bag
	// fn size(&self) -> T {
	// 	self.used
	// }

	// Get the capacity of the bag
	fn get_capacity(&self) -> i64 {
		self.data.len() as i64
	}
}

fn main() {
	let mut a = Bag::<i64>::new();
	println!("{:?}", a);

	let mut b = Bag::<i64>::new_with_capacity(5);
    println!("{:?}", b);

	let mut c = Bag::<i64> {data: vec![0,1,2,3,4], used: 5};
	let mut d = Bag::<i64> {data: vec![0,1,2,3,4], used: 5};
	println!("{:?}", c == d);

	let mut e = Bag::<i64> {data: vec![0,1,2,3,4], used: 5};
	let mut f = Bag::<i64>::new_from_bag(e);
	let mut g = f.clone();
	println!("{:?}", f);
}
