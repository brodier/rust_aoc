pub fn solve(step:usize, input:String) -> String {
    let mut result:usize = 0;

    for line in input.lines() {
        let (game, params) = line.split_once(":").unwrap();
        let game_id:usize = (game.split_once(" ").unwrap().1).parse().unwrap();
        let mut game_set:(usize,usize,usize) = (0,0,0);
        for param in params.split(";") {
            for elem in param.split(",") {
                let (nb,color) = elem.trim().split_once(" ").unwrap();
                let nb:usize = nb.parse().unwrap();
                if color=="red" {
                    if nb > game_set.0 {
                        game_set.0 = nb
                    }
                } else if color=="green" {
                    if nb > game_set.1 {
                        game_set.1 = nb
                    }
                } else if color=="blue" {
                    if nb > game_set.2 {
                        game_set.2 = nb
                    }
                }
            }
        }
        if step==1 && game_set.0 <= 12 && game_set.1 <= 13 && game_set.2 <= 14 {
            result += game_id;
        } else if step == 2{
            result += game_set.0 * game_set.1 * game_set.2;
        }
    }
    result.to_string()
}


pub fn parse(input:String) -> String {
    input
}

pub fn part1(input:&String) -> String {
    solve(1, input.clone())
}

pub fn part2(input:&String) -> String {
    solve(2, input.clone())
}