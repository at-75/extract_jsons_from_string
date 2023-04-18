# About
extract_json_from_string is library to extract valid JSONs from a string and store them in an vector

# Usage 

```
use extract_jsons_from_string::extract;

fn main() { 

    let data = r#"sample text before json {
     "name": "Abigail",
     "age": 34,
     "isMarried": true,
     "hobbies": ["reading", "gardening"]
    } sample text after json
    {
     "name": "Morris",
     "age": 45,
     "isMarried": false,
     "hobbies": ["cycling", "swimming"]
    } sample text after json
    "#;

    let v: Vec<String> = extract(data);

    // Output : {"name":"Abigail","age":34,"isMarried":true,"hobbies":["reading","gardening"]}
    //           {"name":"Morris","age":45,"isMarried":false,"hobbies":["cycling","swimming"]}  

    for s in &v {
        println!("{}", s);
    }
}
```
