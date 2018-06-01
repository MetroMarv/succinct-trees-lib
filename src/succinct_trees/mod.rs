pub mod bp;
pub mod louds;

pub mod parser {
    use std::fs::File;
    use std::io::{Read, Write};
    use succinct_trees::bp::BalancedParenthesis;
    use succinct_trees::louds::Louds;
    use bv::{BitVec, Bits};

    struct TreeParser {
        file: File
    }

    impl TreeParser {
        fn new(file: File) -> TreeParser {
            TreeParser {file}
        }

        fn read_file (&mut self) -> String {
            let mut content = String::new();
            self.file.read_to_string(&mut content);
            content
        }

        fn read_file_as_bitvec (&mut self) -> BitVec {
            let parenthesis = self.read_file();

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

        fn read_bp (&mut self) -> BalancedParenthesis {
            let bitvec = self.read_file_as_bitvec();

            BalancedParenthesis::new(bitvec)
        }

        fn read_louds (&mut self) -> Louds {
            let bitvec = self.read_file_as_bitvec();

            Louds::new(bitvec)
        }

        fn write_file_from_bitvec (&mut self, tree: &BitVec) {
            let i: u64;
            for i in 0..tree.len() {
                let bit = tree.get_bit(i);

                if bit {
                    self.file.write(b"(");
                } else {
                    self.file.write(b")");
                }
            }
        }

        fn write_bp(&mut self, bp_tree: &BalancedParenthesis) {
            self.write_file_from_bitvec(bp_tree.get_parenthesis());
        }

        fn write_louds(&mut self, louds_tree: &BalancedParenthesis) {
            self.write_file_from_bitvec(louds_tree.get_parenthesis());
        }
    }
}