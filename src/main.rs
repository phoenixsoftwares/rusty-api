
fn main() {
    let one_var: i32 = 1;
    let two_var: i32 = 2;
    let three_var: i32 = 3;

    let lib_var: i32 = vars_example::add(one_var, two_var);
    println!("vars_example - {}", lib_var);

    let one_var_copy = one_var;

    println!("One Copy - {}", one_var_copy);

    println!("One - {}", one_var);
    println!("Two - {}", two_var);
    println!("Three - {}", three_var);


    let one_var_string = String::from("One");
    let two_var_string = String::from("Two");
    let three_var_string = String::from("Three");

    // The following line would cause a compile-time error because `one_var_string` has been moved
    // let one_var_copy = one_var_string;
    

    let one_var_copy = one_var_string.clone();
    
    println!("One Copy - {}", one_var_copy);

    println!("One - {}", one_var_string);
    println!("Two - {}", two_var_string);
    println!("Three - {}", three_var_string);
}
