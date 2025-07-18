#[derive(Debug, Clone, Copy)]
pub  struct BinaryHelper {
     pub byte: i32,
     pub place: i32,
     pub index: i32,
}



pub fn test_for_zero_one(t: i32) -> bool {
    t == 1 || t == 0
}

pub fn assign_tag(t: usize, v: &mut Vec<BinaryHelper> ) {
    let mut b = v[t];
    b.place = 1;
    v[t] = b;
    // v
}

pub fn new() -> Vec<BinaryHelper> {
    let mut byte = 512;
    let mut index = 0;
    let place = 0;
    

    let mut v: Vec<BinaryHelper> = vec![];
    
    while byte != 1 {
        byte /= 2;
        
        let b = BinaryHelper {
            byte,
            place,
            index,
        };
        index += 1;
        v.push(b);
    }
   v
}
