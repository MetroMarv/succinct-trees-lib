use bv::{BitVec, Bits};
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
        return false;
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
    use succinct_trees;


    pub fn example_tree() -> BalancedParenthesis{
        return succinct_trees::bp::BalancedParenthesis::new(BitVec::from_bytes(&[0b11101000]));
    }

    pub fn empty_tree() -> BalancedParenthesis{
        return succinct_trees::bp::BalancedParenthesis::new(BitVec::from_bytes(&[]));
    }





    #[test]
    fn test_tree() {
        let parenthesis: BitVec = bit_vec![true, true, true, false, true, false, false, false];
        let tree = succinct_trees::bp::BalancedParenthesis::new(parenthesis);
        println!("{}",tree);
        assert_eq!(tree.get_parenthesis().get_bit(3), false);
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
    #[test]
    fn test_is_leaf(){
        assert_eq!(example_tree().is_leaf(0), false);
        assert_eq!(example_tree().is_leaf(4), true);
    }

    #[test]
    #[should_panic]
    fn test_is_leaf_empty(){
        empty_tree().get_parenthesis().get(0);
    }
    

    #[test]
    #[should_panic]
    fn test_first_child_empty(){
        empty_tree().first_child(0);
    }

    #[test]
    fn test_first_child(){
        assert_eq!(example_tree().first_child(0),1);
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
        assert_eq!(example_tree().ancestor(0,1),0);
    }

    #[test]
    #[should_panic]
    fn test_ancestor_empty(){
        empty_tree().ancestor(0,1);
    }

    #[test]
    fn test_level_ancestor(){
        assert_eq!(empty_tree().level_ancestor(0,1), 0);
    }

    #[test]
    #[should_panic]
    fn test_level_ancestor_empty(){
        empty_tree().level_ancestor(0,1);
    }

    #[test]
    fn test_lca(){
        assert_eq!(empty_tree().lca(0,1),0);
    }

    #[test]
    #[should_panic]
    fn test_lca_empty(){
        empty_tree().lca(0,1);
    }

    #[test]
    fn test_child(){
        assert_eq!(empty_tree().child(0),1);
    }

    #[test]
    #[should_panic]
    fn test_child_empty(){
        empty_tree().child(0,1);
    }

    #[test]
    fn test_depth(){
        assert_eq!(empty_tree().depth(0),2);
    }

    #[test]
    #[should_panic]
    fn test_depth_empty(){
        empty_tree().depth(0);
    }

    #[test]
    fn test_degree(){
        assert_eq!(empty_tree().ancestor(0),0);
    }

    #[test]
    #[should_panic]
    fn test_degree_empty(){
        empty_tree().degree(0);
    }

    #[test]
    fn test_enclose(){
        assert_eq!(empty_tree().ancestor(0),0);
    }

    #[test]
    #[should_panic]
    fn test_enclose_empty(){
        empty_tree().enclose(0);
    }

}