//! # Extract_JSON_from_string
//! Extract_JSON_from_string is library to extract JSONs in a string into a Vector

use std::vec;
/// Extract all the JSONs as Strings from a given String
/// 
/// # Example
/// ```
/// let data = r#"
///    {
///        "name": "John",
///        "age": 30,
///        "isMarried": true,
///        "hobbies": ["reading", "gardening"]
///    }"#;
///    let v: Vec<String> = extract_json_from_string::extract(data);

pub fn extract(s: &str) -> Vec<String>{
    let n = s.len();
    let mut ab_score = 0;
    let mut _m: usize = 0;
    let mut v : Vec<String>   = vec![];
    for mut i in 0..n{
        _m=i;
        if s.chars().nth(i)==Some('{') {
           ab_score+=1;
           let first_bracket_index=i;
           let mut second_bracket_index=i;
           let mut score=1;
           while score>0 && _m<n {
                _m+=1;
                if s.chars().nth(_m)==Some('{') {
                    score+=1;
                }
                else if s.chars().nth(_m)==Some('}'){
                    score-=1;
                }
                i=_m;
                second_bracket_index=i+1;
                if _m==n && score>0  {
                    return vec!["String contains invalid JSONS".to_owned()];
                }
           }
          v.push(s.get(first_bracket_index..second_bracket_index)
          .unwrap()
          .to_string()
          .replace("\n","")
          .replace(" ",""));
        } else if s.chars().nth(i)==Some('}'){
            ab_score-=1;
        }
        if ab_score<0 { 
            return vec!["String contains invalid JSONS".to_owned()]; 
        }
    }
    return v;
}