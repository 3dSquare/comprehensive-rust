fn main() {
    let mut x = 6;

    print!("{x}");
    while x != 1{
        if x % 2 == 0 {
            x = x/2;
        } else {
            x = 3 * x + 1;
        }

        println!("{x}");
    }
    println!();

    compound_types();
    references();
    dangling_ref();
}

fn compound_types(){
    let mut array = [42i16; 10];
    println!("{:?}", array);

    let tuple = (123, true);
    println!("{:?}", tuple);
}

fn references() {
    let mut x = 123;
    let ref_x = &mut x;

    *ref_x = 42;
    println!("{x}");
}

fn dangling_ref(){
    let ref_x: &i32;
    // {
        let x = 123;
        ref_x = &x;
    // }
    println!("{ref_x}");
}

// fn main() {              // Program entry point
//     let mut x: i32 = 6;  // Mutable variable binding
//     print!("{x}");       // Macro for printing, like printf
//     while x != 1 {       // No parenthesis around expression
//         if x % 2 == 0 {  // Math like in other languages
//             x = x / 2;
//         } else {
//             x = 3 * x + 1;
//         }
//         print!(" -> {x}");
//     }
//     println!();
// }