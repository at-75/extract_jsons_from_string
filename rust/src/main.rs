use extract_jsons_from_string::extract;

fn main() { 

    let data = r#"{
     "name": "John",
     "age": 30,
     "isMarried": true,
     "hobbies": ["reading", "gardening"]
    }
    "#;

    let v: Vec<String> = extract(data);

    for s in &v {
        println!("{}", s);
    }
}
