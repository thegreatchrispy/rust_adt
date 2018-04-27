extern crate rand;

use std::char;
use std::mem;
use self::rand::distributions::{IndependentSample, Range};
use bag::Bag;

const MANY_TESTS: usize = 5;
const POINTS: [i32; MANY_TESTS + 1] = [100, 32, 12, 12, 32, 12];
const DESCRIPTION: &'static [&'static str] = &[
	"tests for bag Class",
    "Testing insert and the constant member functions",
    "Testing the copy constructor and == methodr",
    "Testing the assignment operator",
    "Testing erase and erase_one functions",
    "Testing += method and non-instance method +"
];

/// Determines if the `Bag` is correct based on the following requirements:
/// 
/// * it has exactly n items
fn correct(test: &Bag<f64>, n: u64) -> bool {
	let mut answer = true;

	if test.size() != n { answer = false }
	println!("{}", if answer {"Test passed.\n"} else {"Test failed.\n"});
	answer
}

/// Returns POINTS[1] if the tests are passed.
/// 
/// Returns 0 if any tests fail.
/// 
/// Performs some basic tests of insert and the constant member functions.
pub fn test1() -> i32 {
	const TEST_SIZE: u64 = 3000;
	let mut test = Bag::<f64>::new();
	let mut test_letter = 'A';

	println!("{}. Testing size for an empty bag.", test_letter);
	if !correct(&test, 0) { return 0 }

	println!("{}. Adding the number 4.0 to the bag, and then testing size.", char::from_u32(test_letter as u32 + 1).unwrap());
	test.insert(4.0);
	if !correct(&test, 1) { return 0 }

	println!("{}. Inserting the number 2.0 into the bag.\n\tThen checking size.", char::from_u32(test_letter as u32 + 2).unwrap());
	test.insert(2.0);
	if !correct(&test, 2) { return 0 }

	println!("{}. Inserting the number 1.0 into the bag.\n\tThen checking size.", char::from_u32(test_letter as u32 + 3).unwrap());
	test.insert(1.0);
	if !correct(&test, 3) { return 0 }

	println!("{}. Inserting the number 3.0 into the bag.\n\tThen checking size.", char::from_u32(test_letter as u32 + 4).unwrap());
	test.insert(3.0);
	if !correct(&test, 4) { return 0 }

	println!("{}. Inserting the another 2.0 into the bag.\n\tThen checking size.", char::from_u32(test_letter as u32 + 5).unwrap());
	test.insert(2.0);
	if !correct(&test, 5) { return 0 }

	println!("\tThen checking occurrences of 2.0.");
	if test.occurrences(2.0) != 2 {
		return 0
	}
	else {
		println!("Test passed.");
	}

	println!("{}. Inserting the numbers 5.0, 6.0, 7.0 into the bag.\n\tThen checking size.", char::from_u32(test_letter as u32 + 6).unwrap());
	test.insert(5.0);
	test.insert(6.0);
	test.insert(7.0);
	if !correct(&test, 8) { return 0 }

	println!("{}. Inserting the 2 more 2.0's into the bag.\n\tThen checking occurrences of 2.0's.\n", char::from_u32(test_letter as u32 + 7).unwrap());
	test.insert(2.0);
	test.insert(2.0);
	if test.occurrences(2.0) != 4 {
		return 0
	}
	else {
		println!("Test passed.");
	}

	println!("{}. Inserting {} random items between 0 and 49\n\tand then checking size.", char::from_u32(test_letter as u32 + 8).unwrap(), TEST_SIZE);
	for i in 0..TEST_SIZE {
		let next = Range::new(0, 50);
		let mut rng = rand::thread_rng();
		let entry = next.ind_sample(&mut rng);
		test.insert(entry as f64);
	}
	if !correct(&test, TEST_SIZE + 10) { return 0 }

	POINTS[1]
}

/// Returns POINTS[2] if the tests are passed.
/// 
/// Returns 0 if any tests fail.
/// 
/// Performs some tests of the copy constructor and == method.
pub fn test2() -> i32 {
	let mut test = Bag::<f64>::new();

	println!("A. Testing that copy constructor works okay for empty bag...");
	let mut copy1 = Bag::<f64>::new_from_bag(&test);
	if !correct(&copy1, 0) { return 0 }

	println!("B. Testing copy constructor with 4-item bag...");
	test.insert(1.0);
	test.insert(1.0);
	test.insert(1.0);
	test.insert(1.0);
	let mut copy2 = Bag::<f64>::new_from_bag(&test);
	println!("\tand now testing the == method...");
	if test != copy2 || copy2 != test {
		println!("Test failed.\n");
		return 0;
	}
	else {
		println!("Test passed.\n");
	}

	test.insert(1.0);
	println!("C. Then checking size of copy");
	if !correct(&copy2, 4) { return 0 }
	println!("D. Altering original but not the copy");
	if !correct(&test, 5) { return 0 }

	println!("Copy constructor seems okay.");
	POINTS[2]
}

/// Returns POINTS[3] if the tests are passed.
/// 
/// Returns 0 if any tests fail.
/// 
/// Performs some tests of the assignment opertator.
pub fn test3() -> i32 {
	let mut test = Bag::<f64>::new();
	let mut old_bytes = Vec::<u8>::with_capacity(mem::size_of::<Bag<f64>>());
	let mut new_bytes = Vec::<u8>::with_capacity(mem::size_of::<Bag<f64>>());

	println!("A. Testing that assignment operator works okay for empty bag...");
	let mut copy1 = Bag::<f64>::new_from_bag(&test);
	copy1.insert(1.0);
	copy1 = test.clone();
	if !correct(&copy1, 0) { return 0 }

	println!("B. Testing assignment operator with 4-item bag...");
	test.insert(1.0);
	test.insert(1.0);
	test.insert(1.0);
	test.insert(1.0);
	let mut copy2 = Bag::<f64>::new_from_bag(&test);
	test.insert(1.0);
	println!("\taltering original by an insertion...");
	if test.occurrences(1.0) != 5 || copy2.occurrences(1.0) != 4 {
		println!("Test failed.");
		return 0;
	}
	println!("Test passed.\n\ttesting size of assigned to...");
	if !correct(&copy2, 4) { return 0 }
	println!("\ttesting size of original...");
	if !correct(&test, 5) { return 0 }

	println!("C. Testing assignment operator for a self-assignment...");
	// old_bytes.clone_from_slice(&test.get_data());
	// test = test.clone();
	// new_bytes.clone_from_slice(&test.get_data());
	// for i in 0..mem::size_of::<Bag<f64>>() {
	// 	if old_bytes[i] != new_bytes[i] {
	// 		println!("Test failed.");
	// 		return 0;
	// 	}
	// }

	println!("Test passed.");
	drop(old_bytes);
	drop(new_bytes);
	
	println!("Assignment operator seems okay.");
	POINTS[3]
}

/// Returns POINTS[4] if the tests are passed.
/// 
/// Returns 0 if any tests fail.
/// 
/// Performs some tests of the assignment opertator.
pub fn test4() -> i32 {
	POINTS[4]
}

/// Returns POINTS[5] if the tests are passed.
/// 
/// Returns 0 if any tests fail.
/// 
/// Performs some tests of the assignment opertator.
pub fn test5() -> i32 {
	POINTS[5]
}