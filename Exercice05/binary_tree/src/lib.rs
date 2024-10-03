#[derive(Debug, Clone, PartialEq)]
pub struct Tree(Option<Box<Node>>);

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
	value: char,
	left: Tree,
	right: Tree
}

impl Node {
	pub fn new(chr: char) -> Node {
		Node { value: chr, left: Tree(None), right: Tree(None) }
	}

	pub fn get_value(&self) -> char {
		self.value
	}
	pub fn get_left(&self) -> Tree {
		self.left.clone()
	}
	pub	fn get_right(&self) -> Tree {
		self.right.clone()
	}
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_test() {
        let node = Node::new('A');
		assert_eq!(node.value, 'A');
		assert_eq!(node.left, Tree(None));
		assert_eq!(node.right, Tree(None));
    }

	#[test]
	fn get_test() {
        let node = Node::new('A');
		assert_eq!(node.get_value(), 'A');
		assert_eq!(node.get_left(), Tree(None));
		assert_eq!(node.get_right(), Tree(None));
	}
}
