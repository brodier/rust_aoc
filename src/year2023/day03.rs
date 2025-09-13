

struct Puzzle {
   result:usize,
   cur_part_num:usize,
   adj_to_sym:bool,
   first_line:bool,
   line_len:usize,
   engine_schema:String 
}

impl Puzzle {
    fn build(len:usize,engine_schema:String) -> Puzzle {
        Puzzle{result: 0, cur_part_num: 0, adj_to_sym:false, first_line: true, line_len: len, engine_schema}
    }

    fn eval(&mut self) -> usize {
        let engine_schema = self.engine_schema.clone();
        for (i, c) in engine_schema.char_indices() {
            // if !self.first_line {
            //     eprintln!("up is {}", self.engine_schema.chars().nth(i - self.line_len - 1).unwrap());
            // }
            // if i + self.line_len + 1 < self.engine_schema.len() {
            //     eprintln!("down is {}", self.engine_schema.chars().nth(i + self.line_len + 1).unwrap());
            // }        
            match c {
                '0'..='9' => self.on_digit(i),
                '.' => self.on_period(i),
                '\n' => {
                    self.on_period(i);
                    self.first_line = false;
                },
                _ => self.on_symbol(),
            }
        }
        self.result
    }

    fn on_digit(&mut self, i:usize) {
        if self.adj_to_sym == false && self.up_or_down_is_symbol(i) {
            self.adj_to_sym = true;
        }
        let digit = self.engine_schema.as_bytes()[i]  - b'0';
        self.cur_part_num = self.cur_part_num * 10 + digit as usize;
    }

    fn on_period(&mut self, i:usize) {
        if self.up_or_down_is_symbol(i) {
            self.on_symbol();
        } else {
            self.compute();
        }
    }

    fn compute(&mut self) {
        if self.cur_part_num != 0 && self.adj_to_sym == true {
            self.result += self.cur_part_num;
        }
        self.cur_part_num = 0;
        self.adj_to_sym = false;
    }

    fn on_symbol(&mut self) {
        self.adj_to_sym = true;
        self.compute();
        self.adj_to_sym = true;
    }

    fn is_symbol(c:u8) -> bool {
        match c {
            b'0'..=b'9' => false,
            b'.' => false,
            b'\n' => false,
            _ => true,
        }
    }

    fn up_or_down_is_symbol(&self, i:usize) -> bool {

        let up = if !self.first_line {
            self.engine_schema.as_bytes()[i - self.line_len - 1]
        } else {
            b'.'
        };
        let down = if i + self.line_len + 1 < self.engine_schema.len() {
            self.engine_schema.as_bytes()[i + self.line_len + 1]
        } else {
            b'.'
        };
        let up = Puzzle::is_symbol(up);
        let down = Puzzle::is_symbol(down);
        up || down
    }

}
pub fn solve(_step: usize, input: String) -> String {
    let line_len = input.lines().last().unwrap().len();
    let mut puzzle = Puzzle::build(line_len, input);
    puzzle.eval().to_string()
}
