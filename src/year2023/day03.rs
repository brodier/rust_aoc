

#[derive(Debug)]
struct Puzzle {
   result1:usize,
   cur_part_num:usize,
   tmp_gear_list:Vec<usize>,
   gears:Vec<Gear>,
   max_gid:usize,
   adj_to_sym:bool,
   first_line:bool,
   line_len:usize,
   engine_schema:String 
}

#[derive(Debug)]
struct Gear {
    id:usize,
    ratio:usize,
    complete:bool
}

impl Gear {
    fn build(id:usize) -> Gear {
        Gear{id, ratio: 0, complete:false}
    }

    fn update(&mut self, part_num:usize) {
        if self.complete == true {
            self.ratio = 0;
        } else if self.ratio == 0 {
            self.ratio = part_num;
        } else {
            self.ratio = self.ratio * part_num;
            self.complete = true;
        }
    }
}
impl Puzzle {
    fn build(len:usize,engine_schema:String) -> Puzzle {
        Puzzle{result1: 0, cur_part_num: 0, tmp_gear_list:Vec::new(), gears:Vec::new(),
             max_gid: 0, adj_to_sym:false, first_line: true, line_len: len, engine_schema}
    }

    fn eval(&mut self) -> usize {
        let engine_schema = self.engine_schema.clone();
        for (i, c) in engine_schema.char_indices() {
            match c {
                '0'..='9' => self.on_digit(i),
                '.' => self.on_period(i),
                '\n' => {
                    self.on_period(i);
                    self.first_line = false;
                },
                '*' => {
                    self.new_gear(i);
                    self.tmp_gear_list.push(i);
                    self.on_symbol();
                    self.tmp_gear_list.push(i);
                },
                _ => self.on_symbol(),
            }
            // eprintln!("on {} debug puzzle => {:?}, {:?}, {}, {:?}", i, self.gears, self.tmp_gear_list, self.cur_part_num, self.adj_to_sym);
        }
        self.result1
    }

    fn get_gears_ratios(&self) -> usize {
        // eprintln!("{:?}", self.gears);
        let mut ratios:usize = 0;
        for gear in self.gears.iter() {
            if gear.complete {
                ratios += gear.ratio;
            }
        }
        ratios
    }

    fn on_digit(&mut self, i:usize) {
        self.up_or_down_is_symbol(i);
        let digit = self.engine_schema.as_bytes()[i]  - b'0';
        self.cur_part_num = self.cur_part_num * 10 + digit as usize;
    }

    fn on_period(&mut self, i:usize) {
        self.up_or_down_is_symbol(i);
        self.compute();
        self.up_or_down_is_symbol(i);
    }

    fn update_gears(&mut self) {
            for gear_id in self.tmp_gear_list.iter() {
                for gear in self.gears.iter_mut() {
                    if gear.id == *gear_id {
                        gear.update(self.cur_part_num);
                    }
                }
            }
    }

    fn compute(&mut self) {
        if self.cur_part_num != 0 && self.adj_to_sym == true {
            self.update_gears();
            self.result1 += self.cur_part_num;
        }
        self.cur_part_num = 0;
        self.tmp_gear_list.clear();
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

    fn new_gear(&mut self, gear_id: usize) {
        if self.max_gid < gear_id {
            self.gears.push(Gear::build(gear_id));
            self.max_gid = gear_id;
        } 
    }

    fn up_or_down_is_symbol(&mut self, i:usize) -> bool {
        let mut up_id=0;
        let up = if !self.first_line {
            up_id = i - self.line_len - 1;
            self.engine_schema.as_bytes()[up_id]
        } else {
            b'.'
        };
        let mut down_id= 0;
        let down = if i + self.line_len + 1 < self.engine_schema.len() {
            down_id= i + self.line_len + 1;
            self.engine_schema.as_bytes()[down_id]
        } else {
            b'.'
        };
        if down == b'*' {
            self.new_gear(down_id);
            self.tmp_gear_list.push(down_id);
        }
        if up == b'*' {
            self.tmp_gear_list.push(up_id);
        }
        let up = Puzzle::is_symbol(up);
        let down = Puzzle::is_symbol(down);
        self.adj_to_sym |= up || down;
        self.adj_to_sym
    }

}
pub fn solve(step: usize, input: String) -> String {
    let line_len = input.lines().last().unwrap().len();
    let mut puzzle = Puzzle::build(line_len, input);
    let result = puzzle.eval().to_string();
    if step == 1 {
        result
    } else {
        puzzle.get_gears_ratios().to_string()
    }
}
