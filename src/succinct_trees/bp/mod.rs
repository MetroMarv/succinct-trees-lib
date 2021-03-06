use bv::{BitVec, Bits};
use std::fmt;
use super::SuccinctTreeFunctions;
use bio::data_structures::rank_select::RankSelect;
use serde::{Serialize, Serializer, Deserialize, Deserializer};

mod rmm;
use self::rmm::{RangeMinMaxTree};


pub struct BalancedParenthesis {
    blocksize: u64,
    range_min_max_tree: RangeMinMaxTree,
    rank_select: RankSelect
}


impl fmt::Display for BalancedParenthesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parenthesis_expression = String::from("");
        for i in 0..self.range_min_max_tree.parenthesis.len()-1 {
            let bit = self.range_min_max_tree.parenthesis.get_bit(i);

            if bit {
                parenthesis_expression.push_str("(");
            } else {
                parenthesis_expression.push_str(")");
            }
        }
        write!(f, "BP-Tree: {}", parenthesis_expression)
    }
}


impl Serialize for BalancedParenthesis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.get_parenthesis().serialize(serializer)
    }
}


impl<'de> Deserialize<'de> for BalancedParenthesis {
    fn deserialize<D>(deserializer: D) -> Result<BalancedParenthesis, D::Error>
        where
            D: Deserializer<'de>,
    {
        let parenthesis = BitVec::deserialize(deserializer)?;

        let length_f = parenthesis.len() as f64;
        let blocksize = (length_f.log2().powi(2) / 32.0).ceil() as usize;

        Ok(BalancedParenthesis::new(parenthesis, blocksize as u64))
    }
}


impl BalancedParenthesis {
    pub fn new_with_fixed_blocksize(parenthesis: BitVec<u8>) -> BalancedParenthesis {
        let bp = BalancedParenthesis::new(parenthesis, 2);
        bp
    }

    pub fn new(parenthesis: BitVec<u8>, blocksize: u64) -> BalancedParenthesis {
        let rank_select = RankSelect::new(parenthesis.clone(), blocksize.clone() as usize);
        let range_min_max_tree = RangeMinMaxTree::new(parenthesis, blocksize);

        BalancedParenthesis{ blocksize ,range_min_max_tree, rank_select}
    }

    pub fn get_parenthesis(&self) -> &BitVec<u8>{
        &self.range_min_max_tree.parenthesis
    }

    pub(crate) fn excess(&self, position: u64) -> u64 {
        let rank_1 = self.rank_select.rank_1(position).unwrap();
        let rank_0 = self.rank_select.rank_0(position).unwrap();

        rank_1 - rank_0
    }

    fn find_close(&self, node: u64) -> u64 {
        // RMM tree is 1 base so -1 :)
        self.range_min_max_tree.fwdsearch(node +1, -1) -1
    }
}


impl SuccinctTreeFunctions for BalancedParenthesis{
    fn has_index(&self, index:u64) -> bool {
      index < self.range_min_max_tree.parenthesis.len()
    }

    fn is_leaf(&self, _lf:u64) -> bool{
        if self.range_min_max_tree.parenthesis.get_bit(_lf) {
            !self.range_min_max_tree.parenthesis.get_bit(_lf + 1)
        } else {
            self.range_min_max_tree.parenthesis.get_bit(_lf - 1)
        }
    }

    fn first_child(&self,_lf:u64) -> Option<u64>{
        if !self.is_leaf(_lf){
            return Some(_lf + 1);
        }
        return None;
    }

    fn next_sibling(&self,node:u64) -> Option<u64>{
        // TODO: check if has sibling!!
        Some(self.find_close(node) +1)
    }

