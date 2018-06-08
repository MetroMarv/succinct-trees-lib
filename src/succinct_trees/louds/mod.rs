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

    fn is_leaf(&self, _lf:u64) -> bool{
        unimplemented!();
    }
    fn first_child(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn next_sibling(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn parent(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn rank(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn select(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn close_rank(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn close_select(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn enclose(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn subtree_size(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn pre_rank(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn ancestor(&self,_lf:u64, _lf2:u64) -> bool{
        unimplemented!();
    }
    fn child(&self,_lf:u64, _lf2:u64) -> u64{
        unimplemented!();
    }
    fn lca(&self,_lf:u64, _lf2:u64) -> u64{
        unimplemented!();
    }
    fn level_ancestor(&self,_lf:u64, _lf2:u64) -> u64{
        unimplemented!();
    }
    fn degree(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn depth(&self,_lf:u64) -> u64{
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
    fn test_constructor() {
        let parenthesis = bit_vec!(true, true, true, false, true, false, false, false);
        let tree = succinct_trees::bp::BalancedParenthesis::new(parenthesis);

        assert_eq!(tree.get_parenthesis().get_bit(3), false);
    }
}