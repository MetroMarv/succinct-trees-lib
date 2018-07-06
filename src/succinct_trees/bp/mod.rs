use bv::{BitVec, Bits};
use std::fmt;
use super::SuccinctTreeFunctions;


pub struct BalancedParenthesis {
    blocksize: u64,
    range_min_max_tree: RangeMinMaxTree,

    /* For fields added in future please add
     * #[serde(skip_deserializing,skip_serializing)]
     * annotation. So it's not (de)serialized.
     */
}

pub struct RangeMinMaxTree {
    parenthesis: BitVec<u8>,
    excess: Vec<i64>,
    maximum: Vec<i64>,
    minimum: Vec<i64>,
    quantity: Vec<u64>,
    blksize: u64,
}


impl SuccinctTreeFunctions for BalancedParenthesis{
    fn has_index(&self, index:u64) -> bool {
      index < self.range_min_max_tree.parenthesis.len()
    }

    fn is_leaf(&self, _lf:u64) -> bool{
        if self.range_min_max_tree.parenthesis.get_bit(_lf) {
            !self.range_min_max_tree.parenthesis.get_bit(_lf + 1)
        } else {
            self.range_min_max_tree.parenthesis.get_bit(_lf - 1)
        }
    }

    fn first_child(&self,_lf:u64) -> Option<u64>{
        if !self.is_leaf(_lf){
            return Some(_lf + 1);
        }
        return None;
    }

    fn next_sibling(&self,_lf:u64) -> Option<u64>{
        let s = self.range_min_max_tree.fwdsearch(_lf, -1) + 1;
        if self.range_min_max_tree.parenthesis.get_bit(s) == true {
            return Some(s);
        }
        return None;
    }

    fn parent(&self,_lf:u64) -> u64{
        self.range_min_max_tree.bwdsearch(_lf, -2) + 1
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
        (self.range_min_max_tree.fwdsearch(_lf, -1) - _lf + 1)/2
    }
    fn pre_rank(&self,_lf:u64) -> u64{
        unimplemented!();
    }
    fn ancestor(&self,_lf:u64, _lf2:u64) -> bool{
        unimplemented!();
    }
    fn child(&self,_lf:u64, _lf2:u64) -> Option<u64>{
        unimplemented!();
    }
    fn lca(&self,_lf:u64, _lf2:u64) -> u64{
        unimplemented!();
    }
    fn level_ancestor(&self,_lf:u64, _lf2:u64) -> u64{
        self.range_min_max_tree.bwdsearch(_lf, -(_lf2 as i64) - 1) + 1
    }
    fn degree(&self,_lf:u64) -> u64{
        unimplemented!();
    }

    fn depth(&self,_lf:u64) -> u64{
        self.excess(_lf) as u64
    }



}


impl RangeMinMaxTree {

