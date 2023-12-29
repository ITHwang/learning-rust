fn main() {
    // 1. Vectors
    
    let v1: Vec<i32> = Vec::new(); // a new empty vector
    let mut v2 = vec![1, 2, 3]; // a vector initialized with elements

    v2.push(4); // updating a vector
    v2.push(5);
    v2.push(6);
    v2.push(7);

    // reading elements
    let third: &i32 = &v2[2];
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // When updating and reading, you can't have mutable and immutable references to the same scope.
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(5);
    // println!("The first element is {}", first); // causes an error

    // iterating over a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // iterate over a vector using the dereference operator *
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // Using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    // 2. Strings

    // The `String` and the string slice `&str` are UTF-8 encoded.

    // creating 
    let mut s = String::new();
    let data = "Hello, world!";
    let mut s = data.to_string();
    let mut s = String::from("Hello, world!");

    // updating
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); // s2 is still valid.

    let mut s = String::from("lo");
    s.push('l'); // push a letter.

    // + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // The ownership of s1 is moved to s3 and s2 is still valid.
    let s3 = s1 + &s2;
    
    // format!
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);

    /*
    String from Rust's perspective:
    - bytes: `String` is a wrapper over a `Vec<u8>`.
    - scalar values: Unicode scalar value in a string takes 2 bytes of storage.
    - grapheme clusters: the closest things to what we would call 'letters'.
    */ 

    // iterating each char
    for c in "abc".chars() {
        println!("{}", c);
    }

    // iterating each byte
    for b in "abc".bytes() {
        println!("{}", b);
    }

    // 3. Hash Maps

    // creating
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing
    let team_name = String::from("Blue");
    let score = scores
        .get(&team_name) // get the value associated with the key.
        .copied() // copy the content of the reference.
        .unwrap_or(0); // if the key is not found, set `score` to 0.

    // in an arbitrary order
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwriting
    scores.insert(String::from("Blue"), 25);

    // existence check
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores
        .entry(String::from("Blue")) // returns an enum `Entry`.
        // If the key is found, return a mutable reference.
        // If the key is not found, insert the value and return it.
        .or_insert(50); 
    scores.entry(String::from("Yellow")).or_insert(50);

    // updating the old value
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", map);
}