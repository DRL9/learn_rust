use std::collections::HashMap;

fn main() {
    println!("hello world");
    // test_vector();
    // test_string();
    test_hash_map();
}

fn test_hash_map() {
    let mut scores = HashMap::new();
    scores.insert("blue", 2);
    scores.insert("red", 3);
    let score = match scores.get("blue") {
        Option::Some(v) => v,
        None => &0,
    };
    println!("blue's score is {}", score);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let score = match scores.get(&String::from("Blue")) {
        Some(v) => v,
        None => &0,
    };
    println!("Blue is {}", score);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Yellow"), 25);
    scores.entry(String::from("Yellow")).or_insert(11);

    println!("{:?}", scores);
;
}

#[allow(unused_variables)]
fn test_string() {
    let mut s = String::new();
    s.push_str("sde");
    println!("{}", s);
    let data = "hellowr";
    let str = data.to_string();

    println!("{}", str);
    let s1 = String::from("s1");
    let s2 = "s2".to_string();
    let s3 = s1 + &s2;
    let combine_str = format!("{}-{}", s2, s3);
    println!("{}", combine_str);

    let len = String::from("Здравствуйте").len();
    println!("len is {}", len);

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{}", s);

    let zh = "中文";
    println!("{}'s len is {}", zh, zh.len());
    for b in zh.bytes() {
        println!("{}", b)
    }
    for c in zh.chars() {
        println!("{}", c);
    }
}

fn test_vector() {
    let vec1: Vec<i32> = Vec::new();

    let vec2 = vec![1, 2, 3];

    let mut vec3 = Vec::new();

    vec3.push(23);
    vec3.push(123);
    vec3.push(223);
    println!("{}", vec2[1]);
    let zero = match vec3.get(0) {
        Some(v) => v,
        None => &1,
    };
    let zero2 = &vec3[0];

    println!("match {} {}", zero, zero2);

    for i in &vec3 {
        println!("{}", i);
    }
    let c = vec3;

    let mut num_list = vec![Number::Int(12), Number::Long(2344), Number::Float(345.3)];
    for num in &num_list {
        match num {
            Number::Float(v) => println!("f {}", v),
            Number::Int(v) => println!("i {}", v),
            Number::Long(v) => println!("l {}", v),
        };
    }

    let v = num_list.pop();
    for num in &num_list {
        match num {
            Number::Long(v) => println!("l {}", v),
            _ => println!("other "),
        };
    }

    let mut vec4 = vec![
        String::from("122"),
        String::from("12"),
        String::from("1222"),
    ];
    for i in &vec4 {
        println!("vec4: {}", i);
    }

    let mut str1 = String::from("sdjk");
    let mut str2 = &mut str1;
    str2.push_str("sdkjek");
    let str3 = &mut str2;
    str3.push_str("see");
    str2.push_str("see");
    println!("{}", str2);
    println!("{}", str1);
}

enum Number {
    Int(i32),
    Long(u64),
    Float(f32),
}
