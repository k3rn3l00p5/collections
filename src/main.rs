// removes dead code and unused variable/import warnings
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// the data these collections point to are stored on the heap
// amount of data does not need to be known at compile time and can grow or shrink as the program runs
// commonly used collections are vector, string and hash map

// vector- allows you to store a variable number of values next to each other
// string - collection of characters
// hash map - associates a value with a particular key

fn main() {
    // Vec<T> allows you to store more than one value in a single data structure
    // all the values are put next to each other in memory
    // Vectors can only store values of the same type
    let v: Vec<i32> = Vec::new();
    // Type annotations are required here because we aren't inserting values into the Vector on definition
    // Since rust can't infer what values we intend to store we need type annotation
    // <T> means it can hold any type

    // using type annotations are rate because when we create vectors we usually add the values in them
    let v = vec![1, 2, 3]; // macro that turns a set of numbers into a vector
                           // updating a vector is also possible (has to be mutable)
                           // use a push method to add elements to it
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // there are two ways to reference a value stored in a vector
    // indexing syntax or get method are both ways to access vector values
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // using a reference allows you to modify the vector data directly
                             // if you don't use & it makes new data (won't be vector's data)

    println!("Third element is {}", third);

    // get method gives us an Option<&T> or a reference to the value if any
    // if the get method returns None Option enum then there is no value
    // this method is useful for handling when a value doesn't exist
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // you can also access each element in a vector by iterating through all the elements
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // for i is the iterator
    // in &mut v is the collection
    for i in &mut v {
        // dereferences the address to the content and changes it directly
        *i += 50;
    }

    // vectors hold the same type and since values in an enums are considered to be of the same types
    // we can have variants with different types inside an enum and store the enum values in a vector
    enum DifferentTypes {
        // enumerators allows us to give a name to a value
        // enums also associate values to a type (DifferentTypes)
        // Enumerators also allow you to restrict the amount of data in a new enumerator associated to a new name
        Int(i32), // i32 has a name called Int
        Float(f64),
        Text(String),
    }

    use DifferentTypes::*;

    // has to be mutable if you want to be able to change the new vector
    let mut row = vec![
        // DifferentTypes is the data type of these values
        // Int is the name used to store a i32 under a type called DifferentTypes
        Int(3), // creating new values using enum template associated with data allowed
        Text(String::from("blue")),
        Float(10.12),
        // all three values are stored under the enum type
    ];

    // creates a mutable reference to the first value in the vector row
    let vec_value: &mut DifferentTypes = &mut row[0];

    // Pattern Matching to find if the vec_value is same as the DifferentType variant
    match vec_value {
        // if it is then add 20 to the value inside
        DifferentTypes::Int(value) => {
            *value += 20;
            println!("Changed Value: {}", value)
        }
        // if anything else than print this
        _ => println!("Something else"),
    }

    // if let statement version for more concise code
    if let DifferentTypes::Int(value) = vec_value {
        println!("Value: {}", value);
    }

    // Rust only has one string type in the core language
    // It's a string slice str that is usually seen in its borrowed form &str
    // String type is from Rust's standard library and it's a growable mutable owned UTF-8 encoded string type
    // When Rustaceans refer to strings in rust they usually mean the String and the string slice &str types
    // Rust has other standard library string types (OsString, OsStr, CString, and CStr)

    // Creating a new string
    // new empty string called s which is mutable
    let s = String::new();
    // to load some initial data into a string use the to_string method
    let data = "initial contents";
    let s = data.to_string();
    // String::form can also be used to turn a literal into a string
    let s = String::from("initial contents");
    // we can make a string from anything supported by UTF-8 encoding
    let s = String::from("नमस्ते");

    // Updating a string
    // you can append to a string with push_str and push
    let mut s = String::from("foo");
    // takes a string slice because we don't want to take ownership of the parameter so it's still available for use after the method is called
    s.push_str("bar");
    // push method takes a single character and appends it to the string
    s.push('.');

    // String concatenation using +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // String concatenation using format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // format macro is better because it doesn't take ownership of any parameters

    // you can't index strings because an index into a string's bytes will not always correlate to a valid unicode scalar value
    // instead use a slicing
    let hello = "Hi";
    let s = &hello[0..1];
    // create string slices with caution because if they go out of char boundaries it will panic rust code

    // It's better to perform operations on individual unicode scalar values by using chars or byte methods
    for c in "sad".chars() {
        println!("{}", c);
    }
    for b in "happy".bytes() {
        println!("{}", b);
    }

    // The type HashMap<K, V> stores a mapping of keys of type K to values of type V
    // This is done through a hashing function
    // hash maps are useful when you want to look up data not by using an index but by using a key that can be of any type

    // creating a new hash map (bring it into the scope first)
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    // inserting new values into a hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // all keys must have the same type and all values must have the same type

    // another way of constructing a hash map is by using the collect method on a vector of tuples
    // the collect method gathers data into a number of collection types
    // the zip method is used to create a vector of tuples
    // creates a vector with two strings
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    // creates a vector with 2 i32s
    let initial_scores = vec![10, 50];
    // the HashMap<_, _> is needed because it's possible that collect gathers data into many different data structures and Rust doesn't know which you want unless you specify
    // the _s in the HashMap parameters allow Rust to infer the types that the hash map contains based on the data from collect
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // for values that use Copy trait like i32 the values are copied into the hashmap
    // for owned values like String the values are moved and the hash map will become the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point because hashmap took ownership

    // accessing values in a hash map
    // you can get a value out of the hash map by providing its key to the get method
    let score = scores.get(&String::from("Blue")); // get returns an Option enum with either a some or none value
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // there are several ways to update a hashmap
    let mut scores = HashMap::new();

    // overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // only insert a value if the key has no value (entry method takes a key you want to check as a parameter)
    scores.entry(String::from("Yellow")).or_insert(50);
    // entry checks if Blue key has a value and or_insert returns a mutable reference to the value if there is one and adds a new one if there isn't
    scores.entry(String::from("Blue")).or_insert(50);

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
} // <-- stuff goes out of scope and is dropped here
  // when stuff is dropped so are all the contents it holds
