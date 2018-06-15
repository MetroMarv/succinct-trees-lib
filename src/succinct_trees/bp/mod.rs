use bv::{BitVec, Bits};
use std::fmt;
use super::SuccinctTreeFunctions;

#[derive(Debug, Serialize, Deserialize)]
pub struct BalancedParenthesis {
    parenthesis: BitVec<u8>
    /* For fields added in future please add
     * #[serde(skip_deserializing,skip_serializing)]
     * annotation. So it's not (de)serialized.
     */
}

pub struct RangeMinMaxTree {
    excess: Vec<i64>,
    maximum: Vec<i64>,
    minimum: Vec<i64>,
    quantity: Vec<u64>,
}

impl SuccinctTreeFunctions for BalancedParenthesis{
    fn has_index(&self, index:u64) -> bool {
      index < self.parenthesis.len()
    }

    fn is_leaf(&self, _lf:u64) -> bool{
        if self.parenthesis.get_bit(_lf) {
            !self.parenthesis.get_bit(_lf + 1)
        } else {
            self.parenthesis.get_bit(_lf - 1)
        }
    }

    fn first_child(&self,_lf:u64) -> Option<u64>{
        if !self.is_leaf(_lf){
            return Some(_lf + 1);
        }
        return None;
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
    fn child(&self,_lf:u64, _lf2:u64) -> Option<u64>{
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
        self.excess(_lf)
    }

}


impl RangeMinMaxTree {
    
    pub fn new(tree: BalancedParenthesis, block_size: u64) -> RangeMinMaxTree {
        let len = ((2*tree.parenthesis.len())/block_size) as usize;
        let len_f = len as f64;

        // set vec length
        let mut excess = Vec::with_capacity(len);
        let mut maximum = Vec::with_capacity(len);
        let mut minimum = Vec::with_capacity(len);
        let mut quantity = Vec::with_capacity(len);

        let mut rmm = RangeMinMaxTree{excess, maximum, minimum, quantity};


        let mut row = 1;

        for i in 1..len_f.log2().floor() as u64 {
            let mut block_count = 0;
            let mut vec_count = 0;

            let mut exc :i64 = 0;
            let mut max = 0;
            let mut min = 0;
            let mut qty = 0;

            for j in 0..tree.parenthesis.len() - 1 {
                if tree.parenthesis.get_bit(j) {
                    exc += 1;
                } else {
                    exc -= 1;
                }

                if exc > max {
                    max = exc;
                }

                if exc < min {
                    min = exc;
                    qty = 1;
                } else if exc == min {
                    qty += 1;
                }

                if block_count <= block_size {
                    block_count += 1;
                } else {
                    rmm.excess.insert(len/(2_usize.pow(row)) + vec_count, exc);
                    rmm.minimum.insert(len/(2_usize.pow(row)) + vec_count, min);
                    rmm.maximum.insert(len/(2_usize.pow(row)) + vec_count, max);
                    rmm.quantity.insert(len/(2_usize.pow(row)) + vec_count, qty);

                    vec_count += 1;
                    block_count = 0;
                    exc = 0;
                    max = 0;
                    min = 0;
                    qty = 0;
                }
            }

            row += 1;

        }

        rmm

    }

    pub fn fwdsearch(_i: u64, _d: u64) {}

    pub fn bwdsearch(_i: u64, _d: u64) {}


}

impl BalancedParenthesis {
    pub fn new(parenthesis: BitVec<u8>) -> BalancedParenthesis {
        BalancedParenthesis{parenthesis}
    }

    pub fn get_parenthesis(&self) -> &BitVec<u8>{
        &self.parenthesis
    }

