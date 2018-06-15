use bv::{BitVec, Bits};
use std::fmt;
use super::SuccinctTreeFunctions;
use bio::data_structures::rank_select::RankSelect;

pub struct Louds {
    parenthesis: BitVec<u8>,
    /* For fields added in future please add
     * #[serde(skip_deserializing,skip_serializing)]
     * annotation. So it's not (de)serialized.
     */
    rank_select: RankSelect
}

impl Louds {
    pub fn new(parenthesis: BitVec<u8>) -> Louds {
        // TODO: calculate block size floor(log(n)^2/32)
        let rank_select = RankSelect::new(parenthesis.clone(), 4);

        Louds{parenthesis, rank_select}
    }

    pub fn get_parenthesis(&self) -> &BitVec<u8> {
        &self.parenthesis
    }

    fn prev_0 (&self, index: u64) -> u64 {
        assert!(self.has_index(index));

        let message = String::from(format!("Could not determine rank_0 from index {}", index));
        let rank_0 = self.rank_select.rank_0(index).expect(&message);

        let message = String::from(format!("Couldn't determine select_0 from index {}", rank_0));
        self.rank_select.select_0(rank_0).expect(&message)
    }

    fn next_0 (&self, index: u64) -> u64 {
        assert!(self.has_index(index));

        let message = String::from(format!("Could not determine rank_0 from index {}", index));
        let rank_0 = self.rank_select.rank_0(index).expect(&message);
        let rank = rank_0 + 1;

        let message = String::from(format!("Couldn't determine select_0 from index {}", rank));
        self.rank_select.select_0(rank).expect(&message)
    }

    fn child_count (&self, node: u64) -> u64 {
        assert!(self.has_index(node));

        let rank0_until_node = self.rank_select.rank_0(node-1).unwrap();
        let rank1_until_node = self.rank_select.rank_1(node -1).unwrap();

        let index_node_over = self.rank_select.select_0(rank0_until_node +1).unwrap();
        let rank1_after_node = self.rank_select.rank_1(index_node_over).unwrap();

        rank1_after_node - rank1_until_node
    }
}

impl SuccinctTreeFunctions for Louds{
    fn has_index(&self, index:u64) -> bool {
        index < self.parenthesis.len()
    }

    fn is_leaf(&self, node:u64) -> bool{
        assert!(self.has_index(node));

        self.parenthesis.get_bit(node) == false
    }

    fn first_child(&self, node:u64) -> Option<u64>{
        self.child(node, 0)
    }

    fn next_sibling(&self, node:u64) -> u64{
        assert!(self.has_index(node));

        let y = self.rank_select.rank_0(node -1).unwrap() + 1;

        let inner = self.rank_select.select_1(y).unwrap() + 1;
        let message = format!("Couldn't determine select_0 from index {}", inner);
        self.rank_select.select_0(inner).expect(&message)
    }

    fn parent(&self, node:u64) -> u64{
        assert!(node > 0);
        assert!(self.has_index(node));

        let rank = self.rank_select.rank_0(node).unwrap();
        let select_1 = self.rank_select.select_1(rank).unwrap();

        self.prev_0(select_1) + 1
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

    fn child(&self, node:u64, index:u64) -> Option<u64>{
        assert!(self.has_index(node));

        if self.child_count(node) < index +1  {
            return None;
        }

        let message = String::from(format!("Couldn't determine rank_1 of index {}", node));
        let rank_1 = self.rank_select.rank_1(node).expect(&message) -1 ;

        let message = String::from(format!("Couldn't determine select_0 of index {}", rank_1 + index));
        let select = self.rank_select.select_0(rank_1 + index).expect(&message);

        Some(select + 1)
    }

    fn lca(&self,_lf:u64, _lf2:u64) -> u64{
        unimplemented!();
    }

    fn level_ancestor(&self,_lf:u64, _lf2:u64) -> u64{
        unimplemented!();
    }

    fn degree(&self, node:u64) -> u64{
        assert!(self.has_index(node));

        self.next_0(node) - node
    }

    fn depth(&self,_lf:u64) -> u64{
        unimplemented!();
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
        let parenthesis: BitVec<u8> = bit_vec![true, true, false, true, true, false, false, false];
        return Louds::new(parenthesis);
    }

    pub fn empty_tree() -> Louds{
        let parenthesis: BitVec<u8> = bit_vec![];
        return Louds::new(parenthesis);
    }

    #[test]
    fn test_constructor() {
        let tree = example_tree();

        assert_eq!(tree.get_parenthesis().get_bit(3), false);
    }

//    #[test]
//    fn test_serialization () {
//        let parenthesis: BitVec<u8>= bit_vec![true, true, true, false, true, false, false, false];
//        let tree = Louds::new(parenthesis);
//
//        let serialized = serialize(&tree).unwrap();
//
//        let deserialized: Result<Louds> = deserialize(&serialized[..]).unwrap();
//
//        assert_eq!(deserialized.get_parenthesis().get_bit(3), false)
//    }

    #[test]
    fn test_child_count () {
        assert_eq!(example_tree().child_count(1), 1);
        assert_eq!(example_tree().child_count(3), 2);
        assert_eq!(example_tree().child_count(6), 0);
    }

    #[test]
    fn test_is_leaf(){
        assert_eq!(example_tree().is_leaf(0), false);
        assert_eq!(example_tree().is_leaf(6), true);
    }

    #[test]
    #[should_panic]
    fn test_is_leaf_empty() {
        empty_tree().is_leaf(0);
    }


    #[test]
    #[should_panic]
    fn test_first_child_empty(){
        empty_tree().first_child(0);
    }

    #[test]
    fn test_first_child(){
        assert_eq!(example_tree().first_child(1),Some(3));
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
        assert_eq!(example_tree().parent(4), 1);
    }

    #[test]
    #[should_panic]
    fn test_parent_root () {
        example_tree().parent(0);
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
    fn test_child() {
        assert_eq!(example_tree().child(3, 1),Some(7));
    }

    #[test]
    fn test_child_root () {
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, true, false, false, false, false];
        let tree = Louds::new(parenthesis);

        assert_eq!(tree.child(1, 0), Some(4));
        assert_eq!(tree.child(1, 1), Some(7));
    }

    #[test]
    #[should_panic]
    fn test_child_empty(){
        empty_tree().child(0,1);
    }

    #[test]
    fn test_child_non_existing (){
        assert_eq!(example_tree().child(1, 2), None);
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