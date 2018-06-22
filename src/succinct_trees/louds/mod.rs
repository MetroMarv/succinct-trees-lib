use bv::{BitVec, Bits};
use std::fmt;
use super::SuccinctTreeFunctions;
use bio::data_structures::rank_select::RankSelect;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::ser::SerializeSeq;

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

        if !self.parenthesis[node] {
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

impl Serialize for Louds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.parenthesis.serialize(serializer)
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