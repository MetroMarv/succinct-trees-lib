pub mod bp;
pub mod louds;

pub trait SuccinctTreeFunctions{
    fn is_leaf(i32) -> bool;
    fn first_child(i32) -> i32;
    fn next_sibling(i32) -> i32;
    fn parent(i32) -> i32;
    fn rank(i32) -> i32;
    fn select(i32) -> i32;
    fn close_rank(i32) -> i32;
    fn close_select(i32) -> i32;
    fn enclose(i32) -> i32;
    fn subtree_size(i32) -> i32;
    fn pre_rank(i32) -> i32;
    fn ancestor(i32, i32) -> bool;
    fn child(i32,i32) -> i32;
    fn lca (i32, i32) -> i32;
    fn level_ancestor(i32, i32) -> i32;
    fn degree(i32) -> i32;
    fn depth(i32) -> i32;
}