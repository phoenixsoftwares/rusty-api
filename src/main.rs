mod b_to_d_helper;

fn main() {
    let tst = 56;
    
    let mut binary_vector = b_to_d_helper::new();

    if b_to_d_helper::test_for_zero_one(tst) {
        println!("Answer is {}", tst);
        return;
    }

    if tst >= 256 {
            panic!("Cannot print bigger byte = {}", tst);
    }

    for itr in 0..(binary_vector.len()-1) {
        

        if  binary_vector[itr].byte < tst {
            b_to_d_helper::assign_tag(itr, &mut binary_vector);
            break
        }

    }
    for itr in 0..(binary_vector.len()-1) {
        println!("BinaryHelper {}: {:?}", itr, binary_vector[itr]);
    }
    // println!("{:?}", binary_vector);
}


