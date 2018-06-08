use bv::BitVec;
use std::fmt;
use super::SuccinctTreeFunctions;

#[derive(Debug, Serialize, Deserialize)]
pub struct BalancedParenthesis {
    parenthesis: BitVec
    /* For fields added in future please add
     * #[serde(skip_deserializing,skip_serializing)]
     * annotation. So it's not (de)serialized.
     */
}

impl SuccinctTreeFunctions for BalancedParenthesis{

    fn is_leaf(_lf:i32) -> bool{
        unimplemented!();
    }
    fn first_child(_lf:i32) -> i32{
        unimplemented!();
    }
    fn next_sibling(_lf:i32) -> i32{
        unimplemented!();
    }
    fn parent(_lf:i32) -> i32{
        unimplemented!();
    }
    fn rank(_lf:i32) -> i32{
        unimplemented!();
    }
    fn select(_lf:i32) -> i32{
        unimplemented!();
    }
    fn close_rank(_lf:i32) -> i32{
        unimplemented!();
    }
    fn close_select(_lf:i32) -> i32{
        unimplemented!();
    }
    fn enclose(_lf:i32) -> i32{
        unimplemented!();
    }
    fn subtree_size(_lf:i32) -> i32{
        unimplemented!();
    }
    fn pre_rank(_lf:i32) -> i32{
        unimplemented!();
    }
    fn ancestor(_lf:i32, _lf2:i32) -> bool{
        unimplemented!();
    }
    fn child(_lf:i32, _lf2:i32) -> i32{
        unimplemented!();
    }
    fn lca(_lf:i32, _lf2:i32) -> i32{
        unimplemented!();
    }
    fn level_ancestor(_lf:i32, _lf2:i32) -> i32{
        unimplemented!();
    }
    fn degree(_lf:i32) -> i32{
        unimplemented!();
    }
    fn depth(_lf:i32) -> i32{
        unimplemented!();
    }

}


impl BalancedParenthesis {
    pub fn new(parenthesis: BitVec) -> BalancedParenthesis {
        BalancedParenthesis{parenthesis}
    }

    pub fn get_parenthesis(&self) -> &BitVec{
        &self.parenthesis
    }
}

impl fmt::Display for BalancedParenthesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parenthesis_expression = String::from("");
        for bit in &self.parenthesis {
            if bit {
                parenthesis_expression.push_str("(");
            } else {
                parenthesis_expression.push_str(")");
            }
        }
        write!(f, "BP-Tree: {}", parenthesis_expression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::{serialize, deserialize, Result };
    use bv::Bits;
    use succinct_trees;
    use bit_vec::BitVec;

    #[test]
    fn test_tree() {
        let parenthesis: BitVec = BitVec::from_bytes(&[0b11101000]);
        let tree = succinct_trees::bp::BalancedParenthesis::new(parenthesis);
        println!("{}",tree);
        assert_eq!(tree.get_parenthesis().get(3), Some(false));
    }

    #[test]
    fn test_serialization () {
        let parenthesis: BitVec= bit_vec![true, true, true, false, true, false, false, false];
        let tree = BalancedParenthesis::new(parenthesis);

        let serialized = serialize(&tree).unwrap();

        let deserialized: Result<BalancedParenthesis> = deserialize(&serialized[..]);
        println!("{:?}", deserialized);
        assert_eq!(tree.get_parenthesis().get_bit(3), false)
    }
}