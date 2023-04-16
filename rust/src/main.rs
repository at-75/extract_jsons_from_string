// use std::{collections::VecDeque, slice::range};
fn extract(s: String) -> Vec<String>{
    let n = s.len();
    let mut m: usize = 0;
    let mut v : Vec<String>   = vec![];
    println!("{}",n);
    for mut i in 0..n{
        m=i;
        if s.chars().nth(i)==Some('{') {
           let first_bracket_index=i;
           let mut second_bracket_index=i;
           let mut score=1;
           while score>0 && m<n {
                m+=1;
                if s.chars().nth(m)==Some('{') {
                    score+=1;
                }
                else if s.chars().nth(m)==Some('}'){
                    score-=1;
                }
                i=m;
                second_bracket_index=i+1;
           }
           v.push(s.get(first_bracket_index..second_bracket_index).unwrap().to_string());
        }
    }
    for s in &v {
        println!("{}", s);
    }
    return v;
}
fn main() { 
    extract("{\"name\": \"Alice\",\"age\": 25,\"isStudent\": true,\"favoriteFoods\": [\"pizza\", \"sushi\", \"ice cream\"]}".to_owned());
}
