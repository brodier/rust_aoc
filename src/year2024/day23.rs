use std::collections::HashMap;

fn get_id(pc_name:&str) -> usize {
    let pc = pc_name.as_bytes();
    (pc[1] as usize)* 256 + pc[0] as usize
}


fn filter(group:&(usize,usize,usize)) -> bool {
    ((group.0 & 0xff) as u8) == b't' ||
    ((group.1 & 0xff) as u8) == b't' ||
    ((group.2 & 0xff) as u8) == b't'
}


fn get_password(group:&Vec<usize>) -> String {
     let mut result:Vec<String> = group.iter().map(|id| {
        let name = [(id % 256) as u8, (id >> 8) as u8];
        String::from_utf8(name.to_vec()).unwrap()
    }).collect();
     result.sort();
     result.join(",")
}

fn joining_groups(links_by_pc:&HashMap<usize,Vec<usize>>, new_group:&mut Vec<usize>) {
    for pc in links_by_pc.keys() {
        if new_group.iter().all(|m| links_by_pc.get(pc).unwrap().contains(m)) {
            new_group.push(*pc);
        }
    }
    
}

pub fn solve(part:usize, input:String) -> String {
    let mut links_by_pc:HashMap<usize,Vec<usize>> = HashMap::new();
    let mut multi_group:Vec<Vec<usize>> = Vec::new();
    let mut groups = Vec::new();
    for line in input.lines() {
        let (name1,name2) = line.split_once("-").unwrap();
        let (pc1,pc2) = (get_id(name1), get_id(name2));
        let mut new_pc = false;
        if !links_by_pc.contains_key(&pc1) {
            new_pc = true;
            links_by_pc.insert(pc1, Vec::new());
        }
        if !links_by_pc.contains_key(&pc2) {
            new_pc = true;
            links_by_pc.insert(pc2, Vec::new());
        }
        links_by_pc.get_mut(&pc1).unwrap().push(pc2);
        links_by_pc.get_mut(&pc2).unwrap().push(pc1);
        if !new_pc {
            let mut pc3_list = Vec::new();
            let pc2_list =links_by_pc.get(&pc2).unwrap();
            let pc1_list = links_by_pc.get(&pc1).unwrap();
            pc1_list.iter().for_each(|&pc3| 
                if pc2_list.contains(&pc3) {
                    pc3_list.push(pc3);
                }
            );
            // for step 2 for each pc in pc3_list             
            for pc3 in pc3_list {
                groups.push((pc1,pc2,pc3));

                let mut new_group = vec![pc1,pc2,pc3];
                joining_groups(&links_by_pc, &mut new_group);
                multi_group.push(new_group);
            }
        }
    }
    if part == 1 {
        groups.iter().filter(|&g| filter(g)).count().to_string()    
    } else {
        multi_group.sort_unstable_by(|a,b| b.len().cmp(&a.len()));
        get_password(multi_group.first().unwrap())
    }
    
}