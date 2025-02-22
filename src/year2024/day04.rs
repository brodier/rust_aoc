
fn day4_step1(chars:Vec<&[u8]>) -> usize {
    let search_index:[[(i32,i32);3];8] = [
        [(0,1),(0,2),(0,3)], // DOWN
        [(1,1),(2,2),(3,3)], // DIAG DOWN-RIGHT
        [(1,0),(2,0),(3,0)], // RIGHT
        [(1,-1),(2,-2),(3,-3)], // DIAG RIGHT-UP
        [(0,-1),(0,-2),(0,-3)], //UP
        [(-1,-1),(-2,-2),(-3,-3)], // DIAG UP-LEFT
        [(-1,0),(-2,0),(-3,0)], // LEFT
        [(-1,1),(-2,2),(-3,3)], // DIAG LEFT-DOWN
    ];
    let search_patter = [b'M',b'A',b'S'];

    let line_width = chars.get(0).unwrap().len() as i32;
    let line_height = chars.len() as i32;
    let mut counter = 0;
    for x in 0..line_width as usize {
        for y in 0..line_height as usize {
            let letter = chars.get(y).unwrap().get(x).unwrap();
            if *letter == b'X' {
                'ind: for indexes in search_index {
                    let mut let_itt = 0;
                    for (a,b) in indexes {
                        let new_x = x as i32 + a;
                        let new_y = y as i32 + b;
                        if new_x >=0 && new_y >=0 && new_y < line_height  && new_x < line_width{
                            let next_letter = chars.get(new_y as usize).unwrap().get(new_x as usize).unwrap();
                            if *next_letter != search_patter[let_itt] {
                                continue 'ind;
                            }
                        } else {
                            continue 'ind;
                        }
                        let_itt+=1;
                    }
                    counter+=1;
                }
            }
        }
    }
    counter
}

fn pick_letter(chars:&Vec<&[u8]>, indexes:[(i32,i32);4], a_x_pos:i32, a_y_pos:i32) -> [u8;4] {
    let mut i = 0;
    let mut result:[u8;4] = [0,0,0,0];
    for (a,b) in indexes {
            let new_x = a +a_x_pos;
            let new_y = b + a_y_pos;
            let next_letter = chars.get(new_y as usize).unwrap().get(new_x as usize).unwrap();
            result[i] = *next_letter;
            i+=1;
    }
    return result;
}

fn day4_step2(chars:Vec<&[u8]>) -> usize {
    let x_indexes:[(i32,i32);4] = [(1,1),(-1,-1),(1,-1),(-1,1)];
    let search_pattern = [
        [b'M',b'S',b'M',b'S'],
        [b'M',b'S',b'S',b'M'],
        [b'S',b'M',b'M',b'S'],
        [b'S',b'M',b'S',b'M']
    ];
    let line_width = chars.get(0).unwrap().len() as i32;
    let line_height = chars.len() as i32;
    let mut counter = 0;
    for x in 1..line_width-1 {
        for y in 1..line_height-1 {
            let letter = chars.get(y as usize).unwrap().get(x as usize ).unwrap();
            if *letter == b'A' {
                let x_letters = pick_letter(&chars, x_indexes, x, y);
                for pattern in search_pattern {
                    if pattern == x_letters {
                        counter+=1;
                    }
                }
            }
        }
    }
    counter
}

pub fn solve(step:usize, contents:String) -> usize {
    let lines = contents.lines();
    let mut chars = Vec::new();
    for line in lines {
        chars.push(line.as_bytes());
    }    
    if step == 1 {
        return day4_step1(chars);
    } else {
        return day4_step2(chars);
    }

}