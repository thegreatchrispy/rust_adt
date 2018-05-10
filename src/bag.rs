/// The `Bag` type.
/// 
/// An abstract data type that is represented by two basic structures.
/// 
///   1. `data` is a Vector of values.
///   2. `capacity` is an integer that determines how much space is in the `data` vector.
///   3. `used` is an integer that determines how much of the bag's capacity is being used.
use std::ops::{Add, AddAssign};
use std::fmt;

#[derive(Default, Clone, Debug, Hash)]
#[repr(C)]
pub struct Bag<T> {
	data: Vec<T>,
	capacity: u64,
	used: u64,
}

impl<T: PartialEq> Bag<T> where T:Clone + PartialEq + Add + AddAssign + fmt::Display {
	pub fn eq(&self, comparand: &Bag<T>) -> bool {
		let mut is_equal = true;

		if self.get_capacity() != comparand.get_capacity() { is_equal = false }

		if self.size() != comparand.size() { is_equal = false }

		let mut index = 0;

		while is_equal && index < self.size() {
			if self.data[index as usize] != comparand.data[index as usize] {
				is_equal = false;
			}
			else {
				index += 1;
			}
		}

		is_equal
	}

	pub fn ne(&self, comparand: &Bag<T>) -> bool {
		return !(self.eq(comparand))
	}
}

impl<T: fmt::Display> Bag<T> where T:Clone + PartialEq + Add + AddAssign + fmt::Display {
	pub fn fmt(self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str = "";
		writeln!(f, "Bag");
		write!(f, "data: ");

		for datum in &self.data {
			write!(f, "{}", str);
			write!(f, "{}", datum);
			str = ", ";
		}

		write!(f, "\ncapacity: {}", self.get_capacity());

		write!(f, "\nused: {}", self.size());
		Ok(())
	}
}

impl<T: Clone> Bag<T> where T:Clone + PartialEq + Add + AddAssign + fmt::Display {
	pub fn clone(&self) -> Bag<T> {
		let bag = Bag::<T>::new_from_bag(&self);
		bag
	}
}

// impl AddAssign for Bag<T>
impl<T: AddAssign> Bag<T> where T:Clone + PartialEq + Add + AddAssign + fmt::Display {
	pub fn add_assign(&mut self, mut addend: Bag<T>) {
		let mut current_data = self.data.clone();
		let current_used = self.size();

		if self.size() + addend.size() > self.get_capacity() {
			self.ensure_capacity(current_used + addend.size());
		}

		current_data.append(&mut addend.data);

		*self = Bag::<T> {
			data: current_data,
			capacity: self.get_capacity() + addend.get_capacity(),
			used: self.size() + addend.size(),
		};
	}
}

impl<T: Add> Bag<T> where T:Clone + PartialEq + Add + AddAssign + fmt::Display {
	pub fn add(self, addend: Bag<T>) -> Bag<T> {
		let mut new_bag = Bag::<T>::new_with_capacity(self.get_capacity() + addend.get_capacity());
		new_bag.add_assign(self);
		new_bag.add_assign(addend);

		new_bag
	}
}

impl<T> Bag<T> where T:Clone + PartialEq + Add + AddAssign + fmt::Display {
	/// Returns a new `Bag` with a capacity of 1 and no data elements being used.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let x: Bag<T> = Bag::<T>::new();
	/// assert_eq!(x.data.len(), 1);
	/// assert_eq!(x.used, 0);
	/// ```
	pub fn new() -> Bag<T> {
		Bag::<T> {
			data: Vec::with_capacity(1),
			capacity: 1,
			used: 0,
		}
	}

	/// Returns a new `Bag` with the given capacity and no data elements being used.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let x: Bag<T> = Bag::<T>::new_with_capacity(10);
	/// assert_eq!(x.data.len(), 0);
	/// assert_eq!(x.used, 0);
	/// ```
	pub fn new_with_capacity(initial_capacity: u64) -> Bag<T> {
		if initial_capacity < 1 {
			panic!("new_with_capacity() -> initial_capacity must be > 0");
		}

		// Needs to divide initial_capacity by 2 if memory is exceeded
		Bag::<T> {
			data: Vec::with_capacity(initial_capacity as usize),
			capacity: initial_capacity,
			used: 0,
		}
	}

