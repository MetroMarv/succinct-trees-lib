use bv::{BitVec, Bits};


pub struct RangeMinMaxTree {
    pub parenthesis: BitVec<u8>,
    excess: Vec<i64>,
    maximum: Vec<i64>,
    minimum: Vec<i64>,
    quantity: Vec<u64>,
    blksize: u64,
}

impl RangeMinMaxTree {

    pub fn new(parenthesis: BitVec<u8>, block_size: u64) -> RangeMinMaxTree {
        let blksize= block_size;
        let next_power = (parenthesis.len() as f64).log2().ceil() as u32;
        let len = ((2*(2_i32.pow(next_power)))/block_size as i32) as usize;
        //println!("Die Länge ist {}",len);

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

            for j in 0..2_i32.pow(next_power) {
                if  (j as u64) < parenthesis.len() {
                    if parenthesis.get_bit(j as u64) {
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
                }

                if block_count < (block_size*(2 as u64).pow(row - 1)) as u32{

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

    pub fn fwdsearch(&self,_i: u64, mut _d: i64) -> u64 {
        let _b = self.blksize;
        let mut _k :u64 =_i/_b;
        let mut diff = 0;
        let kb = (_k+1)*_b +1; //anscheinend wird der Durchgang für den max index in rust nicht ausgeführt? -> deshalb +1
        if(_i%_b !=0){
            for j in _i+1.. kb {
                println!("Suche in (Rest)-Block ({} ... {}) durch Schritt 1: j = {}", _i + 1, kb-1, j);
                if self.parenthesis[j - 1] {
                    diff += 1;
                } else {
                    diff -= 1;
                }
                println!("Unterschied gesucht: {}, gefunden: {}", _d, diff);
                if _d == diff {
                    println!("Excess gefunden: Gesuchter Index ist {}", j);
                    return j;
                }
            }
        }

        _d = _d - diff;
        //TODO Knoten der k-ten Block beschreibt
        //println!("{} {}", self.excess.len()/ 2,_i/_b);
        let node = (self.excess.len()/ 2 + ((_i-1)/_b) as usize) as u64;
        println!("Aufruf Schritt 2 auf rmmT-Block {}", node);
        return self.fw_step_2(node,_d);
    }

    fn fw_step_2(&self, mut block :u64, mut _d: i64) -> u64 {
        if self.is_right_child(block) {
            if(block/2 != 1){
                println!("Aufruf Schritt 2,rechtes Kind erkannt, gehe zu Elternknoten {}", block/2);
            }else{
                println!("Aufruf Schritt 2, Elternknoten ist Wurzel ({})", block/2);
            }

            return self.fw_step_2(block/2,_d);
        }else {
            if(block == 1){
                println!("Springe zum linken Kind der Wurzel");
            }else {
                println!("Ist linkes Kind");
            }
            let right_sibling :u64 = block +1;
            let min :i64 = self.minimum[right_sibling as usize];
            let max :i64 = self.maximum[right_sibling as usize];
            println!("Min: {}, Max: {}, d: {}", min, max,_d);
            if min <= _d && _d <= max{
                println!("Aufruf Schritt 3 auf  {}-ten Block", right_sibling);
                return self.fw_step_3(right_sibling,_d);

            }else{
                _d = _d - self.excess[right_sibling as usize];
                println!("Aufruf Schritt 2 auf Block {}", block/2);
                return self.fw_step_2(&block/2, _d);
            }
        }
    }

    fn fw_step_3(&self,block :u64,mut _d :i64) -> u64{
        if self.is_leaf(block){
            println!("Block {} ist ein Blatt", block);
            let _b = self.blksize;
            let mut diff = 0;
            let index = (block -(self.excess.len()/2) as u64)* _b+1;
            for j in index..index+_b{
                println!("Suche in Block ({} ... {}) durch Schritt 3: j = {}",index,index+_b-1, j);
                //println!("index = {}, block = {} , j = {}",index,block,j);
                if self.parenthesis[j-1] {
                    diff += 1;
                } else {
                    diff -= 1;
                }
                println!("Unterschied gesucht: {}, gefunden: {}",_d,diff);
                if _d == diff {
                    println!("Excess gefunden: Gesuchter Index ist {}",j);
                    return j;
                }

            }
        }else {
            let _v_l = 2 * block;
            let _v_r = 2 * block + 1;
            let min: i64 = self.minimum[_v_l as usize];
            let max: i64 = self.maximum[_v_l as usize];
            //println!("Min:  {}, Max: {}, d_:   {}", min, max, _d);

            if min <= _d && _d <= max {
                println!("Rufe Schritt 3 auf linkem Kind {} auf", _v_l);
                return self.fw_step_3( _v_l, _d);
            } else {
                println!("Rufe Schritt 3 auf rechtem Kind {} auf", _v_r);
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

    pub fn bwdsearch(&self,_i: u64, mut _d: i64) -> u64 {
        let _b = self.blksize;
        let mut _k :u64 = self.division_round_up(_i,_b);
        let mut diff = 0;
        let kb =(_k-1)*_b+1;
        println!("Suche in (Rest)-Block ({} ... {}) ",_i-1,kb);
        if (_i-1)%_b !=0{
            let mut j= _i-1;
            while(kb <= j){
                println!("Suche in (Rest)-Block ({} ... {}) durch Schritt 1: j = {}",j,kb, j);
                if self.parenthesis[j]{
                    diff-=1;
                }else{
                    diff+=1;
                }
                println!("Unterschied gesucht: {}, gefunden: {}",_d,diff);
                if _d  == diff {
                    println!("Excess gefunden: Gesuchter Index ist {}",j);
                    return j;
                }
                j -=1;

            }
        }

        let mut k = 0;

        if self.parenthesis[_i-1]{
            k +=1;
        }else{
            k-=1;
        }

        _d = _d+k-diff;
        //TODO Knoten der k-ten Block beschreibt
        //println!("{} {}", self.excess.len()/ 2,_i/_b);
        let node = (self.excess.len()/ 2 + ((_i-1)/_b)  as usize) as u64;
        println!("Aufruf Schritt 2 auf rmmT-Block {}, gesucht excess: {}", node,_d);
        return self.bw_step_2(node,_d);
    }

    fn bw_step_2(&self, mut block :u64, mut _d: i64) -> u64 {
        if self.is_left_child(block) {
            if(block/2 != 1){
                println!("Aufruf Schritt 2,linkes Kind erkannt, gehe zu Elternknoten {}", block/2);
            }else{
                println!("Aufruf Schritt 2, Elternknoten ist Wurzel ({})", block/2);
            }

            return self.bw_step_2(block/2,_d);
        }else {
            if(block == 1){
                panic!("Fehler: Wurzel erreicht");
            }else {
                println!("Ist rechtes Kind");
            }
            let left_sibling :u64 = block -1;
            let min :i64 = self.minimum[left_sibling as usize];
            let max :i64 = self.maximum[left_sibling as usize];
            println!("Min: {}, Max: {}, d: {}", min, max,_d);
            if min <= _d && _d <= max{
                println!("Aufruf Schritt 3 auf linkem Geschwister: Block {}", left_sibling);
                _d = _d - self.excess[left_sibling as usize];
                return self.bw_step_3(left_sibling,_d);

            }else{
                //println!("d ist jetzt {}", _d + self.excess[left_sibling as usize]);
                //_d = _d + self.excess[left_sibling as usize];
                println!("Aufruf Schritt 2 auf Block {}", block/2);
                return self.bw_step_2(&block/2, _d);
            }
        }
    }


    fn bw_step_3(&self,block :u64,mut _d :i64) -> u64{
        if self.is_leaf(block){
            println!("Block {} ist ein Blatt", block);
            let _b = self.blksize;
            let mut diff = 0;//self.excess[block as usize];
            let index = (block -(self.excess.len()/2) as u64)* _b+1;
            let mut j= index+_b-1;
            while(index<=j){
                println!("Suche in Block ({} ... {}) durch Schritt 3: j = {}",index+_b-1,index, j);
                //println!("index = {}, block = {} , j = {}",index,block,j);
                if self.parenthesis[j-1] {
                    diff += 1;
                } else {
                    diff -= 1;
                }
                println!("Unterschied gesucht: {}, gefunden: {}",_d,diff);
                if _d == diff {
                    println!("Excess gefunden: Gesuchter Index ist {}",j);
                    return j;
                }
                j-=1;

            }
        }else {
            let _v_l = 2 * block;
            let _v_r = 2 * block + 1;
            let min: i64 = self.minimum[_v_l as usize];
            let max: i64 = self.maximum[_v_l as usize];
            println!("Min:  {}, Max: {}, d_:   {}", min, max, _d);

            if min <= _d && _d <= max {
                println!("Rufe Schritt 3 auf linkem Kind {} auf", _v_l);
                _d = _d + self.excess[_v_r as usize];
                return self.bw_step_3( _v_l, _d);
            } else {
                println!("Rufe Schritt 3 auf rechtem Kind {} auf, vl war {}", _v_r, _v_l);
                println!("d ist jetzt d = {} + {}", _d,self.excess[_v_l as usize]);
                //_d = _d + self.excess[_v_l as usize];
                return self.bw_step_3( _v_r, _d);
            }
        }
        panic!("No result for bwdsearch");
    }

    fn is_leaf(&self,_v :u64) -> bool{
        return 2*_v as usize >self.excess.len() -1;
    }

    fn has_right_sibling(&self,_v :u64) -> bool{
        return self.excess.len() -1 == _v as usize;
    }



    pub(crate) fn get_excess(&self) -> &Vec<i64> {&self.excess}

    pub(crate) fn get_minimum(&self) -> &Vec<i64> {&self.minimum}

    pub(crate) fn get_maximum(&self) -> &Vec<i64> {&self.maximum}

    pub(crate) fn get_quantity(&self) -> &Vec<u64> {&self.quantity}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fwdsearch_len32_blk8(){
        let parenthesis_bigger: BitVec<u8> =bit_vec![true, true, true, true, false, true, false, false, false, true, true, false, true, false, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false, false];
        let rmm_bigger: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis_bigger,8);
        /*assert_eq!(rmm_bigger.fwdsearch(11,1), 18);
        assert_eq!(rmm_bigger.fwdsearch(6,0), 18);
        assert_eq!(rmm_bigger.fwdsearch(7,-2), 9);*/
        assert_eq!(rmm_bigger.fwdsearch(11,-3), 32);
        //panic!("");
    }

    #[test]
    fn test_fwdsearch_len8_blk4(){
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, false];
        let rmm: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis,4);
        assert_eq!(rmm.fwdsearch(5,-3), 8);

    }
    #[test]
    fn test_fwdsearch_len8_blk2(){
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, false];
        let rmm: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis,2);
        assert_eq!(rmm.fwdsearch(5,-3), 8);
        assert_eq!(rmm.fwdsearch(1,1), 2);


    }

    #[test]
    fn test_fwdsearch_len32_blk16(){
        let parenthesis_bigger: BitVec<u8> =bit_vec![true, true, true, true, false, true, false, false, false, true, true, false, true, false, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false, false];
        let rmm_bigger: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis_bigger,16);
        assert_eq!(rmm_bigger.fwdsearch(11,1), 18);

        assert_eq!(rmm_bigger.fwdsearch(6,0), 18);


    }

    #[test]
    fn test_fwdsearch2(){
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, false];

        let parenthesis_bigger: BitVec<u8> =bit_vec![true, true, true, true, false, true, false, false, false, true, true, false, true, false, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false, false];

        let rmm: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis,2);
        let rmm_bigger: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis_bigger,8);

        assert_eq!(rmm.fwdsearch(5,-3), 8);



        assert_eq!(rmm_bigger.fwdsearch(11,1), 18);

        assert_eq!(rmm_bigger.fwdsearch(6,0), 18);
        //assert_eq!(rmm_bigger.bwdsearch(18,0), 6);


        //assert_eq!(rmm_bigger.bwdsearch(18,-2), 16);

    }

    #[test]
    fn test_bwdsearch_len8_blk2(){
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, false];
        let rmm: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis,2);

        assert_eq!(rmm.bwdsearch(8,3), 5);

        //assert_eq!(rmm.bwdsearch(2,-1), 1);
    }

    #[test]
    fn test_bwdsearch_len32_blk8_1(){
        let parenthesis_bigger: BitVec<u8> =bit_vec![true, true, true, true, false, true, false, false, false, true, true, false, true, false, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false, false];
        let rmm_bigger: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis_bigger,8);


        assert_eq!(rmm_bigger.bwdsearch(18,0), 6);

    }
    #[test]
    fn test_bwdsearch_len32_blk8_2(){
        let parenthesis_bigger: BitVec<u8> =bit_vec![true, true, true, true, false, true, false, false, false, true, true, false, true, false, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false, false];
        let rmm_bigger: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis_bigger,8);


        assert_eq!(rmm_bigger.bwdsearch(18,-2), 16);
    }

    #[test]
    fn test_bwdsearch_len8_blk2_1(){
        let parenthesis: BitVec<u8> = bit_vec![true, true, true, false, true, false, false, false];
        let rmm: RangeMinMaxTree = RangeMinMaxTree::new(parenthesis,2);


        assert_eq!(rmm.bwdsearch(8,2), 6);
    }
}
