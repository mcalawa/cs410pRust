pub struct Equation {
    first_num: i64,
    operator: String,
    second_num: i64,
}

impl Equation {
    pub fn new(first: i64, op: String, second: i64) -> Self {
        let eq = Equation {
            first_num : first,
            operator : op,
            second_num : second,
        };
        eq
    }
}

pub struct WordProblem {
    problems: Vec<Equation>,
}

impl WordProblem {
    pub fn new(command: &str) -> Self {
        let mut wp = WordProblem {
            problems: Vec::new()
        };
        
        let mut first = 0;
        let mut op = "".to_string();
        let mut second = 0;
        let command2 = command.replace("?", "");
        let mut words = command2.split_whitespace();
        let length = words.clone().count();

        for _i in 0..length {
            let mut word = words.next().unwrap();
            if word.chars().all(|x| x.is_numeric()) {
                if op.is_empty() {
                    first = word.parse::<i64>().unwrap();
                }
                else {
                    second = word.parse::<i64>().unwrap();
                    wp.problems.push(Equation::new(first, op.clone(), second));
                    first = 0;
                }
            }
            else if word.starts_with("-") {
                if word[1..].chars().all(|x| x.is_numeric()) {
                    if op.is_empty() {
                        first = word.parse::<i64>().unwrap();
                    }
                    else {
                        second = word.parse::<i64>().unwrap();
                        wp.problems.push(Equation::new(first, op.clone(), second));
                        first = 0;
                    }
                }
            }
            else if word.to_string() != "What" && word.to_string() != "is" && word.to_string() != "by" {
                if op.is_empty() {
                    op.push_str(word.clone());
                }
                else {
                    op = word.clone().to_string();
                }
            }
        }
        wp
    }

    pub fn answer(&mut self) -> Result<i64, &str> {
        let mut solution = 0;
        let mut op = "".to_string();
        let length = self.problems.len();
        if length < 1 {
            Err("Error!")
        }
        else {
            for i in 0..length {
                op = self.problems[i].operator.to_string();
                if op == "plus" {
                    solution = self.problems[i].first_num + self.problems[i].second_num;
                }
                else if op == "minus" {
                    solution = self.problems[i].first_num - self.problems[i].second_num;
                }
                else if op == "multiplied" {
                    solution = self.problems[i].first_num * self.problems[i].second_num;
                }
                else if op == "divided" {
                    solution = self.problems[i].first_num / self.problems[i].second_num;
                }
                else {
                    return Err("Error!")
                }

                if i + 1 < length {
                    self.problems[i + 1].first_num = solution;
                }
            }
            Ok(solution)
        }
    }
}