    pub fn new(parenthesis: BitVec<u8>, block_size: u64) -> RangeMinMaxTree {
        let blksize = block_size;
        let len = ((2*parenthesis.len())/block_size) as usize;
        let len_f = len as f64;
        // set vec length
        let mut excess = Vec::with_capacity(len);
        let mut maximum = Vec::with_capacity(len);
        let mut minimum = Vec::with_capacity(len);
        let mut quantity = Vec::with_capacity(len);
        for i in 0..len{
            excess.push(0);
            maximum.push(0);
            minimum.push(0);
            quantity.push(0);
        }

        if parenthesis.len() == 0 {
            return RangeMinMaxTree{parenthesis, excess, maximum, minimum, quantity, blksize};
        }

        for i in 1..(len_f.log2().floor() + (1 as f64)) as u64 {
            let row: u32 = i as u32;
            let mut block_count = 1;
            let mut vec_count = 0;

            let mut exc :i64 = 0;
            let mut max = 0;
            let mut min = 0;
            let mut qty = 0;
            let mut is_first = true;

            for j in 0..parenthesis.len() {
                println!("j: {}", i);

                if parenthesis.get_bit(j) {
                    exc += 1;
                } else {
                    exc -= 1;
                }


                if !is_first {
                    if exc > max {
                        max = exc;
                    }
                    if exc < min {
                        min = exc;
                        qty = 1;
                    } else if exc == min {
                        qty += 1;
                    }
                } else {
                    min = exc;
                    qty = 1;
                    max = exc;
                    is_first = false;
                }

                println!("akt. Exc {}", exc);

                if block_count < (block_size*(2 as u64).pow(row - 1)) as u32{

                    println!("{}", block_count);

                    block_count += 1;
                } else {


                    println!("Excess wird eingetragen{}", exc);

                    excess[(len/(2_usize.pow(row)) + vec_count)] =  exc;
                    minimum[(len/(2_usize.pow(row)) + vec_count)] = min;
                    maximum[(len/(2_usize.pow(row)) + vec_count)] = max;
                    quantity[(len/(2_usize.pow(row)) + vec_count)] = qty;


                    vec_count += 1;
                    block_count = 1;
                    exc = 0;
                    max = 0;
                    min = 0;
                    qty = 0;
                    is_first = true;
                }
            }

        }

        RangeMinMaxTree{parenthesis, excess, maximum, minimum, quantity, blksize}
    }

    fn fwdsearch(&self,_i: u64, mut _d: i64) -> u64 {
        let _b = self.blksize;
        let mut _k :u64 =_i/_b;
        let mut diff = 0;
        for j in _i+1.. (_k+1)*_b{
            println!("Suche in Block durch Schritt1");
            if self.parenthesis[j]{
                diff+=1;
            }else{
                diff-=1;
            }
            if _d  == diff {
                return j;
            }

        }
        _d = _d - diff;
        //TODO Knoten der k-ten Block beschreibt
        let node = self.parenthesis.len()/ 2 + _i/_b;
        println!("Springe zu {}-ten Block: Schritt 2", node);
        return self.fw_step_2(node,_d);
    }

    fn fw_step_2(&self, mut block :u64, mut _d: i64) -> u64 {
        if self.is_right_child(block) {
            println!("Aufruf Schritt 2 Als RightChild : mit v {}", block/2);
            return self.fw_step_2(block/2,_d);
        }else {
            let right_sibling :u64 = block +1;
            let min :i64 = self.minimum[right_sibling as usize];
            let max :i64 = self.maximum[right_sibling as usize];
            if min <= _d && _d <= max{
                println!("Aufruf Schritt 3 : mit v {}", right_sibling);
                return self.fw_step_3(right_sibling,_d);

            }else{
                _d = _d - self.excess[right_sibling as usize];
                println!("Aufruf Schritt 2 : mit v {}", block/2);
                return self.fw_step_2(&block/2, _d);
            }
        }
    }

    fn fw_step_3(&self,block :u64,mut _d :i64) -> u64{
        println!("Schritt 3 mit v = {} d = {}",block,_d );
        if self.is_leaf(block){
            println!("Schritt 3 : Leaf");
            let _b = self.blksize;
            let mut diff = 0;
            let index = (block -self.parenthesis.len()/2)* _b;
            for j in index..index+_b{
                println!("Suche in Block durch Schritt 3");
                println!("index = {}, block = {} , j = {}",index,block,j);
                if self.parenthesis[j] {
                    diff += 1;
                } else {
                    diff -= 1;
                }
                if _d == diff {
                    return j;
                }

            }
        }else {
            let _v_l = 2 * block;
            let _v_r = 2 * block + 1;
            let min: i64 = self.minimum[_v_l as usize];
            let max: i64 = self.maximum[_v_l as usize];
            println!("Min:  {}, Max: {}, d_:   {}", min, max, _d);

            if min <= _d && _d <= max {
                println!("Schritt 3 if");
                return self.fw_step_3( _v_l, _d);
            } else {
                println!("Schritt 3 else");
                _d = _d - self.excess[_v_l as usize];
                return self.fw_step_3( _v_r, _d);
            }
        }
        panic!("No result for fwdsearch");
    }

