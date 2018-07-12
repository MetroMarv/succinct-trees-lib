use bv::{BitVec, Bits};
use std::fmt;
use super::SuccinctTreeFunctions;
use bio::data_structures::rank_select::RankSelect;
use serde::{Serialize, Serializer, Deserialize, Deserializer};

pub struct Louds {
    rank_select: RankSelect
}

impl Louds {
    pub fn new(parenthesis: BitVec<u8>) -> Louds {
        let length_f = parenthesis.len() as f64;
        let blocksize = (length_f.log2().powi(2) / 32.0).ceil() as usize;

        Louds::new_blocksize(parenthesis, blocksize)
    }

    pub fn new_blocksize (parenthesis: BitVec<u8>, blocksize: usize) -> Louds {
        let rank_select = RankSelect::new(parenthesis.clone(), blocksize);

        Louds{rank_select}
    }

    pub fn get_parenthesis(&self) -> &BitVec<u8> {
        &self.rank_select.bits()
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

    fn get_representing_true(&self, node: u64) -> u64 {
        assert!(node > 0);
        assert!(self.has_index(node));

        let rank = self.rank_select.rank_0(node - 1).unwrap();
        println!("Rank: {}", rank);
        self.rank_select.select_1(rank +1).unwrap()
    }

    fn has_next_sibling(&self, node: u64 ) -> bool {
        assert!(node > 0);
        assert!(self.has_index(node));

        if node == 1 {
            return false;
        }

        let rep_true_index = self.get_representing_true(node);

        self.rank_select.bits().get_bit(rep_true_index + 1)
    }
}

impl SuccinctTreeFunctions for Louds{
    fn has_index(&self, index:u64) -> bool {
        index < self.get_parenthesis().len()
    }

    fn is_leaf(&self, node:u64) -> bool{
        assert!(self.has_index(node));

        self.get_parenthesis().get_bit(node) == false
    }

    fn first_child(&self, node:u64) -> Option<u64>{
        self.child(node, 0)
    }

    fn next_sibling(&self, node:u64) -> Option<u64>{
        assert!(self.has_index(node));

        if !self.has_next_sibling(node) {
            return None;
        }

        let y = self.rank_select.rank_0(node -1).unwrap() + 1;

        let inner = self.rank_select.rank_1(self.rank_select.select_1(y).unwrap()).unwrap() + 1;

        let message = format!("Couldn't determine select_0 from index {}", inner);
        Some(self.rank_select.select_0(inner).expect(&message))
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

    fn ancestor(&self,_lf:u64, _lf2:u64) -> bool{
        unimplemented!();
    }

    fn child(&self, node:u64, index:u64) -> Option<u64>{
        assert!(self.has_index(node));

        if self.degree(node) < index +1  {
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

        if !self.get_parenthesis()[node] {
            return 0;
        }

        self.next_0(node) - node
    }

    fn depth(&self,_lf:u64) -> u64{
        unimplemented!();
    }
}

impl fmt::Display for Louds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parenthesis_expression = String::from("");
        for i in 0..self.get_parenthesis().len()-1 {
            let bit = self.get_parenthesis().get_bit(i);

            if bit {
                parenthesis_expression.push_str("(");
            } else {
                parenthesis_expression.push_str(")");
            }
        }
        write!(f, "LOUDS-Tree: {}", parenthesis_expression)
    }
}

impl Serialize for Louds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.get_parenthesis().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Louds {
    fn deserialize<D>(deserializer: D) -> Result<Louds, D::Error>
        where
            D: Deserializer<'de>,
    {
        let parenthesis = BitVec::deserialize(deserializer)?;

        Ok(Louds::new(parenthesis))
    }
}

#[cfg(test)]
mod tests;