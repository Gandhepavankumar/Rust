mod ruststruct;
mod vec;
mod hashmaps;

fn main() {
    // print!("Hello, world!");
    // let  str = String::from("Hello, Rust!");
    // println!("{}", str);
    // For Numbers and Boolean the cahnge over the is easy and faster but it is completely opp for Strings

    // let mut i :i8 = 10;
    // println!("i8: {}", i);
    // i = 20;
    // println!("i8: {}", i);

    // 0



    // Coditions and loops
    // Until the condtion is complex don;'t use braceses
    // let mut k:i8=10;
    // for j in 0..10 {
    //     if j % 2 == 0 {
    //         k += j;
    //     } else {
    //         k -= j;
    //     }
    // }
    // println!("k: {}", k);

    // let input = String::from("first second");

    // let res = get_first_word(input);

    // println!("First word: {}", res);

    // for (i,c) in res.chars().enumerate(){
    //     println!("{} : {}", i, c);
    // }

    // Memory Management are 5 JARGONS
    /*  1.Mutable 
        2.Heap and Memory
        3.Ownership
        4.References and Borrowing
        5.Lifetimes

     */ 

    // println!("Help");

    // let a : i8 = 10;
    // let b : i8 = 20;
    // println!("res : {}",sum(a,b));


    // SCOPE 
    let a :i8 = 10;
    {
        let b :i8 = 20;
        println!("a: {}, b: {}", a, b);
    }
    //println!("a: {}, b: {}", a, b); // This will give error

    let s1 = String::from("Hello");
    let s2 = s1.clone(); // This will move the ownership of s1 to s2
    println!("String: {}", s2);

    let mut str = String::from("Hello, Rust!");
    let str1 = &str;
    println!("String: {}", str1);
    update(&mut str);

    let rect = ruststruct::Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of Rectangle: {}", rect.area());
    vec::vector();
    hashmaps::hashmaps();

}

// fn sum(a:i8, b:i8) -> i8 {

//     return a + b;
// }


// fn get_first_word(str :String)->String{
//     let mut res = String::new();
//     for char in str.chars(){
//         if char == ' ' {
//             res.remove(res.len()-1);
//             break;
//         }
//         res.push(char);
//     }
//     return res;
// }


fn update(str: &mut String) {
    str.push_str(" Updated");
    println!("Updated String: {}", str);
}