    fn is_right_child(&self,_v :u64) -> bool{
        if _v == 0 || _v == 1 {
            return false;
        }
            else{
                return _v%2 == 1;
            }
    }

    fn is_left_child(&self,_v :u64) -> bool{
        if _v == 0 || _v == 1 {
            return false;
        }
            else{
                return _v%2 == 0;
            }
    }

    fn division_round_up(&self,_a :u64, _b :u64) -> u64{
        if  _a%_b == 0{
            return _a/_b;
        }else{
            return _a/_b +1 ;
        }
    }

    fn bw_step_2(&self, mut _v :u64, mut _d: i64) -> u64 {
        if self.is_right_child(_v) {
            return self.bw_step_2(_v/2,_d);
        }else {
                let _v2 :u64 = _v +1;
                let min :i64 = self.minimum[_v2 as usize];
                let max :i64 = self.maximum[_v2 as usize];
                if min <= _d && _d <= max{
                    return self.bw_step_3(_v2,_d);

                }else{
                    _d = _d - self.excess[_v2 as usize];
                    return self.bw_step_2(_v/2, _d);
                }
        }
    }


    fn bw_step_3(&self,_v :u64,mut _d :i64) -> u64{
        if self.is_leaf(_v){
            let mut excess :u64 = 0;
            let _b = self.blksize;
            let _k :u64 = self.division_round_up(_v,_b);
            for j in _v-1.. (_k-1)*_b{
                if self.excess[_v as usize] as i64 +_d  == self.excess[_v as usize] as i64{
                    excess = j;
                }
            }
            return excess;
        }else {
            let _v_l = 2 * _v;
            let _v_r = 2 * _v + 1;
            let min: i64 = self.minimum[_v_l as usize];
            let max: i64 = self.maximum[_v_r as usize];
            if min <= _d && _d <= max {
                return self.bw_step_3( _v_l, _d);
            } else {
                _d = _d - self.excess[_v_l as usize];
                return self.bw_step_3(_v_r, _d);
            }
        }
    }

    fn is_leaf(&self,_v :u64) -> bool{
        return 2*_v as usize >self.excess.len() -1;
    }

    fn has_right_sibling(&self,_v :u64) -> bool{
        return self.excess.len() -1 == _v as usize;
    }

    pub fn bwdsearch(&self,_i: u64,mut _d: i64) -> u64{
        let _b = self.blksize;
        let mut _k :u64 = self.division_round_up(_i,_b);
        for j in _i.wrapping_sub(1).. (_k.wrapping_sub(1)).wrapping_mul(_b){
            if self.excess[_i as usize] as i64 +_d  == self.excess[j as usize] as i64{
                return j;
            }

        }
        _d = _d - (self.excess[(_k*self.blksize) as usize] - self.excess[_i as usize]);
        return self.bw_step_2(_k+1,_d);
    }

    pub(crate) fn get_excess(&self) -> &Vec<i64> {&self.excess}

    pub(crate) fn get_minimum(&self) -> &Vec<i64> {&self.minimum}

    pub(crate) fn get_maximum(&self) -> &Vec<i64> {&self.maximum}

    pub(crate) fn get_quantity(&self) -> &Vec<u64> {&self.quantity}

}

impl BalancedParenthesis {

    pub fn new_with_fixed_blocksize(parenthesis: BitVec<u8>) -> BalancedParenthesis {
        let bp = BalancedParenthesis::new(parenthesis, 2);
        bp
    }

    pub fn new(parenthesis: BitVec<u8>, blocksize: u64) -> BalancedParenthesis {
        let range_min_max_tree = RangeMinMaxTree::new(parenthesis, blocksize);
        BalancedParenthesis{ blocksize ,range_min_max_tree}
    }