	/// Returns a new `Bag` with the given capacity and no data elements being used.
	/// 
	/// # Examples
	/// 
	/// ```
	/// // Create an original bag.
	/// let mut x: Bag<T> = Bag::<T>::new_with_capacity(3);
	/// x.data.push(1.0);
	/// x.data.push(2.0);
	/// x.data.push(3.0);
	/// 
	/// // Create the new bag from the original bag.
	/// let y: Bag<T> = Bag::<T>::new_from_bag(&x);
	/// assert_eq!(x.data.len(), y.data.len());
	/// assert_eq!(x.used, y.used);
	/// ```
	pub fn new_from_bag(source: &Bag<T>) -> Bag<T> {
		// Needs to check if source is null
		let mut dest = Vec::with_capacity(source.get_capacity() as usize);
		let mut transfer = source.get_data();

		for i in 0..source.size() {
			dest.push(transfer[i as usize]);
		}

		Bag::<T> {
			data: dest.clone(),
			capacity: source.get_capacity(),
			used: source.size(),
		}
	}

	pub fn get_data(&self) -> &Vec<T> {
		&self.data
	}

	/// Returns the capacity of `data` as an integer.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let x: Bag<T> = Bag::<T>::new_with_capacity(1);
	/// x.insert(1.0);
	/// x.insert(2.0);
	/// assert_eq!(2, x.get_capacity());
	/// ```
	pub fn get_capacity(&self) -> u64 {
		self.capacity
	}

	/// Returns the number of elements being used in `data`.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let x: Bag<T> = Bag::<T>::new_with_capacity(5);
	/// x.insert(1.0);
	/// x.insert(2.0);
	/// assert_eq!(2, x.size());
	/// ```
	pub fn size(&self) -> u64 {
		self.used
	}

	/// Inserts an element into the `data` list in the bag.
	/// Checks to ensure the capacity can handle the new entry.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let mut x: Bag<T> = Bag::<T>::new();
	/// x.insert(1.0);
	/// x.insert(2.0);
	/// assert_eq!(2, x.size());
	/// ```
	pub fn insert(&mut self, entry: T) {
		let current_capacity = self.get_capacity();

		if self.size() == current_capacity {
			self.ensure_capacity(current_capacity * 2);
		}

		self.data.push(entry);
		self.used += 1;
	}

	pub fn occurrences(&self, target: T) -> u64 {
		let mut answer = 0;

		for i in 0..self.size() {
			if target == self.data[i as usize] {
				answer += 1;
			}
		}

		answer
	}

	/// Determines whether a new_capacity is safe to use.
	/// If the new_capacity is safe to use, then a new bag is
	/// created, cloning the data from the old bag into the new bag.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let mut x: Bag<T> = Bag::<T>::new();
	/// x.insert(5.0);
	/// x.ensureCapacity(x.get_capacity + 1);
	/// ```
	pub fn ensure_capacity(&mut self, new_capacity: u64) {
		if new_capacity < 1 {
			panic!("ensure_capacity -> new_capacity must be > 0");
		}

		if self.capacity < new_capacity {
			let mut x = Vec::with_capacity(new_capacity as usize);
			x = self.data.clone();
			drop(&self.data);
			self.data = x;
			self.capacity = new_capacity;
		}
	}

	/// Returns the number of erased elements matching the given target.
	/// 
	/// # Examples
	/// 
	/// ```
	/// let mut x: Bag<T> = Bag::<T>::new();
	/// x.insert(1.0);
	/// x.insert(1.0);
	/// x.insert(1.0);
	/// x.insert(2.0);
	/// let count = x.erase(1.0);
	/// assert_eq!(3, count);
	/// assert_eq!(1, x.size());
	/// ```
	pub fn erase(&mut self, target: T) -> u64 {
		let mut index = 0;
		let mut number_removed = 0;

		while index < self.size() {
			if self.data[index as usize] == target {
				self.used -= 1;
				self.data.remove(index as usize);
				number_removed += 1;
			}
			else {
				index += 1;
			}
		}

		number_removed
	}

	pub fn erase_one(&mut self, target: T) -> bool {
		let mut index = 0;
		
		while index < self.size() && self.data[index as usize].clone() != target {
			index += 1;
		}

		if index == self.size() { return false }

		self.used -= 1;
		self.data.remove(index as usize);
		true
	}

	//Needs the capacity variable.
	pub fn trim_to_size(&mut self) {
		if self.size() < self.get_capacity() {
			let mut new_capacity = 0;

			if self.size() <= 1 {
				new_capacity = 1;
			}
			else {
				new_capacity = self.size();
			}

			self.capacity = new_capacity;
			let mut trimmed_array = Vec::with_capacity(new_capacity as usize);
			trimmed_array = self.data.clone();
			drop(&self.data);
			self.data = trimmed_array;
			self.capacity = new_capacity;
		}
	}
}