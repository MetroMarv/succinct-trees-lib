use bit_vec::BitVec;
use std::fmt;

pub struct Louds {
    parenthesis: BitVec
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
        for bit in &self.parenthesis {
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
    use bit_vec::BitVec;

    #[test]
    fn test_tree() {
        let parenthesis: BitVec = BitVec::from_bytes(&[0b11101000]);
        let tree = succinct_trees::bp::BalancedParenthesis::new(parenthesis);
        println!("{}",tree);
        assert_eq!(tree.get_parenthesis().get(3), Some(false));
    }
}