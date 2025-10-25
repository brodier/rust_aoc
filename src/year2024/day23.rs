

pub struct LinkByPc {
    links_by_pc:[Option<[bool;26*26]>;26*26],
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
            self.links_by_pc[pc] = Some([false;26*26]);
            self.all_pc.push(pc);
        }
        return (pc, new_pc);
    }

    fn link(&mut self, name1:&str, name2:&str) -> (bool, usize, usize) {
        let (pc1, pc1_is_new) = self.init(name1);
        let (pc2, pc2_is_new) = self.init(name2);
        self.links_by_pc[pc1].as_mut().unwrap()[pc2] = true;
        self.links_by_pc[pc2].as_mut().unwrap()[pc1] = true;
        (pc1_is_new||pc2_is_new, pc1, pc2)
    }

    fn get_common_pcs(&self, pc1:usize, pc2:usize) -> Vec<bool> {
        let mut common_pcs = vec![false;26*26];
        for i in 0..26*26 {
            if self.links_by_pc[pc1].as_ref().unwrap()[i] && self.links_by_pc[pc2].as_ref().unwrap()[i] {
                common_pcs[i] = true;
            }   
        }
        common_pcs
    }
    // algorithme BronKerbosch1(R, P, X)
    //     si P et X sont vides alors
    //         déclarer que R est une clique maximale
    //     pour tout sommet v dans P faire
    //         BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
    //         P := P \ {v}
    //         X := X ⋃ {v}
    fn bord_kerbosch(&self, r:Vec<usize>,mut p:Vec<usize>, mut x:Vec<usize>) -> Option<Vec<usize>> {
        if p.is_empty() && x.is_empty() {
            return Some(r);
        }
        let mut best_result:Option<Vec<usize>> = None;
        for &v in p.clone().iter() {
            let n_v = self.links_by_pc[v].as_ref().unwrap();
            let mut new_r = r.clone();
            new_r.push(v);
            let mut p_inter_n_v  = p.clone();
            p_inter_n_v.retain(|&pc| n_v[pc]);
            let mut x_inter_n_v  = p.clone();
            x_inter_n_v.retain(|&pc| n_v[pc]);
            if let Some(result) = self.bord_kerbosch(new_r, p_inter_n_v, x_inter_n_v) {
                if best_result.is_none() || result.len() > best_result.as_ref().unwrap().len() {
                    best_result = Some(result);
                }
            }
            p.retain(|&pc| pc != v);
            x.push(v);
        }
        best_result
    }
}



fn _build_best_group(links_by_pc:&LinkByPc, groups:&Vec<(usize,usize,usize)>) -> Vec<usize> {
    let mut best_group = Vec::new();
    for &(pc1,pc2,pc3) in groups.iter() {
        let mut new_group = vec![pc1,pc2,pc3];
        for &pc in links_by_pc.all_pc.iter() {
            let pc_links = links_by_pc.links_by_pc[pc].as_ref().unwrap();
            if new_group.iter().all(|&pc| pc_links[pc]==true) {
                new_group.push(pc);
            }
        }        
        if new_group.len() > best_group.len() {
            best_group = new_group;
        }
    }
    best_group
}

fn get_password(group:&Vec<usize>) -> String {
     let mut result:Vec<String> = group.iter().map(|id| {
        let name = [(id / 26) as u8 + b'a', (id % 26) as u8 + b'a'];
        String::from_utf8(name.to_vec()).unwrap()
    }).collect();
     result.sort();
     result.join(",")
}

pub fn parse(input:String) -> (LinkByPc,Vec<(usize,usize,usize)>) {
     let mut links_by_pc = LinkByPc::build();
    let mut groups = Vec::new();
    for line in input.lines() {
        let (name1,name2) = line.split_once("-").unwrap();
        let (new_pc, pc1, pc2) = links_by_pc.link(name1, name2);
        if !new_pc {
            let pc3_list = links_by_pc.get_common_pcs(pc1, pc2);
            // for step 2 for each pc in pc3_list             
            for (pc3, &linked) in pc3_list.iter().enumerate() {
                if linked {groups.push((pc1,pc2,pc3));}
            }
        }
    }
    (links_by_pc, groups)
}

pub fn part1(input:&(LinkByPc, Vec<(usize,usize,usize)>)) -> String {
            input.1.iter().filter(|&group| {
            ((group.0 / 26) as u8) + b'a' == b't' ||
            ((group.1 / 26) as u8) + b'a' == b't' ||
            ((group.2 / 26) as u8) + b'a' == b't'}).count().to_string()   
}

pub fn part2(input:&(LinkByPc, Vec<(usize,usize,usize)>)) -> String {
    // get_password(&build_best_group(&input.0, &input.1))
    let lbp = &input.0;
    let p = lbp.all_pc.clone();
    get_password(&input.0.bord_kerbosch(Vec::new(), p, Vec::new()).unwrap())
}