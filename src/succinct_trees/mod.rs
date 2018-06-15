pub mod bp;
pub mod louds;

pub trait SuccinctTreeFunctions{
    fn has_index(&self, _:u64) -> bool;
    fn is_leaf(&self, _:u64) -> bool;
    fn first_child(&self,_: u64) -> Option<u64>;
    fn next_sibling(&self,_: u64) -> u64;
    fn parent(&self,_: u64) -> u64;
    fn rank(&self,_: u64) -> u64;
    fn select(&self,_: u64) -> u64;
    fn close_rank(&self,_: u64) -> u64;
    fn close_select(&self,_: u64) -> u64;
    fn enclose(&self,_: u64) -> u64;
    fn subtree_size(&self,_: u64) -> u64;
    fn pre_rank(&self,_: u64) -> u64;
    fn ancestor(&self,_: u64, _: u64) -> bool;
    fn child(&self,_: u64,_: u64) -> Option<u64>;
    fn lca (&self,_: u64, _: u64) -> u64;
    fn level_ancestor(&self,_: u64, _: u64) -> u64;
    fn degree(&self,_: u64) -> u64;
    fn depth(&self,_: u64) -> u64;
}

pub mod parser {
    use std::fs::File;
    use std::io::{Read, Write};
    use succinct_trees::bp::BalancedParenthesis;
    use succinct_trees::louds::Louds;
    use bv::{BitVec, Bits};

    pub struct TreeParser {
        file: File
    }

    impl TreeParser {
        pub fn new(file: File) -> TreeParser {
            TreeParser {file}
        }

        fn read_file (&mut self) -> String {
            let mut content = String::new();
            let result = self.file.read_to_string(&mut content);

            match result {
                Ok(_) => content,
                Err(err) => panic!("Could not read string from file. {}", err)
            }
        }

        fn read_file_as_bitvec (&mut self) -> BitVec<u8> {
            let parenthesis = self.read_file();

            println!("Parenthesis: {}", parenthesis);

            let mut bitvec = BitVec::new();

            for c in parenthesis.chars() {
                if c == '(' {
                    bitvec.push(true);
                } else if c == ')' {
                    bitvec.push(false);
                } else {
                    panic!("Couldn't read BitVec from string '{}'. Contains invalid char '{}'.",
                           parenthesis, c);
                }
            }

            bitvec
        }

        pub fn read_bp (&mut self) -> BalancedParenthesis {
            let bitvec = self.read_file_as_bitvec();

            BalancedParenthesis::new(bitvec)
        }

        pub fn read_louds (&mut self) -> Louds {
            let bitvec = self.read_file_as_bitvec();

            Louds::new(bitvec)
        }

        fn write_bit (&mut self, bit: bool) {
            let parenthesis : String;

            if bit {
                parenthesis = String::from("(");
            } else {
                parenthesis = String::from(")");
            }

            let result = self.file.write(parenthesis.into_bytes().as_slice());

            match result {
                Ok(_) => (),
                Err(err) => panic!("Could not write to file. Error: {}", err)
            }
        }

        fn write_file_from_bitvec (&mut self, tree: &BitVec<u8>) {
            for i in 0..tree.len() {
                self.write_bit(tree.get_bit(i));
            }

            self.file.sync_all().expect("Couldn't sync data to file.");
        }

        pub fn write_bp(&mut self, bp_tree: &BalancedParenthesis) {
            self.write_file_from_bitvec(bp_tree.get_parenthesis());
        }

        pub fn write_louds(&mut self, louds_tree: &BalancedParenthesis) {
            self.write_file_from_bitvec(louds_tree.get_parenthesis());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{File, remove_file};
    use std::path::Path;
    use std::io::{Read, Write};
    use bv::{BitVec, Bits};
    use succinct_trees::bp::BalancedParenthesis;
    use super::parser::TreeParser;

    fn delete_file_by_string(file: String) {
        let path = Path::new(&file);
        remove_file(path).expect("Could not delete test file.");
    }

    #[test]
    fn test_parse_bp_read () {
        //### Before
        let filename = "test_parser_read.txt";

        let mut file = File::create(String::from(filename)).unwrap();
        file.write_all(b"((()()))").expect("Could not create test file.");

        // Why do I have to open file again?
        let file = File::open(String::from(filename)).unwrap();

        //### When
        let mut parser = TreeParser::new(file);
        let tree = parser.read_bp();

        //### Then
        let parenthesis: BitVec<u8>= bit_vec![true, true, true, false, true, false, false, false];

        assert_eq!(tree.get_parenthesis().len(), parenthesis.len());

        for i in 0..parenthesis.len() {
            let expected = parenthesis.get_bit(i);
            assert_eq!(tree.get_parenthesis().get_bit(i), expected);
        }

        //### Clean Up
        delete_file_by_string(String::from(filename));
    }

    #[test]
    fn test_parse_bp_write () {
        //### Before
        let filename = "test_parser_write.txt";
        let file = File::create(String::from(filename)).unwrap();

        let parenthesis: BitVec<u8>= bit_vec![true, true, true, false, true, false, false, false];
        let tree = BalancedParenthesis::new(parenthesis);

        //### When
        let mut parser = TreeParser::new(file);
        parser.write_bp(&tree);

        //### Then
        let mut file_content = String::new();
        let mut file = File::open(String::from(filename)).unwrap();
        file.read_to_string(&mut file_content).expect("Could not read BP write test file.");

        //### Then
        assert_eq!(file_content, "((()()))");


        //### Clean Up
        delete_file_by_string(String::from(filename));
    }
}