pub fn gray_code(n: u32) -> u32 {
	n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn zero() {
		assert_eq!(gray_code(0), 0);
	}

	#[test]
	fn one() {
		assert_eq!(gray_code(1), 1);
	}

	#[test]
	fn other() {
		assert_eq!(gray_code(89), 117);
	}

	#[test]
	fn max() {
		assert_eq!(gray_code(std::u32::MAX), 2147483648);
	}

	#[test]
	fn convert_binary_to_gray() {
		let tab_binary = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
		let mut tab_gray = vec![];

		for nb in tab_binary.iter() {
			tab_gray.push(gray_code(*nb));
		}

		assert_eq!(tab_gray, vec![0, 1, 3, 2, 6, 7, 5, 4, 12, 13])
	}
}