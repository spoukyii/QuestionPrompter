use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use rand::seq::SliceRandom; 
use rand::thread_rng;

fn main() {
    let path = "src/question.txt";
    let file = File::open(path);

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut lines: Vec<String> = reader.lines()
                .filter_map(Result::ok)
                .collect();

            let mut rng = thread_rng();
            lines.shuffle(&mut rng);

            let mut score = 0;
            let mut questions = 0;
            let stdin = io::stdin();
            let mut stdout = io::stdout();

            for line in lines {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() != 2 {
                    println!("Invalid format: {}", line);
                    continue;
                }

                let question = parts[0].trim();
                let correct_answer = parts[1].trim();

                loop {
                    println!("{}", question);
                    questions += 1;
                    print!("Your answer: ");
                    stdout.flush().unwrap();

                    let mut user_answer = String::new();
                    stdin.read_line(&mut user_answer).unwrap();
                    let user_answer = user_answer.trim();

                    if user_answer.eq_ignore_ascii_case(correct_answer) {
                        println!("Correct!\n");
                        score += 1;
                        break;
                    } else {
                        println!("Incorrect. Try again.\n");
                    }
                }
            }

            println!("Final score is {}/{}", score, questions);
        },
        Err(_) => {
            println!("Error reading the file.");
        },
    }
}
