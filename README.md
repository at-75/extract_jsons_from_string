# About 
Extract_JSON_from_string is library to extract valid JSONs from a raw string. If string contains an invalid JSONs it will return an ERROR string

# Rust Installation
Run the following Cargo command in your project directory:
```
cargo add extract_jsons_from_string
```
Or add the following line to your Cargo.toml:
```
extract_jsons_from_string = "0.1.3"
```

# Python Installation
```bash
pip install extract-json-from-string
```

# Python example

```python
from extract_json_from_string import extract

data = '''text before json{
  "name": "John",
  "age": 30,
  "city": "New York",
  "isStudent": true,
  "interests": [
    "reading",
    "hiking",
    "traveling"
  ],
  "address": {
    "street": "123 Main St",
    "city": "Anytown",
    "state": "CA",
    "zip": "12345"
  }
}text after json
'''
print(extract(data))

```

# Rust example
```rust
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
