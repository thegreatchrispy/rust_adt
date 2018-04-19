/// The `Bag` type.
/// 
/// An abstract data type that is represented by two basic structures.
/// 
///   1. `data` is a Vector of values.
///   2. `used` is an integer that determines how much of the bag's capacity is being used.
#[derive(Default, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Bag<f64> {
	data: Vec<f64>,
	used: u64
}

// impl Drop for Bag<f64>
// impl Deref for Bag<f64>
// impl Copy for Bag<f64>
// impl Clone for Bag<f64>
// impl PartialEq for Bag<f64>
// // Compare a bag with another bag for equality
// impl PartialEq for Bag<f64> {
// 	fn eq(&self, other: &Bag<f64>) -> bool {
// 		let mut isEqual = true;

// 		if self.used != other.used {
// 			isEqual = false;
// 		}

// 		for i in 0..other.data.len() {
// 			if self.data[i] != other.data[i] {
// 				isEqual = false;
// 			}
// 		}

// 		isEqual
// 	}
// }


impl Bag<f64> {
	/// Returns a new `Bag` with a capacity of 1 and no data elements being used.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let x: Bag<f64> = Bag::<f64>::new();
	/// assert_eq!(x.data.len(), 1);
	/// assert_eq!(x.used, 0);
	/// ```
	pub fn new() -> Bag<f64> {
		Bag::<f64> {
			data: Vec::with_capacity(1),
			used: 0
		}
	}

	/// Returns a new `Bag` with the given capacity and no data elements being used.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let x: Bag<f64> = Bag::<f64>::new_with_capacity(10);
	/// assert_eq!(x.data.len(), 0);
	/// assert_eq!(x.used, 0);
	/// ```
	pub fn new_with_capacity(initial_capacity: u64) -> Bag<f64> {
		if initial_capacity < 1 {
			panic!("new_with_capacity() -> initial_capacity must be > 0");
		}

		// Needs to divide initial_capacity by 2 if memory is exceeded
		Bag::<f64> {
			data: Vec::with_capacity(initial_capacity as usize),
			used: 0
		}
	}

	/// Returns a new `Bag` with the given capacity and no data elements being used.
	/// 
	/// # Examples
	/// 
	/// ```
	/// // Create an original bag.
	/// let mut x: Bag<f64> = Bag::<f64>::new_with_capacity(3);
	/// x.data.push(1.0);
	/// x.data.push(2.0);
	/// x.data.push(3.0);
	/// 
	/// // Create the new bag from the original bag.
	/// let y: Bag<f64> = Bag::<f64>::new_from_bag(&x);
	/// assert_eq!(x.data.len(), y.data.len());
	/// assert_eq!(x.used, y.used);
	/// ```
	pub fn new_from_bag(source: &Bag<f64>) -> Bag<f64> {
		// Needs to check if source is null
		let mut dest = Vec::with_capacity(source.get_capacity() as usize);

		for i in 0..source.data.len() {
			dest[i] = source.data[i];
		}

		Bag::<f64> {
			used: source.used,
			data: dest.clone()
		}
	}

	/// Returns the length of `data` as an integer.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let x: Bag<f64> = Bag::<f64>::new_with_capacity(1);
	/// x.data.push(1.0);
	/// x.data.push(2.0);
	/// assert_eq!(2, x.get_capacity());
	/// ```
	pub fn get_capacity(&self) -> u64 {
		self.data.len() as u64
	}

	/// Returns the number of elements being used in `data`.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let x: Bag<f64> = Bag::<f64>::new_with_capacity(5);
	/// x.insert(1.0);
	/// x.insert(2.0);
	/// assert_eq!(2, x.size());
	/// ```
	pub fn size(&self) -> u64 {
		self.used
	}

	/// Determines whether a new_capacity is safe to use.
	/// If the new_capacity is safe to use, then a new bag is
	/// created, cloning the data from the old bag into the new bag.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let mut x: Bag<f64> = Bag::<f64>::new();
	/// x.insert(5.0);
	/// x.ensureCapacity(x.get_capacity + 1);
	/// ```
	pub fn ensure_capacity(&mut self, new_capacity: u64) {
		if new_capacity < 1 {
			panic!("ensure_capacity -> new_capacity must be > 0");
		}

		if self.data.len() < new_capacity as usize {
			let mut x = Vec::with_capacity(new_capacity as usize);
			x = self.data.clone();
			drop(&self.data);
			self.data = x;
		}
	}

	/// Returns the number of erased elements matching the given target.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let mut x: Bag<f64> = Bag::<f64>::new();
	/// x.insert(1.0);
	/// x.insert(1.0);
	/// x.insert(1.0);
	/// x.insert(2.0);
	/// let count = x.erase(1.0);
	/// assert_eq!(3, count);
	/// assert_eq!(1, x.size());
	/// ```
	pub fn erase(&mut self, target: f64) -> u64 {
		let mut index = 0;
		let mut number_removed = 0;

		while index < self.used {
			if self.data[index as usize] == target {
				self.used -= 1;
				self.data[index as usize] = self.data[self.used as usize];
				number_removed += 1;
			}
			else {
				index += 1;
			}
		}

		return number_removed
	}

	/// Inserts an element into the `data` list in the bag.
	/// Checks to ensure the capacity can handle the new entry.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let mut x: Bag<f64> = Bag::<f64>::new();
	/// x.insert(1.0);
	/// x.insert(2.0);
	/// assert_eq!(2, x.size());
	/// ```
	pub fn insert(&mut self, entry: f64) {
		let current_capacity = self.get_capacity();

		if self.size() == current_capacity {
			self.ensure_capacity(current_capacity * 2);
		}

		self.used += 1;
		self.data[self.used as usize] = entry;
	}
}