fn filter(group:&(usize,usize,usize)) -> bool {
    ((group.0 / 26) as u8) + b'a' == b't' ||
    ((group.1 / 26) as u8) + b'a' == b't' ||
    ((group.2 / 26) as u8) + b'a' == b't'
}


fn get_password(group:&Vec<usize>) -> String {
     let mut result:Vec<String> = group.iter().map(|id| {
        let name = [(id / 26) as u8 + b'a', (id % 26) as u8 + b'a'];
        String::from_utf8(name.to_vec()).unwrap()
    }).collect();
     result.sort();
     result.join(",")
}

struct LinkByPc {
    links_by_pc:[Option<Vec<usize>>;26*26],
    all_pc:Vec<usize>,
}

impl LinkByPc {
    fn build() -> LinkByPc {
        LinkByPc { links_by_pc: [const {None};26*26],all_pc:Vec::new()}
    }

    fn index(pc_name:&str) -> usize {
        let a =pc_name.as_bytes();
        let index = (a[0] - b'a') as usize * 26 + a[1] as usize - b'a' as usize;
        index
    }

    fn init(&mut self, name: &str) -> (usize,bool) {
        let pc = LinkByPc::index(name);
        let mut new_pc = false;
        if self.links_by_pc[pc].is_none() {
            new_pc = true;
            self.links_by_pc[pc] = Some(Vec::new());
            self.all_pc.push(pc);
        }
        return (pc, new_pc);
    }

    fn link(&mut self, name1:&str, name2:&str) -> (bool, usize, usize) {
        let (pc1, pc1_is_new) = self.init(name1);
        let (pc2, pc2_is_new) = self.init(name2);
        self.links_by_pc[pc1].as_mut().unwrap().push(pc2);
        self.links_by_pc[pc2].as_mut().unwrap().push(pc1);
        (pc1_is_new||pc2_is_new, pc1, pc2)
    }

    fn get_common_pcs(&self, pc1:usize, pc2:usize) -> Vec<usize> {
        let list_connected_to_pc1 = self.links_by_pc[pc1].as_ref().unwrap();
        let list_connected_to_pc2 = self.links_by_pc[pc2].as_ref().unwrap();
        list_connected_to_pc2.iter().filter(|&one_pc_connected_to_pc2|
             list_connected_to_pc1.contains(one_pc_connected_to_pc2)).map(|pc| *pc).collect()

    }
}

pub fn solve(part:usize, input:String) -> String {
    let mut links_by_pc = LinkByPc::build();
    let mut best_group:Vec<usize> = Vec::new();
    let mut max_len = 0;
    let mut groups = Vec::new();
    for line in input.lines() {
        let (name1,name2) = line.split_once("-").unwrap();
        let (new_pc, pc1, pc2) = links_by_pc.link(name1, name2);
        if !new_pc {
            let pc3_list = links_by_pc.get_common_pcs(pc1, pc2);
            // for step 2 for each pc in pc3_list             
            for pc3 in pc3_list {
                groups.push((pc1,pc2,pc3));
            }
        }
    }
    if part == 1 {
        groups.iter().filter(|&g| filter(g)).count().to_string()    
    } else {
        let mut seen = [false;26*26];
        for &(pc1,pc2,pc3) in groups.iter() {
            let mut new_group = vec![pc1,pc2,pc3];
            if !(seen[pc1] && seen[pc2] && seen[pc3]) {
                joining_groups(&mut seen, &links_by_pc, &mut new_group);
                let new_len = new_group.len();
                if new_len > max_len {
                    best_group = new_group;
                    max_len = new_len;
                }
            }
        }
        get_password(&best_group)
    }
    
}

fn joining_groups(seen:&mut [bool;26*26], links_by_pc:&LinkByPc, new_group:&mut Vec<usize>) {
    for &pc in links_by_pc.all_pc.iter() {
        let pc_links = links_by_pc.links_by_pc[pc].as_ref().unwrap();
        if new_group.iter().all(|m| pc_links.contains(m)) {
            seen[pc] =true;
            new_group.push(pc);
        }
    }
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