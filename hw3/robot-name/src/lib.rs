extern crate rand;

pub struct Robot {
    name: String,
}

pub fn create_name() -> String {
    let alpha = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let mut name = "".to_string();
    for _i in 0..2 {
        let index = rand::random::<usize>() % 26;
        name.push(alpha[index].clone());
    }
    for _i in 0..3 {
        let index = rand::random::<usize>() % 10;
        name.push_str(index.to_string().as_str());
    }
    name
}

impl Robot {
    pub fn new() -> Self {
        let robot = Robot {
            name: create_name(),
        };
        robot
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = create_name();
    }
}
