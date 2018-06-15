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
    use super::*;
    use bincode::{serialize, deserialize, Result };
    use bv::Bits;
    use succinct_trees::SuccinctTreeFunctions;


    pub fn example_tree() -> Louds{
        let parenthesis: BitVec = bit_vec![true, true, false, true, true, false, false, false];
        return Louds::new(parenthesis);
    }

    pub fn empty_tree() -> Louds{
        let parenthesis: BitVec = bit_vec![];
        return Louds::new(parenthesis);
    }





    #[test]
    fn test_constructor() {
        let tree = example_tree();
        println!("{}",tree);
        assert_eq!(tree.get_parenthesis().get_bit(3), false);
    }

    #[test]
    fn test_serialization () {
        let parenthesis: BitVec= bit_vec![true, true, true, false, true, false, false, false];
        let tree = Louds::new(parenthesis);

        let serialized = serialize(&tree).unwrap();

        let deserialized: Result<Louds> = deserialize(&serialized[..]);
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
        empty_tree().get_parenthesis().get_bit(0);
    }


    #[test]
    #[should_panic]
    fn test_first_child_empty(){
        empty_tree().first_child(0);
    }

    #[test]
    fn test_first_child(){
        assert_eq!(example_tree().first_child(0),3);
    }


    #[test]
    fn test_next_sibling(){
        assert_eq!(example_tree().next_sibling(6), 7);
    }

    #[test]
    #[should_panic]
    fn test_next_sibling_empty(){
        empty_tree().next_sibling(6);
    }

    #[test]
    #[should_panic]
    fn test_parent_empty(){
        empty_tree().parent(0);
    }

    #[test]
    fn test_parent(){
        assert_eq!(example_tree().parent(3), 0)
    }

    #[test]
    fn test_subtree_size(){
        assert_eq!(example_tree().subtree_size(0), 2)
    }

    #[test]
    #[should_panic]
    fn test_subtree_size_empty(){
        empty_tree().subtree_size(0);
    }

    #[test]
    fn test_ancestor(){
        assert_eq!(example_tree().ancestor(6,7),true);
        assert_eq!(example_tree().ancestor(3,7),false);
    }

    #[test]
    #[should_panic]
    fn test_ancestor_empty(){
        empty_tree().ancestor(0,1);
    }

    #[test]
    fn test_level_ancestor(){
        assert_eq!(example_tree().level_ancestor(6,2), 0);
    }

    #[test]
    #[should_panic]
    fn test_level_ancestor_empty(){
        empty_tree().level_ancestor(0,1);
    }

    #[test]
    fn test_lca(){
        assert_eq!(example_tree().lca(6,7),3);
    }

    #[test]
    #[should_panic]
    fn test_lca_empty(){
        empty_tree().lca(0,1);
    }

    #[test]
    fn test_child(){
        assert_eq!(example_tree().child(3, 2),7);
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
        assert_eq!(example_tree().degree(3),2);
    }

    #[test]
    #[should_panic]
    fn test_degree_empty(){
        empty_tree().degree(0);
    }

    #[test]
    fn test_enclose(){
        assert_eq!(example_tree().enclose(0),1);
    }

    #[test]
    #[should_panic]
    fn test_enclose_empty(){
        empty_tree().enclose(0);
    }

}