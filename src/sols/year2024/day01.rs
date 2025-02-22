use super::load_puzzle;

fn day1_step1(list1:&Vec<usize>,list2:&Vec<usize>) -> usize {
    let mut sum:usize = 0;
    for i in 0..list1.len() {
        let a = list1.get(i).unwrap();
        let b = list2.get(i).unwrap();
        sum += a.abs_diff(*b);
    }
    sum
}

fn day1_step2(list1:&Vec<usize>,list2:&Vec<usize>) -> usize {
    let mut similarity_score = 0;
    for i in 0..list1.len() {
        let a = list1.get(i).unwrap();
        let mut sim_score_4_a = 0;
        for j in 0..list2.len() {
            let b = list2.get(j).unwrap();
            if a==b {
                sim_score_4_a+=1;
            }
        }
        similarity_score += a * sim_score_4_a;
    }
    similarity_score
}

pub fn day1(step:usize) -> usize {
    let mut list1:Vec<usize> = Vec::new();
    let mut list2:Vec<usize> = Vec::new();
    let mut read_l1 = true;
    let contents = load_puzzle(1);
    for val in contents.split_whitespace().into_iter()  {
        let num:usize = val.parse().unwrap();
        if read_l1 {
            list1.push(num);
            read_l1=false;
        } else {
            list2.push(num);
            read_l1=true;
        }
    }
    list1.sort();
    list2.sort();
    if step == 1 {
        day1_step1(&list1, &list2)
    } else {
        day1_step2(&list1, &list2)
    }
    
}
