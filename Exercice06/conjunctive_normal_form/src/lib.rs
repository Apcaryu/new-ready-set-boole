use negation_normal_form::negation_normal_form;

fn conjunctive_normal_form(formula: &str) -> String {
	let nnf_formula = negation_normal_form(formula);
	let mut stack = Vec::new();

	for chr in nnf_formula.chars() {
		match chr {
			'A'..='Z' => {
				stack.push(String::from(chr));
			},
			'!' => {
				let tmp = stack.pop().unwrap();
				stack.push(String::from(format!("{}!", tmp)));
			},
			'&' => {},
			'|' => {},
			_ => {
				panic!("invalid input");
			}
		}
	}

	match stack.pop() {
		Some(val) => return val,
		None => {
			panic!("Stack is empty")
		}
	}
	// nnf_formula
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
		assert_eq!(conjunctive_normal_form("A"), "A");
		assert_eq!(conjunctive_normal_form("A!"), "A!");
		assert_eq!(conjunctive_normal_form("AB&"), "AB&");
		assert_eq!(conjunctive_normal_form("AB|"), "AB|");
		assert_eq!(conjunctive_normal_form("AB^"), "B!A!|AB|&");
		assert_eq!(conjunctive_normal_form("AB>"), "A!B|");
		assert_eq!(conjunctive_normal_form("AB="), "BA!|AB!|&");
	}

	#[test]
	fn mid_tests() {
		// assert_eq!(conjunctive_normal_form("AB|AB&|"), "ABA|&AB|B&");
		// assert_eq!(conjunctive_normal_form("AB|AB&&!"), "A!A!B!&B!A!B!&&");
	}

	#[test]
	fn to_cnf_only_use_nnf_function() {
		// assert_eq!(conjunctive_normal_form("A!!"), "A");
		// assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
		// assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
		// assert_eq!(conjunctive_normal_form("A!B>"), "AB|");
	}

	#[test]
	fn subject_tests() {
		// assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
		// assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
		// assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
		// assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
		// assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
		// assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
		// assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
	}
}