    fn parent(&self,_lf:u64) -> u64{
        println!("Parenthesis: {}", self.get_parenthesis().get_bit(_lf));
        self.enclose(_lf)
    }
    fn rank(&self,_lf:u64) -> u64{
        self.rank_select.rank_1(_lf).unwrap()
    }
    fn select(&self,_lf:u64) -> u64{
        self.rank_select.select_1(_lf).unwrap()
    }
    fn close_rank(&self,_lf:u64) -> u64{
        self.rank_select.rank_0(_lf).unwrap()
    }
    fn close_select(&self,_lf:u64) -> u64{
        self.rank_select.select_0(_lf).unwrap()
    }
    fn enclose(&self,_lf:u64) -> u64{
        self.range_min_max_tree.bwdsearch(_lf, -2) + 1
    }
    fn subtree_size(&self,node:u64) -> u64{
        ((self.find_close(node)- node + 1) / 2) -1
    }
    fn ancestor(&self,_lf:u64, _lf2:u64) -> bool{
        _lf <= _lf2 && _lf2 < self.find_close(_lf)
    }
    fn child(&self,_lf:u64, _lf2:u64) -> Option<u64>{
        unimplemented!();
    }
    fn lca(&self,_lf:u64, _lf2:u64) -> u64{
        unimplemented!();
    }
    fn level_ancestor(&self,node:u64, d:u64) -> u64 {
        /* This is slow version determining parent x times.
           Faster would be to call bwdsearch(_lf -1, -(_lf2 -1)) */

        //self.range_min_max_tree.bwdsearch(node +1 , -(d as i64)-1)
        let mut result = node;

        for i in 0..d {
            result = self.parent(result);
        }

        result
    }

    fn degree(&self,_lf:u64) -> u64{
        unimplemented!();
    }

