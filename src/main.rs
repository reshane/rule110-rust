use std::fmt;

#[derive(Debug)]
struct State {
    cells: Vec<u8>,
}

impl State {
    fn new() -> Self {
        Self {
            cells: vec![1, 1, 0],
        }
    }

    fn step(&mut self) {
        let mut next_state: Vec<u8> = Vec::new();
        let mut tmp_state: Vec<u8> = vec![0, 0];
        tmp_state.append(&mut self.cells.clone());
        tmp_state.append(&mut vec![0, 0]);
        for i in 2..tmp_state.len() {
            let a = tmp_state[i - 2];
            let b = tmp_state[i - 1];
            let c = tmp_state[i];
            let total = a * 4 + b * 2 + c * 1;
            if total == 4 || total == 7 || total == 0 {
                next_state.push(0);
            } else {
                next_state.push(1);
            }
        }
        self.cells = next_state.clone();
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = String::from("");
        for c in &self.cells {
            res.push_str(&format!("{}", c));
        }
        write!(f, "{}", res)
    }
}

fn main() {
    let mut state: State = State::new();
    println!("{}", state);
    loop {
        let mut input: String = Default::default();
        std::io::stdin().read_line(&mut input).unwrap();
        if input != "\n" {
            break;
        }
        state.step();
        println!("{}", state);
    }
}
