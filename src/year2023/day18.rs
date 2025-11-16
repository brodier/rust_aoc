use crate::utils::common::parse_i64;

fn gauss_formula_with_boundary(points:&Vec<(i64,i64)>) -> usize {
    let a = points.last().unwrap();
    let mut a = *a;
    let mut sum:i64 = 0;
    let mut bound = 0;
    for itt in points.iter() {
        let b:(i64,i64) = (itt.0 as i64, itt.1 as i64);
        bound += (a.0 - b.0 + a.1 - b.1).abs();
        sum += a.1 * b.0 - a.0 * b.1;
        a = b;
    }
    println!("sum {} and bound {}", sum, bound);
    sum = sum.abs();
    sum = (sum + bound) / 2 + 1;
    sum as usize
}


fn ascii_hex_to_u8(hex_digit:u8) -> u8 {
    let r = hex_digit - b'0';
    if r > 9 {
        r + b'0' - b'a' + 10
    } else {
        r
    }
}

fn parse_line(input:&str) -> ((i64, i64),(i64,i64)) {
    let bytes = input.as_bytes();
    let dir = bytes[0];
    let &nb_step = parse_i64(input).first().unwrap();
    let offset = match dir {
        b'R' => (1,0),
        b'D' => (0,1),
        b'L' => (-1,0),
        b'U' => (0,-1),
        _ => unreachable!()
    };
    let part1 = (nb_step * offset.0,nb_step * offset.1);
    let hex_start = input.find("#").unwrap();
    let mut nb_step = 0;
    for i in 1..6 {
        nb_step *= 16;
        nb_step += ascii_hex_to_u8(bytes[hex_start + i]) as i64;
    }
    let offset = match bytes[hex_start + 6] {
        b'0' => (1,0),
        b'1' => (0,1),
        b'2' => (-1,0),
        b'3' => (0,-1),
        _ => unreachable!()
    };
    let part2 = (nb_step * offset.0,nb_step * offset.1);
    (part1,part2)
}

pub struct ParsedPuzzleInput {
    part1:Vec<(i64,i64)>,
    part2:Vec<(i64,i64)>
}

pub fn parse(input:String) -> ParsedPuzzleInput {
    let mut points_part1 = Vec::new();
    let mut points_part2 = Vec::new();
    points_part1.push((0,0));
    points_part2.push((0,0));
    input.lines().for_each(|l| {
        let offsets:((i64,i64),(i64,i64)) = parse_line(l);
        let mut new_point = points_part1.last().unwrap().clone();
        new_point.0 += offsets.0.0;
        new_point.1 += offsets.0.1;
        points_part1.push(new_point);
        let mut new_point = points_part2.last().unwrap().clone();
        new_point.0 += offsets.1.0;
        new_point.1 += offsets.1.1;
        points_part2.push(new_point);

    });
    let last = points_part1.pop().unwrap();
    assert_eq!(last,(0,0));
    let last = points_part2.pop().unwrap();
    assert_eq!(last,(0,0));
    ParsedPuzzleInput { part1: points_part1, part2: points_part2 }
}

pub fn part1(points:&ParsedPuzzleInput) -> String {
    gauss_formula_with_boundary(&points.part1).to_string()
}

pub fn part2(points:&ParsedPuzzleInput) -> String {
    gauss_formula_with_boundary(&points.part2).to_string()
}
