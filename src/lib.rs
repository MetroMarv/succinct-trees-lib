#![feature(test)]
extern crate test;

#[macro_use]
extern crate bv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate bio;

pub mod succinct_trees;

#[cfg(test)]
mod tests {
    use succinct_trees;
    use succinct_trees::parser::TreeParser;
    use bv::{Bits,BitVec};
    use std::fs::File;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_tree() {
        let parenthesis: BitVec<u8>= bit_vec![true, true, true, false, true, false, false, false];
        let tree = succinct_trees::bp::BalancedParenthesis::new_with_fixed_blocksize(parenthesis);

        assert_eq!(tree.get_parenthesis().get_bit(3), false);
    }

    #[bench]
    fn bench_read_bp_10000_nodes(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_bp_nodes_7691_depth_5_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            let bp = parser.read_bp();
        });
    }

    #[bench]
    fn bench_read_louds_10000_nodes(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_louds_nodes_7691_depth_5_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            let bp = parser.read_louds();
        });
    }

    #[bench]
    fn bench_read_bp_27_nodes(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_bp_nodes_27_depth_10_max_children_2.txt");
        let fileResult= File::open(path);

        let file = match fileResult {
            Ok(file) => file,
            Err(err) => panic!("My Error. {:?}", err)
        };

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            let bp = parser.read_bp();
        });
    }

    #[bench]
    fn bench_read_louds_27_nodes(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_louds_nodes_27_depth_10_max_children_2.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            let bp = parser.read_louds();
        });
    }
}
