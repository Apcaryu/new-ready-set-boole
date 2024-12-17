use negation_normal_form::negation_normal_form;

fn get_set(stack: &mut Vec<Vec<i32>>, chr: char, sets: &Vec<Vec<i32>>) {
	let idx = chr.to_digit(16).unwrap() - 10; // 10 is the value of A
	stack.push(sets[idx as usize].clone());
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
	let nnf_formula = negation_normal_form(formula);
	let mut stack = Vec::new();

	for chr in nnf_formula.chars() {
		match chr {
			'A'..='Z' => {
				get_set(&mut stack, chr, &sets);
			},
			'!' => {},
			'&' => {},
			'|' => {},
			_ => {}
		}
	}
	println!("stack {:?}", stack);
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_tests() {
        let sets = vec![
			vec![0, 1, 2],
			vec![0, 3, 4],
		];
		assert_eq!(eval_set("AB&", sets), vec![0]);

		let sets = vec![
			vec![0, 1, 2],
			vec![3, 4, 5],
		];
		assert_eq!(eval_set("AB|", sets), vec![0, 1, 2, 3, 4, 5]);

		let sets = vec![
			vec![0, 1, 2],
		];
		assert_eq!(eval_set("A!", sets), vec![]);
    }
}
