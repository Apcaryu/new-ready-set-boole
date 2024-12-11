pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
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
