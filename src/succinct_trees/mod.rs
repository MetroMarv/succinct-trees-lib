pub mod bp;
pub mod louds;

pub trait SuccinctTreeFunctions{
    fn is_leaf(_: i32) -> bool;
    fn first_child(_: i32) -> i32;
    fn next_sibling(_: i32) -> i32;
    fn parent(_: i32) -> i32;
    fn rank(_: i32) -> i32;
    fn select(_: i32) -> i32;
    fn close_rank(_: i32) -> i32;
    fn close_select(_: i32) -> i32;
    fn enclose(_: i32) -> i32;
    fn subtree_size(_: i32) -> i32;
    fn pre_rank(_: i32) -> i32;
    fn ancestor(_: i32, _: i32) -> bool;
    fn child(_: i32,_: i32) -> i32;
    fn lca (_: i32, _: i32) -> i32;
    fn level_ancestor(_: i32, _: i32) -> i32;
    fn degree(_: i32) -> i32;
    fn depth(_: i32) -> i32;
}