    pub fn get_parenthesis(&self) -> &BitVec<u8>{
        &self.range_min_max_tree.parenthesis
    }

    pub(crate) fn excess(&self, position: u64) -> u64 {
        let mut count = 0;
        for i in 0..position {
            if self.range_min_max_tree.parenthesis.get_bit(i) {
                count += 1;
            } else {
                count -= 1;
            }
        }
        count
    }

}

impl fmt::Display for BalancedParenthesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parenthesis_expression = String::from("");
        for i in 0..self.range_min_max_tree.parenthesis.len()-1 {
            let bit = self.range_min_max_tree.parenthesis.get_bit(i);

            if bit {
                parenthesis_expression.push_str("(");
            } else {
                parenthesis_expression.push_str(")");
            }
        }
        write!(f, "BP-Tree: {}", parenthesis_expression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::{serialize, deserialize, Result };
    use bv::Bits;
    use succinct_trees::SuccinctTreeFunctions;


    pub fn example_tree() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, false];
        return BalancedParenthesis::new(parenthesis, 2);
    }

    pub fn example_tree_2() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![true, false, true, true, true, false, false, false];
        return BalancedParenthesis::new(parenthesis, 2);
    }

    pub fn example_tree_big() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false];
        return BalancedParenthesis::new(parenthesis, 4);
    }

    pub fn empty_tree() -> BalancedParenthesis{
        let parenthesis: BitVec<u8> = bit_vec![];
        return BalancedParenthesis::new(parenthesis, 2);
    }

    #[test]
    fn test_constructor() {
        let tree = example_tree();
        println!("{}",tree);
        assert_eq!(tree.get_parenthesis().get_bit(3), false);
    }

    /*#[test]
    fn test_serialization () {
        let tree = example_tree();

        let serialized = serialize(&tree).unwrap();

        let deserialized: Result<BalancedParenthesis> = deserialize(&serialized[..]);

        assert_eq!(deserialized.unwrap().get_parenthesis().get_bit(3), false)
    }*/

    #[test]
    fn test_is_leaf() {
        assert_eq!(example_tree().is_leaf(0), false);
        assert_eq!(example_tree().is_leaf(4), true);
    }

    #[test]
    #[should_panic]
    fn test_is_leaf_empty(){
        empty_tree().get_parenthesis().get_bit(0);
    }
    

    #[test]
    #[should_panic]
    fn test_first_child_empty(){
        empty_tree().first_child(0);
    }

    #[test]
    fn test_first_child(){
        assert_eq!(example_tree().first_child(0),Some(1));
    }


    #[test]
    fn test_next_sibling(){
        assert_eq!(example_tree().next_sibling(2).unwrap(), 4);
    }

    #[test]
    #[should_panic]
    fn test_next_sibling_empty(){
        empty_tree().next_sibling(2);
    }

    #[test]
    #[should_panic]
    fn test_parent_empty(){
        empty_tree().parent(4);
    }

    #[test]
    fn test_parent(){
        assert_eq!(example_tree().parent(1), 0)
    }

    #[test]
    fn test_subtree_size(){
        assert_eq!(example_tree().subtree_size(1), 2)
    }

    #[test]
    #[should_panic]
    fn test_subtree_size_empty(){
        empty_tree().subtree_size(0);
    }

    #[test]
    fn test_ancestor(){
        assert_eq!(example_tree().ancestor(0,1),false);
    }

    #[test]
    #[should_panic]
    fn test_ancestor_empty(){
        empty_tree().ancestor(0,1);
    }

    #[test]
    fn test_level_ancestor(){
        assert_eq!(example_tree().level_ancestor(0,1), 0);
    }

    #[test]
    #[should_panic]
    fn test_level_ancestor_empty(){
        empty_tree().level_ancestor(0,1);
    }

    #[test]
    fn test_lca(){
        assert_eq!(example_tree().lca(2,4),0);
    }

    #[test]
    #[should_panic]
    fn test_lca_empty(){
        empty_tree().lca(2,4);
    }

    #[test]
    fn test_child(){
        assert_eq!(example_tree().child(0, 0),Some(1));
    }

    #[test]
    #[should_panic]
    fn test_child_empty(){
        empty_tree().child(0,1);
    }

    #[test]
    fn test_depth(){
        assert_eq!(example_tree().depth(0),0);
    }

    #[test]
    #[should_panic]
    fn test_depth_empty(){
        empty_tree().depth(0);
    }

    #[test]
    fn test_degree(){
        assert_eq!(example_tree().degree(0),0);
    }

    #[test]
    #[should_panic]
    fn test_degree_empty(){
        empty_tree().degree(0);
    }

    #[test]
    fn test_enclose(){
        // TODO: (MR) Expects u64 instead of bool. Put 0 as expected result now but is this correct?
        assert_eq!(example_tree().enclose(0),0);
    }

    #[test]
    #[should_panic]
    fn test_enclose_empty(){
        empty_tree().enclose(0);
    }

    #[test]
    fn  test_construct_rmm_tree() {
        let tree = example_tree();
        let range_min_max_tree = tree.range_min_max_tree;

        // test excess
        let vec_exc = vec![0, 0, 2, -2, 2, 0, 0, -2];
        assert_eq!(*range_min_max_tree.get_excess(), vec_exc);

        //test minimum
        let vec_min = vec![0, 0, 1, -2, 1, 0, 0, -2];
        assert_eq!(*range_min_max_tree.get_minimum(), vec_min);

        //test maximum
        let vec_max = vec![0, 3, 3, 1, 2, 1, 1, -1];
        assert_eq!(*range_min_max_tree.get_maximum(), vec_max);

        //test quantity
        let vec_qty = vec![0, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(*range_min_max_tree.get_quantity(), vec_qty);
    }

    #[test]
    fn  test_construct_rmm_tree_2() {
        let tree = example_tree_2();
        let range_min_max_tree = tree.range_min_max_tree;

        // test excess
        let vec_exc = vec![0, 0, 2, -2, 0, 2, 0, -2];
        assert_eq!(*range_min_max_tree.get_excess(), vec_exc);

        //test minimum
        let vec_min = vec![0, 0, 0, -2, 0, 1, 0, -2];
        assert_eq!(*range_min_max_tree.get_minimum(), vec_min);

        //test maximum
        let vec_max = vec![0, 3, 2, 1, 1, 2, 1, -1];
        assert_eq!(*range_min_max_tree.get_maximum(), vec_max);

        //test quantity
        let vec_qty = vec![0, 2, 1, 1, 1, 1, 1, 1];
        assert_eq!(*range_min_max_tree.get_quantity(), vec_qty);
    }

    #[test]
    fn  test_construct_rmm_tree_big() {
        let tree = example_tree_big();
        let range_min_max_tree = tree.range_min_max_tree;

        // test excess
        let vec_exc = vec![0, 0, 2, -2, 2, 0, 0, -2];
        assert_eq!(*range_min_max_tree.get_excess(), vec_exc);

        //test minimum
        let vec_min = vec![0, 0, 1, -2, 1, -1, -1, -2];
        assert_eq!(*range_min_max_tree.get_minimum(), vec_min);

        //test maximum
        let vec_max = vec![0, 3, 3, 1, 3, 1, 1, 1];
        assert_eq!(*range_min_max_tree.get_maximum(), vec_max);

        //test quantity
        let vec_qty = vec![0, 1, 2, 1, 1, 1, 1, 1];
        assert_eq!(*range_min_max_tree.get_quantity(), vec_qty);
    }

    #[test]
    fn test_fwdsearch(){
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, false];

        let rmm: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis,2);

        assert_eq!(rmm.fwdsearch(2,-1), 3);
        assert_eq!(rmm.fwdsearch(2,0), 4);

    }


}