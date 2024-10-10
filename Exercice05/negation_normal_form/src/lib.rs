// fn get_a_b(){}

fn get_a(formula: &str, index: &mut usize) -> String {
	let chr = formula.as_bytes()[*index - 1] as char;

	println!("idx = {}", index);
	match chr {
		'A'..='Z' => {
			*index -= 1;
			return String::from(format!("{}", chr))
		},
		'!' => {
			*index -= 1;
			format!("{}!", get_a(formula, index))
		}
		_ => {
			panic!("invalid input");
		}
	}
}

fn negation(formula: &str, index: &mut usize) -> String {
	let mut a = get_a(formula, index);
	println!("a = {}", a);
	if a.chars().last() == Some('!') {
		a.pop();
		return a
	} else {
		a.push('!');
		return a
	}
}

pub fn negation_normal_form(formula: &str) -> String {
	let mut nnf_formula = String::new();
	let mut idx = formula.len();
	// println!("idx start = {}", idx);

	while idx > 0 {
		let chr = formula.as_bytes()[idx - 1] as char;
		// println!("chr = {}", chr);
		match chr {
			'!' => {
				idx -= 1;
				let tmp = negation(formula, &mut idx);
				println!("tmp = {}", tmp);
				nnf_formula.insert_str(0,tmp.clone().as_str());
			},
			'&' => {},
			'|' => {},
			'^' => {},
			'>' => {},
			'=' => {},
			_ => {
				panic!("invalid input")
			}
		}
		if idx != 0 {idx -= 1}
	}
	
	nnf_formula
}

#[cfg(test)]
mod tests {
    use super::*;
	// /*
	#[test]
	fn base_case() {
		assert_eq!(negation_normal_form("A!"), "A!");
		// assert_eq!(negation_normal_form("AB&"), "AB&");
		// assert_eq!(negation_normal_form("AB|"), "AB|");
		// assert_eq!(negation_normal_form("AB^"), "AB!&A!B&|");
		// assert_eq!(negation_normal_form("AB>"), "A!B|");
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
