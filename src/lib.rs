#[macro_use]
extern crate bv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate bio;

pub mod succinct_trees;

#[cfg(test)]
mod tests {
    use succinct_trees;

    use bv::{Bits,BitVec};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_tree() {
        let parenthesis: BitVec<u8>= bit_vec![true, true, true, false, true, false, false, false];
        let tree = succinct_trees::bp::BalancedParenthesis::new(parenthesis);

        assert_eq!(tree.get_parenthesis().get_bit(3), false);
    }
}
