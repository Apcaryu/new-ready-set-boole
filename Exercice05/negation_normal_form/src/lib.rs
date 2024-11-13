// fn get_a_b(){}

fn negation(stack: &mut Vec<String>) {
	if stack.len() == 0 {
		panic!("missing variable");
	}

	let tmp = stack.pop();
	let mut res = String::new();

	match tmp {
		Some(val) => {
			if val.chars().last() != Some('!') {
				res = String::from(format!("{}!", val));
			} else {
				let mut tmp_res = val.chars();
				tmp_res.next_back();
				res = String::from(tmp_res.as_str())
			}
		},
		_ => {}
	}

	stack.push(res);
}

fn conjunction(stack: &mut Vec<String>) {
	if stack.len() < 2 {
		panic!("missing variable")
	}

	let var_b = stack.pop().unwrap();
	let var_a = stack.pop().unwrap();

	stack.push(String::from(format!("{}{}&", var_a, var_b)));
}

fn disjunction(stack: &mut Vec<String>) {
	if stack.len() < 2 {
		panic!("missing variable")
	}

	let var_b = stack.pop().unwrap();
	let var_a = stack.pop().unwrap();

	stack.push(String::from(format!("{}{}|", var_a, var_b)));
}

fn exclusive_disjunction(stack: &mut Vec<String>) {
	if stack.len() < 2 {
		panic!("missing variable")
	}

	let var_b = stack.pop().unwrap();
	let var_a = stack.pop().unwrap();

	stack.push(String::from(format!("{}{}!&{}!{}&|", var_a, var_b, var_a, var_b)));
}

fn material_condition(stack: &mut Vec<String>) {
	if stack.len() < 2 {
		panic!("missing variable")
	}

	let var_b = stack.pop().unwrap();
	let var_a = stack.pop().unwrap();

	stack.push(String::from(format!("{}!{}|", var_a, var_b)));
}

pub fn negation_normal_form(formula: &str) -> String {
	let mut stack = Vec::new();

	for chr in formula.chars() {
		match chr {
			'A'..='Z' => {
				stack.push(String::from(chr));
			},
			'!' => {
				negation(&mut stack);
			},
			'&' => {
				conjunction(&mut stack);
			},
			'|' => {
				disjunction(&mut stack);
			},
			'^' => {
				exclusive_disjunction(&mut stack);
			},
			'>' => {
				material_condition(&mut stack);
			},
			'=' => {},
			_ => {
				panic!("invalid input")
			}
		}
	}

	match stack.pop() {
		Some(val) => return val,
		_ => {
			panic!("Stack is empty")
		},
	}

}

#[cfg(test)]
mod tests {
    use super::*;
	// /*
	#[test]
	fn base_case() {
		assert_eq!(negation_normal_form("A!"), "A!");
		assert_eq!(negation_normal_form("AB&"), "AB&");
		assert_eq!(negation_normal_form("AB|"), "AB|");
		assert_eq!(negation_normal_form("AB^"), "AB!&A!B&|");
		assert_eq!(negation_normal_form("AB>"), "A!B|");
		// assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
	}
	// */
	#[test]
	fn mid_case() {
		assert_eq!(negation_normal_form("A!!"), "A");
		// assert_eq!(negation_normal_form("AB&!"), "A!B!|");
		// assert_eq!(negation_normal_form("AB|!"), "A!B!&");
	}

	/*
	#[test]
	fn tricky_case() {
		// assert_eq!(negation_normal_form("A!B>A&"), "AB|A&");
		// assert_eq!(negation_normal_form("AB&!C|!"),"AB&C!&");
	}
	*/
	/*	
	#[test]
	fn already_in_nnf() {
		// assert_eq!(negation_normal_form("AB!&A!B&|"), "AB!&A!B&|");
		// assert_eq!(negation_normal_form("A!B|"), "A!B|");
		// assert_eq!(negation_normal_form("AB&A!B!&|"), "AB&A!B!&|");
	}
*/
	/*
	#[test]
	#[should_panic(expected = "invalid input")]
	fn invalid_input() {
		// negation_normal_form("AB@");
	}
*/
	/*
	#[test]
	#[should_panic(expected = "missing operator")]
	fn missing_operator() {
		// negation_normal_form("AB!");
	}
 	*/
	/*
	#[test]
	#[should_panic(expected = "missing variable")]
	fn missing_variable() {
		// negation_normal_form("A&");
	}
*/
}
