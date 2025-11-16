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

fn parse_line(input:&str) -> (i64, i64) {
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
    (nb_step * offset.0,nb_step * offset.1)
}

pub fn parse(input:String) -> Vec<(i64,i64)> {
    let mut points = Vec::new();
    points.push((0,0));
    input.lines().for_each(|l| {
        let offset:(i64,i64) = parse_line(l);
        let mut new_point = points.last().unwrap().clone();
        new_point.0 += offset.0;
        new_point.1 += offset.1;
        points.push(new_point);
    });
    let last = points.pop().unwrap();
    assert_eq!(last,(0,0));
    points
}

pub fn part1(points:&Vec<(i64,i64)>) -> String {
    println!("{:?}", points);
    gauss_formula_with_boundary(points).to_string()
}

pub fn part2(_:&Vec<(i64,i64)>) -> String {
    "2".to_string()
}
