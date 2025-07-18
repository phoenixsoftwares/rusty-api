mod b_to_d_helper;

fn main() {
    let tst = 56;

    // let p = (0..9).map (|n| (tst >> n) & 1);
    // let c = 33 & 1;
    // println!("\n\n\nBinary representation of {}: {:?}", tst, p.collect::<Vec<_>>());
    // println!("{} & 1 = {}", c, c);


    
    println!("Answer: {}", b_to_d_helper::decimal_to_binary(tst));
}


