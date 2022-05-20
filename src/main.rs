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
fn solve(quizes: &Quizes) {
    let all = quizes.len();
    let mut correct = 0;
    let mut wrong: Quizes = Vec::new();

    println!("何問解きますか?1以上{}以下。整数で入力してください", all);
    let mut numofq = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut numofq).ok();
    let mut numofq: usize = numofq.trim().parse().unwrap();

    if numofq > all {
        numofq = all;
    } else if numofq < 1 {
        numofq = 1;
    }
    for (i, q) in quizes.iter().enumerate() {
        if numofq == i {break};
        println!("問題{}:{}", i+1, q.question);
        let mut ans = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut ans).ok();
        let ans: String = ans.trim().parse().unwrap();
        if q.judge(&ans) {
            println!("正解です");
            correct += 1;
        } else {
            println!("不正解です。\n正解は{}です", q.answer);
            wrong.push(Quiz::new(&q.question, &q.answer));
        } 
    }
    println!("{}問中 {}問正解です", numofq, correct);
    loop {
        if (wrong.len() > 0) {
            println!("間違えた問題を復習しますか？(y/n)");
            let mut ans = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut ans).ok();
            let ans: String = ans.trim().parse().unwrap();
            if ans == "y" || ans == "Y" {
                solve(&wrong);
                break;
            } else {
                println!("bye");
                break;
            }
        } else {
            break;
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
    let mut quizes = load_question(file_name);
    solve(&quizes);
}
