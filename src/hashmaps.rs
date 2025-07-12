use std::collections::HashMap;
pub fn hashmaps(){
    let mut users : HashMap<String,i8> = HashMap::new();
    users.insert(String::from("Pavan"), 21);
    users.insert(String::from("Gandhi"), 22);
    users.insert(String::from("Suresh"), 23);
    users.insert(String::from("Kind"),19);
    println!("Users: {:?}", users);
}