#[derive(PartialEq)]
enum Symbol {
	Negation,
	Conjunction,
	Disjunction,
	ExclusiveDisjunction,
	MaterialCondition,
	LogicalEquivalence,
}

fn separator(formula: String) -> (String, String) {
	let mut var_a = String::new();
	let mut var_b = String::new();
	let mut stack: Vec<String>;

	for chr in formula.chars() {
		match chr {
			'A'..='Z' => {},
			'&' => {},
			_ => {
				panic!("invalid input");
			}
		}
	}


	(var_a, var_b)
}

fn negation(var_a: String) -> String {
	let res;

	// if var_a.chars().last() != Some('!') {
	// 	res = String::from(format!("{}!", var_a));
	// } else {
	// 	let mut tmp_res = var_a.chars();
	// 	tmp_res.next_back();
	// 	res = String::from(tmp_res.as_str())
	// }

	// if var_a.chars().last() == Some('!') {
	// 	let mut tmp_res = var_a.chars();
	// 	tmp_res.next_back();
	// 	res = String::from(tmp_res.as_str());
	// } else {
		
	// }

	match var_a.chars().last() {
		Some('!') => {
			let mut tmp_res = var_a.chars();
			tmp_res.next_back();
			res = String::from(tmp_res.as_str())
		},
		Some('&') => {
			res = String::from(format!("{}!", var_a)) // Not definitive
			},
		Some('|') => {
			res = String::from(format!("{}!", var_a)) // Not definitive
			},
		Some('^') => {
			res = String::from(format!("{}!", var_a)) // Not definitive
			},
		Some('>') => {
			res = String::from(format!("{}!", var_a)) // Not definitive
			},
		Some('=') => {
			res = String::from(format!("{}!", var_a)) // Not definitive
			},
		Some('A'..='Z') => {
			res = String::from(format!("{}!", var_a))
		},
		_ => {
			panic!("invalid input");
		}
	}

	res
}

fn conjunction(var_a: String, var_b: String) -> String {
	format!("{}{}&", var_a, var_b)
}

fn disjunction(var_a: String, var_b: String) -> String {
	format!("{}{}|", var_a, var_b)
}

fn exclusive_disjunction(var_a: String, var_b: String) -> String {
	let n_var_a = negation(var_a.clone());
	let n_var_b = negation(var_b.clone());
	format!("{}{}&{}{}&|", var_a, n_var_b, n_var_a, var_b)
}

fn material_condition(var_a: String, var_b: String) -> String {
	let n_var_a = negation(var_a);
	format!("{}{}|", n_var_a, var_b)
}

fn logical_equivalence(var_a: String, var_b: String) -> String {
	let n_var_a = negation(var_a.clone());
	let n_var_b = negation(var_b.clone());
	format!("{}{}&{}{}&|", var_a, var_b, n_var_a, n_var_b)
}

fn nnf_for_one_symbol(symbol:Symbol, stack: &mut Vec<String>) {
	let var_a;
	let var_b ;

	if symbol == Symbol::Negation {
		var_a = stack.pop().unwrap();
		var_b = String::new();
	} else {
		var_b = stack.pop().unwrap();
		var_a = stack.pop().unwrap();
	}

	match symbol {
		Symbol::Negation => {
			stack.push(negation(var_a));
		},
		Symbol::Conjunction => {
			stack.push(conjunction(var_a, var_b));
		},
		Symbol::Disjunction => {
			stack.push(disjunction(var_a, var_b));
		},
		Symbol::ExclusiveDisjunction => {
			stack.push(exclusive_disjunction(var_a, var_b));
		},
		Symbol::MaterialCondition => {
			stack.push(material_condition(var_a, var_b));
		},
		Symbol::LogicalEquivalence => {
			stack.push(logical_equivalence(var_a, var_b));
		},
	}
}

pub fn negation_normal_form(formula: &str) -> String {
	let mut stack = Vec::new();

	for chr in formula.chars() {
		match chr {
			'A'..='Z' => {
				stack.push(String::from(chr));
			},
			'!' => {
				nnf_for_one_symbol(Symbol::Negation, &mut stack);
			},
			'&' => {
				nnf_for_one_symbol(Symbol::Conjunction, &mut stack);
			},
			'|' => {
				nnf_for_one_symbol(Symbol::Disjunction, &mut stack);
			},
			'^' => {
				nnf_for_one_symbol(Symbol::ExclusiveDisjunction, &mut stack);
			},
			'>' => {
				nnf_for_one_symbol(Symbol::MaterialCondition, &mut stack);
			},
			'=' => {
				nnf_for_one_symbol(Symbol::LogicalEquivalence, &mut stack);
			},
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
		assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
	}
	// */
	#[test]
	fn mid_case() {
		assert_eq!(negation_normal_form("A!!"), "A");
		assert_eq!(negation_normal_form("A!B^"), "A!B!&AB&|");
		assert_eq!(negation_normal_form("AB!^"), "AB&A!B!&|");
		assert_eq!(negation_normal_form("A!B!^"), "A!B&AB!&|");
		assert_eq!(negation_normal_form("A!B>"), "AB|");
		assert_eq!(negation_normal_form("AB!>"), "A!B!|");
		assert_eq!(negation_normal_form("A!B!>"), "AB!|");
		assert_eq!(negation_normal_form("A!B="), "A!B&AB!&|");
		assert_eq!(negation_normal_form("AB!="), "AB!&A!B&|");
		assert_eq!(negation_normal_form("A!B!="), "A!B!&AB&|");
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

	#[test]
	fn already_in_nnf() {
		assert_eq!(negation_normal_form("AB!&A!B&|"), "AB!&A!B&|");
		assert_eq!(negation_normal_form("A!B|"), "A!B|");
		assert_eq!(negation_normal_form("AB&A!B!&|"), "AB&A!B!&|");
	}

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
