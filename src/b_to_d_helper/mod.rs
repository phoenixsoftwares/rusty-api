#[derive(Debug, Clone, Copy)]
struct BinaryHelper {
     pub byte: i32,
     pub place: i32,
}



fn test_for_zero_one(t: i32) -> bool {
    t == 1 || t == 0
}

fn assign_tag(t: usize, v: &mut Vec<BinaryHelper> ) -> i32{
    let mut b = v[t];
    b.place = 1;
    v[t] = b;
    b.byte
}

fn new() -> Vec<BinaryHelper> {
    let mut byte = 512;
    let place = 0;
    let mut v: Vec<BinaryHelper> = vec![];
    
    while byte != 1 {
        byte /= 2;
        
        let b = BinaryHelper {
            byte,
            place,
        };
        v.push(b);
    }
   v
}


pub fn decimal_to_binary(pst: i32) -> String {
    let mut tst = pst;
    if tst >= 256 {
            panic!("Cannot print bigger byte = {}", tst);
    }

    if test_for_zero_one(tst) {
        println!("Answer is {}", tst);
        return tst.to_string();
    }

    let mut binary_vector = new();
    let lngth = binary_vector.len();
    let mut answer = String::new();

    for _ in 0..(lngth-1) {
        if tst == 0 {
            break;
        }
        tst = tst - test_probablities(tst, lngth, &mut binary_vector);
    }

    for itr in 0..(lngth) {
        answer.push_str(&binary_vector[itr].place.to_string());
    }
    answer
}

fn test_probablities(tst: i32, lngth: usize, binary_vector: &mut Vec<BinaryHelper> ) -> i32 {
    let mut nk = 0;    
    for itr in 0..(lngth-1) {
        
        if  binary_vector[itr].byte <= tst {
            nk = assign_tag(itr, binary_vector);
            // println!("\n\nNK {}: ", nk);
            break
        }
    }
    nk

}