    fn depth(&self,_lf:u64) -> u64{
        self.excess(_lf) as u64
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use bincode::{serialize, deserialize };
    use bv::Bits;
    use succinct_trees::SuccinctTreeFunctions;


    pub fn example_tree() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, false];
        return BalancedParenthesis::new(parenthesis, 2);
    }

    pub fn example_tree_incomplete() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, true, false, true, false, false];
        return BalancedParenthesis::new(parenthesis, 2);
    }

    pub fn example_tree_big() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false];
        return BalancedParenthesis::new(parenthesis, 4);
    }

    pub fn example_tree_bigger() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, true, false, true, false, false, false, true, true, false, true, false, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false, false];
        return BalancedParenthesis::new(parenthesis, 8);
    }

    pub fn empty_tree() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![];
        return BalancedParenthesis::new(parenthesis, 2);
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
        print!("SERIALIZED: {:?}", serialized);

        let expected: Vec<u8> = vec![2, 0, 0, 0, 0, 0, 0, 0, 23, 0, 8, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialization () {
        let serialized = [2, 0, 0, 0, 0, 0, 0, 0, 23, 0, 8, 0, 0, 0, 0, 0, 0, 0];

        let result: BalancedParenthesis = deserialize(&serialized).unwrap();

        let tree = example_tree();
        let expected = tree.get_parenthesis();
        assert_eq!(result.get_parenthesis(), expected);
    }

    #[test]
    fn test_rank() {
        let tree = example_tree();

        assert_eq!(tree.rank(4), 4);
        assert_eq!(tree.rank(5), 4);
    }

    #[test]
    fn test_rank_close() {
        let tree = example_tree();

        assert_eq!(tree.close_rank(4), 1);
        assert_eq!(tree.close_rank(5), 2);
        assert_eq!(tree.close_rank(7), 4);
    }

    #[test]
    fn test_select() {
        let tree = example_tree();

        assert_eq!(tree.select(3), 2);
        assert_eq!(tree.select(1), 0);
        assert_eq!(tree.select(4), 4);
    }

    #[test]
    fn test_close_select() {
        let tree = example_tree();

        assert_eq!(tree.close_select(1), 3);
        assert_eq!(tree.close_select(4), 7);
        assert_eq!(tree.close_select(2), 5);
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
        assert_eq!(example_tree().next_sibling(2).unwrap(), 4);
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
    fn test_parent_root(){
        assert_eq!(example_tree().parent(1), 0);
    }

    #[test]
    fn test_parent(){
        assert_eq!(example_tree().parent(2), 1);
        assert_eq!(example_tree().parent(4), 1);
    }

    #[test]
    fn test_subtree_size(){
        assert_eq!(example_tree().subtree_size(1), 2);
        assert_eq!(example_tree().subtree_size(0), 3);
        assert_eq!(example_tree().subtree_size(4), 0);
    }

    #[test]
    #[should_panic]
    fn test_subtree_size_empty(){
        empty_tree().subtree_size(0);
    }

    #[test]
    fn test_ancestor(){
        let tree = example_tree();

        assert_eq!(tree.ancestor(0, 1), true);
        assert_eq!(tree.ancestor(2, 4), false);
    }

    #[test]
    #[should_panic]
    fn test_ancestor_empty(){
        empty_tree().ancestor(0,1);
    }

    #[test]
    fn test_level_ancestor(){
        assert_eq!(example_tree().level_ancestor(4,1), 1);
        assert_eq!(example_tree().level_ancestor(2,1), 1);
    }

    #[test]
    #[should_panic]
    fn test_level_ancestor_empty(){
        empty_tree().level_ancestor(0,1);
    }

    #[test]
    fn test_depth(){
        assert_eq!(example_tree().depth(0),1);
        assert_eq!(example_tree().depth(2),3);
    }

    #[test]
    #[should_panic]
    fn test_depth_empty(){
        empty_tree().depth(0);
    }

    #[test]
    fn test_enclose(){
        assert_eq!(example_tree().enclose(2),1);
        assert_eq!(example_tree().enclose(4),1);
    }

    #[test]
    #[should_panic]
    fn test_enclose_empty(){
        empty_tree().enclose(0);
    }

    #[test]
    fn  test_construct_rmm_tree() {
        let tree = example_tree();
        let range_min_max_tree = tree.range_min_max_tree;

        // test excess
        let vec_exc = vec![0, 0, 2, -2, 2, 0, 0, -2];
        assert_eq!(*range_min_max_tree.get_excess(), vec_exc);

        //test minimum
        let vec_min = vec![0, 0, 1, -2, 1, 0, 0, -2];
        assert_eq!(*range_min_max_tree.get_minimum(), vec_min);

        //test maximum
        let vec_max = vec![0, 3, 3, 1, 2, 1, 1, -1];
        assert_eq!(*range_min_max_tree.get_maximum(), vec_max);

        //test quantity
        let vec_qty = vec![0, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(*range_min_max_tree.get_quantity(), vec_qty);
    }

    #[test]
    fn  test_construct_rmm_tree_incomplete() {
        let tree = example_tree_incomplete();
        let range_min_max_tree = tree.range_min_max_tree;

        // test excess
        let vec_exc = vec![0, 0, 2, -2, 2, 0, -2, 0, 2, 0, 0, 0, 0, -2, 0, 0];
        assert_eq!(*range_min_max_tree.get_excess(), vec_exc);

        //test minimum
        let vec_min = vec![0, 0, 1, -2, 1, -1, -2, 0, 1, 0, 0, -1, -1, -2, 0, 0];
        assert_eq!(*range_min_max_tree.get_minimum(), vec_min);

        //test maximum
        let vec_max = vec![0, 3, 3, 0, 3, 1, 0, 0, 2, 1, 1, 0, 0, -1, 0, 0];
        assert_eq!(*range_min_max_tree.get_maximum(), vec_max);

        //test quantity
        let vec_qty = vec![0, 1, 2, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0];
        assert_eq!(*range_min_max_tree.get_quantity(), vec_qty);
    }

    #[test]
    fn  test_construct_rmm_tree_big() {
        let tree = example_tree_big();
        let range_min_max_tree = tree.range_min_max_tree;

        // test excess
        let vec_exc = vec![0, 0, 2, -2, 2, 0, 0, -2];
        assert_eq!(*range_min_max_tree.get_excess(), vec_exc);

        //test minimum
        let vec_min = vec![0, 0, 1, -2, 1, -1, -1, -2];
        assert_eq!(*range_min_max_tree.get_minimum(), vec_min);

        //test maximum
        let vec_max = vec![0, 3, 3, 1, 3, 1, 1, 1];
        assert_eq!(*range_min_max_tree.get_maximum(), vec_max);

        //test quantity
        let vec_qty = vec![0, 1, 2, 1, 1, 1, 1, 1];
        assert_eq!(*range_min_max_tree.get_quantity(), vec_qty);
    }

    #[test]
    fn  test_construct_rmm_tree_bigger() {
        let tree = example_tree_bigger();
        let range_min_max_tree = tree.range_min_max_tree;

        // test excess
        let vec_exc = vec![0, 0, 2, -2, 2, 0, 0, -2];
        assert_eq!(*range_min_max_tree.get_excess(), vec_exc);

        //test minimum
        let vec_min = vec![0, 0, 1, -2, 1, -1, 0, -2];
        assert_eq!(*range_min_max_tree.get_minimum(), vec_min);

        //test maximum
        let vec_max = vec![0, 4, 4, 2, 4, 1, 2, 2];
        assert_eq!(*range_min_max_tree.get_maximum(), vec_max);

        //test quantity
        let vec_qty = vec![0, 1, 3, 1, 1, 2, 2, 1];
        assert_eq!(*range_min_max_tree.get_quantity(), vec_qty);
    }
}