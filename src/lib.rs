extern crate bit_vec;

pub mod succinct_trees;

#[cfg(test)]
mod tests {
    use succinct_trees;

    use bit_vec::BitVec;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_tree() {
        let parenthesis: BitVec= BitVec::from_bytes(&[0b11101000]);
        let tree = succinct_trees::bp::BalancedParenthesis::new(parenthesis);

        assert_eq!(tree.get_parenthesis().get(3), Some(false));
    }
}
