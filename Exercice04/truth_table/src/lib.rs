use boolean_evaluation::eval_formula;

fn search_value(tab: &Vec<char>, target: char) -> bool {
	for iter in tab.iter() {
		if *iter == target {
			return true
		}
	}
	false
}

fn get_variables(formula: &str) -> Vec<char> {
	let mut out = Vec::new();

	for chr in formula.chars() {
		match chr {
			'A'..='Z' => {
				if !search_value(&out, chr) {
					out.push(chr);
				}
			}
			_ => {}
		}
	}
	out
}

pub fn print_truth_table(formula: &str) {
	if formula.len() == 0 { panic!("need an input") }
	let vars_tab = get_variables(formula);
	// println!("vars : {:?}", get_variables(formula));
    println!("{}", eval_formula("0!") as u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	#[should_panic(expected = "need an input")]
    fn no_input() {
		print_truth_table("");
	}

	#[test]
	#[should_panic(expected = "invalid input")]
	fn invalid_input() {
		print_truth_table("AN@");
	}

	#[test]
	#[should_panic(expected = "missing operator")]
	fn missing_operator() {
		print_truth_table("AB|C");
	}

	#[test]
	#[should_panic(expected = "missing value")]
	fn missing_variable() {
		print_truth_table("A&");
	}
}
