pub fn vector(){
    let mut vec = Vec::new();
    for i in 0..10 {
        if i%2==0{
            vec.push(i);
        }
        // vec.push(i);
    }
    println!("Vector: {:?}", vec);
    add_odd(&mut vec);
}
fn add_odd(v:&mut Vec<i32>)->Vec<i32>{
    for i in 0..10 {
        if i%2!=0{
            v.push(i);
        }
    }
    println!("Vector after adding odd numbers: {:?}", v);
    return v.to_vec();
}