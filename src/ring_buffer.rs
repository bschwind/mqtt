pub struct RingBuffer<T> {
	read: u32,
	write: u32,
	array: Vec<T>
}

impl <T: Clone + Default> RingBuffer<T> {
	pub fn new(capacity: u32) -> RingBuffer<T> {
		if capacity == 0
		   || capacity & (capacity - 1) != 0 // Check power of two
		   || capacity > 2147483648 { // Check less than 2^31 - 1
			panic!("Capacity {} is not a power of two in RingBuffer::new()", capacity);
		}

		RingBuffer {
			read: 1u32,
			write: 1u32,
			array: vec![T::default(); capacity as usize]
		}
	}

	fn mask(&self, val: u32) -> u32 {
		return val & (self.array.capacity() as u32 - 1);
	}

	pub fn push(&mut self, val: T) {
		assert!(!self.full());
		let temp = self.mask(self.write) as usize;
		self.write += 1;
		self.array[temp] = val;
	}

	pub fn peek(&self) -> &T {
		assert!(!self.empty());
		&self.array[self.mask(self.read) as usize]
	}

	pub fn shift(&mut self) -> &T {
		assert!(!self.empty());
		let temp = self.mask(self.read) as usize;
		self.read += 1;
		&self.array[temp]
	}

	pub fn len(&self) -> u32 {
		self.write - self.read
	}

	pub fn empty(&self) -> bool {
		self.read == self.write
	}

	pub fn full(&self) -> bool {
		self.len() == self.array.capacity() as u32
	}
}

#[test]
fn test_ringbuffer_new() {
	let _ = RingBuffer::<u8>::new(256);
}

#[test]
#[should_panic]
fn test_non_power_of_2_capacities_1() {
	let _ = RingBuffer::<u8>::new(0);
}

#[test]
#[should_panic]
fn test_non_power_of_2_capacities_2() {
	let _ = RingBuffer::<u8>::new(17);
}

#[test]
#[should_panic]
fn test_non_power_of_2_capacities_3() {
	let _ = RingBuffer::<u8>::new(250);
}

#[test]
fn test_len_1() {
	let mut buf = RingBuffer::<u8>::new(256);
	assert_eq!(buf.len(), 0);
}

#[test]
fn test_len_2() {
	let mut buf = RingBuffer::<u8>::new(256);

	buf.push(1);
	buf.push(2);
	buf.push(3);

	assert_eq!(buf.len(), 3);
}

#[test]
fn test_len_3() {
	let mut buf = RingBuffer::<u8>::new(4);

	buf.push(1);
	buf.push(2);
	buf.push(3);
	buf.push(4);

	assert_eq!(buf.len(), 4);
}

#[test]
fn test_empty() {
	let mut buf = RingBuffer::<u8>::new(4);
	assert!(buf.empty(), true);
}

#[test]
fn test_full() {
	let mut buf = RingBuffer::<u8>::new(4);

	buf.push(1);
	buf.push(2);
	buf.push(3);
	buf.push(4);

	assert!(buf.full(), true);
}

#[test]
#[should_panic]
fn test_push_full() {
	let mut buf = RingBuffer::<u8>::new(4);

	buf.push(1);
	buf.push(2);
	buf.push(3);
	buf.push(4);
	buf.push(5);
}

#[test]
fn test_shift_1() {
	let mut buf = RingBuffer::<u8>::new(4);

	buf.push(1);
	buf.push(2);
	buf.push(3);
	buf.push(4);

	assert_eq!(*buf.shift(), 1);
	assert_eq!(*buf.shift(), 2);
	assert_eq!(*buf.shift(), 3);
	assert_eq!(*buf.shift(), 4);

	assert_eq!(buf.len(), 0);
}

#[test]
#[should_panic]
fn test_shift_2() {
	let mut buf = RingBuffer::<u8>::new(4);

	buf.push(1);
	buf.push(2);
	buf.push(3);
	buf.push(4);

	assert_eq!(*buf.shift(), 1);
	assert_eq!(*buf.shift(), 2);
	assert_eq!(*buf.shift(), 3);
	assert_eq!(*buf.shift(), 4);
	assert_eq!(*buf.shift(), 55555); // Should panic on an assert
}

#[test]
fn test_peek() {
	let mut buf = RingBuffer::<u8>::new(4);

	buf.push(1);
	buf.push(2);
	buf.push(3);
	buf.push(4);

	assert_eq!(*buf.peek(), 1);
	assert_eq!(*buf.shift(), 1);
	assert_eq!(*buf.peek(), 2);
	assert_eq!(*buf.shift(), 2);
	assert_eq!(*buf.shift(), 3);
	assert_eq!(*buf.shift(), 4);

	buf.push(5);
	assert_eq!(*buf.peek(), 5);

	buf.push(6);
	buf.push(7);
	assert_eq!(*buf.peek(), 5);
	assert_eq!(*buf.shift(), 5);
	assert_eq!(*buf.peek(), 6);
	assert_eq!(*buf.shift(), 6);
	assert_eq!(*buf.peek(), 7);
	assert_eq!(*buf.shift(), 7);
}
