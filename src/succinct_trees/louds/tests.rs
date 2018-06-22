use super::*;
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
    let tree = example_tree();

    assert_eq!(tree.child_count(1), 1);
    assert_eq!(tree.child_count(3), 2);
    assert_eq!(tree.child_count(6), 0);
}

#[test]
fn test_is_leaf(){
    let tree = example_tree();

    assert_eq!(tree.is_leaf(0), false);
    assert_eq!(tree.is_leaf(6), true);
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
    let tree = example_tree();

    assert_eq!(tree.ancestor(6,7),true);
    assert_eq!(tree.ancestor(3,7),false);
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