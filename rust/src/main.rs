// use std::{collections::VecDeque, slice::range};
fn extract(s: String) -> Vec<i32>{
    let n = s.len();
    let m: usize = 0;
    let v   = vec![1, 2, 3];
    for i in 0..n{
        m=i;
        if s.chars().nth(i)==Some('{') {
           let mut score=1;
           while score>0 {
                m+=1;
                if s.chars().nth(i)==Some('{') {
                    score+=1;
                }
                else if s.chars().nth(i)==Some('}'){
                    score-=1;
                }
           }
        }
        
    }
    return v;
}
fn main() { 
    extract("{\"name\": \"Alice\",\"age\": 25,\"isStudent\": true,\"favoriteFoods\": [\"pizza\", \"sushi\", \"ice cream\"]}".to_owned());
}
