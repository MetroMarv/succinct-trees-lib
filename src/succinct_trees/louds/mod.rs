use bv::{BitVec, Bits};
use std::fmt;
use super::SuccinctTreeFunctions;

#[derive(Debug, Serialize, Deserialize)]
pub struct Louds {
    parenthesis: BitVec
    /* For fields added in future please add
     * #[serde(skip_deserializing,skip_serializing)]
     * annotation. So it's not (de)serialized.
     */
}

impl SuccinctTreeFunctions for Louds{

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

impl Louds {
    pub fn new(parenthesis: BitVec) -> Louds {
        Louds{parenthesis}
    }

    pub fn get_parenthesis(&self) -> &BitVec {
        &self.parenthesis
    }
}

impl fmt::Display for Louds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parenthesis_expression = String::from("");
        for i in 0..self.parenthesis.len()-1 {
            let bit = self.parenthesis.get_bit(i);

            if bit {
                parenthesis_expression.push_str("(");
            } else {
                parenthesis_expression.push_str(")");
            }
        }
        write!(f, "LOUDS-Tree: {}", parenthesis_expression)
    }
}

#[cfg(test)]
mod tests {
    use succinct_trees;
    use bv::Bits;

    #[test]
    fn test_tree() {
        let parenthesis = bit_vec!(true, true, true, false, true, false, false, false);
        let tree = succinct_trees::bp::BalancedParenthesis::new(parenthesis);
        println!("{}",tree);
        assert_eq!(tree.get_parenthesis().get_bit(3), false);
    }
}