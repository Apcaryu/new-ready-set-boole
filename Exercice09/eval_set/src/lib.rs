use std::collections::btree_map::Values;

use negation_normal_form::negation_normal_form;

fn get_universe(sets: &Vec<Vec<i32>>) -> Vec<i32> {
	let mut universe = Vec::new();
	for set in sets {
		for value in set {
			let res = universe.iter().find(|&&x| x == *value);
			match res {
				None => {
					universe.push(*value);
				},
				_ => {}
			}
		}
	}
	
	universe
}

fn get_set(stack: &mut Vec<Vec<i32>>, chr: char, sets: &Vec<Vec<i32>>) {
	let idx = chr.to_digit(16).unwrap() - 10; // 10 is the value of A
	stack.push(sets[idx as usize].clone());
}

fn complement(stack: &mut Vec<Vec<i32>>, universe: &Vec<i32>) {
	let var = stack.pop();
	let mut res: Vec<i32> = Vec::new();
	match var {
		Some(var) => {
			for val in universe {
				let finded = var.iter().find(|&&x| x == *val);
				match finded {
					None => {
						res.push(*val);
					},
					_ => {}
				}
			}
		},
		_ => { panic!("set missing") }
	}
	
	stack.push(res);
}

fn intersection(stack: &mut Vec<Vec<i32>>) {
	let var_b = stack.pop();
	let var_a = stack.pop();
	let mut res =Vec::new();

	match var_b {
		Some(value_b) => {
			match var_a {
				Some(_) => {
					for value_a in var_a.unwrap() {
						let finded = value_b.iter().find(|&&x| x == value_a);
						match finded {
							Some(val) => {
								res.push(*val);
							}
							_ => {}
						}
					}
				},
				_ => { panic!("set missing") }
			}
		},
		_ => { panic!("set missing") }
	}

	stack.push(res);
}

fn union(stack: &mut Vec<Vec<i32>>) {
	let var_b = stack.pop();
	let var_a = stack.pop();
	let mut res;

	match var_a {
		Some(values) => {
			res = values;
		},
		_ => { panic!("set missing") }
	}

	match var_b {
		Some(values) => {
			for val in values {
				let finded = res.iter().find(|&&x| x == val);
				match finded {
					None => {
						res.push(val);
					},
					_ => {}
				}
			}
		},
		_ => { panic!("set missing") }
	}

	stack.push(res);
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
	let nnf_formula = negation_normal_form(formula);
	let universe = get_universe(&sets);
	let mut stack = Vec::new();

	for chr in nnf_formula.chars() {
		match chr {
			'A'..='Z' => {
				get_set(&mut stack, chr, &sets);
			},
			'!' => {
				complement(&mut stack, &universe);
			},
			'&' => {
				intersection(&mut stack);
			},
			'|' => {
				union(&mut stack);
			},
			_ => {}
		}
	}
	println!("stack {:?}", stack);
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn universe_test() {
		let sets = vec![
			vec![0, 1, 2],
			vec![0, 3, 4],
		];
		assert_eq!(get_universe(&sets), vec![0, 1, 2, 3, 4]);

		let sets = vec![
			vec![1, 3, 2],
			vec![0, 3, 4],
		];
		assert_eq!(get_universe(&sets), vec![1, 3, 2, 0, 4]);

	}

	#[test]
	fn complement_test() {
		let universe = vec![0, 1, 2];
		let mut stack = vec![vec![0, 1, 2]];
		complement(&mut stack, &universe);
		assert_eq!(stack.pop().unwrap(), vec![]);

		let universe = vec![2, 0, 1];
		stack.push(vec![0, 1, 2]);
		complement(&mut stack, &universe);
		assert_eq!(stack.pop().unwrap(), vec![]);

		let universe = vec![0, 1, 2, 3, 10];
		stack.push(vec![0, 1, 2]);
		complement(&mut stack, &universe);
		assert_eq!(stack.pop().unwrap(), vec![3, 10]);
	}

	#[test]
	fn intersection_test(){
		let mut stack = vec![vec![0, 1, 2], vec![3, 4, 5]];
		intersection(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![]);

		let mut stack = vec![vec![0, 1, 2], vec![0, 3, 4]];
		intersection(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0]);

		let mut stack = vec![vec![2, 1, 0], vec![0, 3, 4]];
		intersection(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0]);

		let mut stack = vec![vec![0, 1, 3], vec![0, 3, 4]];
		intersection(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0, 3]);

		let mut stack = vec![vec![1, 3, 0], vec![0, 3, 4]];
		intersection(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![3, 0]);
	}

	#[test]
	fn union_test() {
		let mut stack = vec![vec![], vec![]];
		union(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![]);

		let mut stack = vec![vec![0, 1, 2], vec![]];
		union(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0, 1, 2]);

		let mut stack = vec![vec![], vec![0, 1, 2]];
		union(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0 ,1, 2]);

		let mut stack = vec![vec![0, 1, 2], vec![3, 4, 5]];
		union(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0, 1, 2, 3, 4, 5]);

		let mut stack = vec![vec![0, 5, 1], vec![4, 2, 3]];
		union(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0, 5, 1, 4, 2, 3]);
	}

    #[test]
    fn subject_tests() {
		let sets = vec![
			vec![0, 1, 2],
		];
		assert_eq!(eval_set("A!", sets), vec![]);
		
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

    }

}
