use std::io;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

type Quizes = Vec<Quiz>;

#[derive(Serialize, Deserialize, Debug)]
struct QuizJson {
    jsonquiz: HashMap <String, HashMap<String, String>>
}
struct Quiz {
    question: String,
    answer: String
}
impl Quiz {
    fn new(question: &str, answer: &str) -> Quiz {
        let mut quiz = Quiz {
            question: String::from(question),
            answer: String::from(answer),           
        };
        quiz
    }
    fn inquire(&self) {
        println!("{}", self.question);
    }
    fn judge(&self, x:&str) -> bool {
        if self.answer == x {
            return true;
        }
        false
    }
}
fn solve(quizes: Quizes) {
    for q in quizes.iter() {
        println!("問題:{}", q.question);
        let mut ans = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut ans).ok();
        let ans: String = ans.trim().parse().unwrap();
        if q.judge(&ans) {
            println!("正解です");
        } else {
            println!("不正解です。\n正解は{}です", q.answer);
        } 
    }
}
fn load_question(file_name: &str) -> Quizes {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let deserialized: QuizJson = serde_json::from_reader(reader).unwrap();
    let mut quizes: Quizes = Vec::new();
    for (k, v) in deserialized.jsonquiz.iter() {
        quizes.push(Quiz::new(&v["question"], &v["answer"]));
    }
    quizes
    
}
fn main() {
    let file_name = "quz.json";
    let quizes = load_question(file_name);
    solve(quizes);
}
