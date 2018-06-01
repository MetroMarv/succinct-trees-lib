use bv::BitVec;

#[derive(Debug, Serialize, Deserialize)]
pub struct Louds {
    parenthesis: BitVec
    /* For fields added in future please add
     * #[serde(skip_deserializing,skip_serializing)]
     * annotation. So it's not (de)serialized.
     */
}

impl Louds {
    pub fn new(parenthesis: BitVec) -> Louds {
        Louds{parenthesis}
    }

    pub fn get_parenthesis(&self) -> &BitVec {
        &self.parenthesis
    }
}