    pub(crate) fn excess(&self, position: u64) -> u64 {
        let mut count = 0;
        for i in 0..position {
            if self.parenthesis.get_bit(i) {
                count += 1;
            } else {
                count -= 1;
            }
        }
        count
    }
}

impl fmt::Display for BalancedParenthesis {
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
        write!(f, "BP-Tree: {}", parenthesis_expression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::{serialize, deserialize, Result };
    use bv::Bits;
    use succinct_trees::SuccinctTreeFunctions;


    pub fn example_tree() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, false];
        return BalancedParenthesis::new(parenthesis);
    }

    pub fn empty_tree() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![];
        return BalancedParenthesis::new(parenthesis);
    }

    #[test]
    fn test_constructor() {
        let tree = example_tree();
        println!("{}",tree);
        assert_eq!(tree.get_parenthesis().get_bit(3), false);
    }

    #[test]
    fn test_serialization () {
        let tree = example_tree();

        let serialized = serialize(&tree).unwrap();

        let deserialized: Result<BalancedParenthesis> = deserialize(&serialized[..]);

        assert_eq!(deserialized.unwrap().get_parenthesis().get_bit(3), false)
    }

    #[test]
    fn test_is_leaf() {
        assert_eq!(example_tree().is_leaf(0), false);
        assert_eq!(example_tree().is_leaf(4), true);
    }

    #[test]
    #[should_panic]
    fn test_is_leaf_empty(){
        empty_tree().get_parenthesis().get_bit(0);
    }
    

    #[test]
    #[should_panic]
    fn test_first_child_empty(){
        empty_tree().first_child(0);
    }

    #[test]
    fn test_first_child(){
        assert_eq!(example_tree().first_child(0),Some(1));
    }


    #[test]
    fn test_next_sibling(){
        assert_eq!(example_tree().next_sibling(2), 4);
    }

    #[test]
    #[should_panic]
    fn test_next_sibling_empty(){
        empty_tree().next_sibling(2);
    }

    #[test]
    #[should_panic]
    fn test_parent_empty(){
        empty_tree().parent(4);
    }

    #[test]
    fn test_parent(){
        assert_eq!(example_tree().parent(1), 0)
    }

    #[test]
    fn test_subtree_size(){
        assert_eq!(example_tree().subtree_size(1), 2)
    }

    #[test]
    #[should_panic]
    fn test_subtree_size_empty(){
        empty_tree().subtree_size(0);
    }

    #[test]
    fn test_ancestor(){
        assert_eq!(example_tree().ancestor(0,1),false);
    }

    #[test]
    #[should_panic]
    fn test_ancestor_empty(){
        empty_tree().ancestor(0,1);
    }

    #[test]
    fn test_level_ancestor(){
        assert_eq!(example_tree().level_ancestor(0,1), 0);
    }

    #[test]
    #[should_panic]
    fn test_level_ancestor_empty(){
        empty_tree().level_ancestor(0,1);
    }

    #[test]
    fn test_lca(){
        assert_eq!(example_tree().lca(0,1),0);
    }

    #[test]
    #[should_panic]
    fn test_lca_empty(){
        empty_tree().lca(0,1);
    }

    #[test]
    fn test_child(){
        assert_eq!(example_tree().child(0, 0),Some(1));
    }

    #[test]
    #[should_panic]
    fn test_child_empty(){
        empty_tree().child(0,1);
    }

    #[test]
    fn test_depth(){
        assert_eq!(example_tree().depth(0),2);
    }

    #[test]
    #[should_panic]
    fn test_depth_empty(){
        empty_tree().depth(0);
    }

    #[test]
    fn test_degree(){
        assert_eq!(example_tree().degree(0),0);
    }

    #[test]
    #[should_panic]
    fn test_degree_empty(){
        empty_tree().degree(0);
    }

    #[test]
    fn test_enclose(){
        // TODO: (MR) Expects u64 instead of bool. Put 0 as expected result now but is this correct?
        assert_eq!(example_tree().enclose(0),0);
    }

    #[test]
    #[should_panic]
    fn test_enclose_empty(){
        empty_tree().enclose(0);
    }

}