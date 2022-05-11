type Quizes = Vector<Quiz>;

struct Quiz {
    question: String,
    answer: String
}
impl Quiz {
    fn new(question: &str, answer: &str) -> Quiz {
        retutn Quiz {
            question: String::from(question),
            answer: String::from(answer),           
        };
    }
    fn inquire() {
        println!("{}", self.question);
    }
    fn judge(x:&str) -> bool {
        if self.answer == x {
            return true;
        }
        false
    }
}
fn solve(quizes: Quizes) {
    for i in quizes.iter() {

    }
}
fn main() {
    let one = Quiz::new("りんごは果物である", "o");
    println!("Hello, world!");

}
