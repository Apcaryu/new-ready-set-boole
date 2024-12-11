// this function assumed that the set passed as parameter is valid (no duplicate elements)
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
	let mut out: Vec<Vec<i32>> = Vec::new();

	out.push(vec![]);
	
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_test() {
        assert_eq!(powerset(vec![]), vec![vec![]]);
		assert_eq!(powerset(vec![1]), vec![vec![], vec![1]]);
		assert_eq!(powerset(vec![1, 2]), vec![vec![], vec![1], vec![2], vec![1, 2]]);
		assert_eq!(powerset(vec![1, 2, 3]), vec![vec![], vec![1], vec![2], vec![3], vec![1, 2], vec![1, 3], vec![2, 3], vec![1, 2, 3]]);
    }

	#[test]
	fn size_test() {
		assert_eq!(powerset(vec![1, 2, 3, 4]).len(), 16);
		assert_eq!(powerset(vec![1, 2, 3, 4, 5]).len(), 32);
		assert_eq!(powerset(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).len(), 1024);
	}
}
