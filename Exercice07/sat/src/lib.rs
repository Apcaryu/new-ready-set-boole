pub fn sat(formula: &str) -> bool {false}

#[cfg(test)]
mod tests {
    use super::*;

	
	#[test]
	fn one_var() {
		assert!(sat("AA&"));
		assert!(sat("AA|"));
		assert!(!sat("AA^"));
		assert!(sat("AA>"));
		assert!(sat("AA="));
		assert!(!sat("AA!&"));
		assert!(sat("AA!|"));
		assert!(sat("AA!^"));
		assert!(sat("AA!>"));
		assert!(!sat("AA!="));
	}

	#[test]
    fn basic_satisfiable() {
		assert!(sat("A"));
		assert!(sat("A!"));
		assert!(sat("AB|"));
		assert!(sat("AB&"));
		assert!(sat("AB^"));
		assert!(sat("AB>"));
		assert!(sat("AB="));
	}

	#[test]
	fn mid_test() {
		assert!(sat("AA!&B|"));
		assert!(!sat("AA!&BB!&|"));
		assert!(sat("AA^B|"));
		assert!(!sat("AA^B&"));
		assert!(sat("AB=C&"));
		assert!(!sat("AB=AB=^"));
		assert!(sat("ABC|||"));
		assert!(!sat("AB&CC^&"));
	}

	#[test]
	fn explosive_test() {
		assert!(sat("ABCDEFGHIJKLMNOPQRSTUVWXYZ&&&&&&&&&&&&&&&&&&&&&&&&&"));
	}

}
