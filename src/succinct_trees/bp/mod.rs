use std::fmt;
use bv::{BitVec, Bits};
use serde::ser::{Serialize, Serializer, SerializeStruct, SerializeSeq};
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::result::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct BalancedParenthesis {
    parenthesis: BitVec
    /* For fields added in future please add
     * #[serde(skip_deserializing,skip_serializing)]
     * annotation. So it's not (de)serialized.
     */
}

impl BalancedParenthesis {
    pub fn new(parenthesis: BitVec) -> BalancedParenthesis {
        BalancedParenthesis{parenthesis}
    }

    pub fn get_parenthesis(&self) -> &BitVec{
        &self.parenthesis
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use bincode::{serialize, deserialize, Result };
    use std::fs::File;
    use std::io::prelude::*;

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