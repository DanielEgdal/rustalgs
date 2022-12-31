use std::collections::VecDeque;
// use std::collections::HashSet;
use std::collections::HashMap;
use std::{
    fs::read_to_string};

// type Cube = (u64, u64);

// #[inline(always)]
pub fn swap5(k: u64, pos1: u8, pos2: u8) -> u64{
    let set1 =  (k >> pos1) & 31;
    let set2 = (k >> pos2) & 31;
    let mut xor = set1 ^ set2;
    xor = (xor << pos1) | (xor << pos2);
    let ret_val = k ^ xor;
    return ret_val;
}

pub fn twist_corner(k:u64) -> u64{
    let u = k & 24;
    let o = (u+8+((u&16)>>1))&24;
    return o + (k&7)
}

pub fn twist_corner_c(k:u64) -> u64{

    let u = k & 24;
    let t = u + 24;
    let o = (t - ((t & 16)>>1)) & 24;
    return o + (k&7)
}

// (mut c, mut e): Cube
pub fn u(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = (c >> 20) & 31;
    let block2 = (c >> 25) & 31;
    let block3 = (c >> 30) & 31;
    let block4 = (c >> 35) & 31;

    c = c & 34084861509631;
    c = c ^ (( block1 << 25) | ( block2 << 35) |( block3 << 20 ) |( block4 << 30));

    let block1 = (e >> 40) & 31;
    let block2 = (e >> 45) & 31;
    let block3 = (e >> 50) & 31;
    let block4 = (e >> 55) & 31;
    e = e & 1099511627775;
    e = e ^ (( block1 << 45) | ( block2 << 55) |( block3 << 40) |( block4 << 50));

    return (c,e);
}

pub fn up(mut c:u64, mut e: u64)->(u64,u64){
    let block1 = (c >> 20) & 31;
    let block2 = (c >> 25) & 31;
    let block3 = (c >> 30) & 31;
    let block4 = (c >> 35) & 31;

    c = c & 34084861509631;
    c = c ^  ((block1 << 30) | (block2 << 20) |(block3 << 35 ) |(block4 << 25));

    let block1 = (e >> 40) & 31;
    let block2 = (e >> 45) & 31;
    let block3 = (e >> 50) & 31;
    let block4 = (e >> 55) & 31;
    e = e & 1099511627775;
    e = e ^ ((block1 << 50) | (block2 << 40) |(block3 << 55) |(block4 << 45));

    return (c,e);
}

pub fn u2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,20,35);
    nc = swap5(nc,30,25);

    let mut ne = swap5(e,40,55);
    ne = swap5(ne,45,50);

    return (nc,ne);
}

pub fn d(mut c:u64, mut e: u64)->(u64,u64){
    let block1 = (c) & 31;
    let block2 = (c >> 5) & 31;
    let block3 = (c >> 10) & 31;
    let block4 = (c >> 15) & 31;

    c = c & 35184371040256;
    c = c ^ ((block1 << 5) | (block2 << 15) |(block3) |(block4 << 10));

    let block1 = (e) & 31;
    let block2 = (e >> 5) & 31;
    let block3 = (e >> 10) & 31;
    let block4 = (e >> 15) & 31;
    e = e & 1152921504605798400;
    e = e ^ ((block1 << 5) | (block2 << 15) |(block3) |(block4 << 10));

    return (c,e);
}

pub fn dp(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = (c) & 31;
    let block2 = (c >> 5) & 31;
    let block3 = (c >> 10) & 31;
    let block4 = (c >> 15) & 31;

    c = c & 35184371040256;
    c = c ^ ((block1 << 10) | (block2) |(block3 << 15) |(block4 << 5));

    let block1 = (e) & 31;
    let block2 = (e >> 5) & 31;
    let block3 = (e >> 10) & 31;
    let block4 = (e >> 15) & 31;
    e = e & 1152921504605798400;
    e = e ^ ((block1 << 10) | (block2) |(block3 << 15) |(block4 << 5));

    return (c,e);
}

pub fn d2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,0,15);
    nc = swap5(nc,5,10);

    let mut ne = swap5(e,0,15);
    ne = swap5(ne,5,10);

    return (nc,ne);
}

pub fn f2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,20,15);
    nc = swap5(nc,25,10);

    let mut ne = swap5(e,15,40);
    ne = swap5(ne,30,35);

    return (nc,ne);
}

pub fn l2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,35,15);
    nc = swap5(nc,25,5);

    let mut ne = swap5(e,45,5);
    ne = swap5(ne,35,20);

    return (nc,ne);
}

pub fn b2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,30,5);
    nc = swap5(nc,35,0);

    let mut ne = swap5(e,25,20);
    ne = swap5(ne,55,0);

    return (nc,ne);
}

pub fn r2(c:u64, e: u64)->(u64,u64){
    // let nc =  swap5(swap5(swap5(c,20,25),30,35),20,35);
    // let ne =  swap5(swap5(swap5(e,40,45),55,50),40,55);
    let mut nc = swap5(c,10,30);
    nc = swap5(nc,20,0);

    let mut ne = swap5(e,10,50);
    ne = swap5(ne,30,25);

    return (nc,ne);
}


pub fn f(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner((c >> 10) & 31);
    let block2 = twist_corner_c((c >> 15) & 31);
    let block3 = twist_corner_c((c >> 20) & 31);
    let block4 = twist_corner((c >> 25) & 31);
    c = c & 35183298348031;
    c = c ^ ((block1 << 15) | (block2 << 25) |(block3 << 10) |(block4 << 20));

    let block1 = ((e >> 15) & 31)^16;
    let block2 = ((e >> 30) & 31) ^16;
    let block3 = ((e >> 35) & 31) ^16;
    let block4 = ((e >> 40) & 31) ^16;
    e = e & 1152886321307484159;
    e = e ^ ((block1 << 35) | (block2 << 15) |(block3 << 40) |(block4 << 30));
    return (c,e);
}

pub fn fp(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner((c >> 10) & 31);
    let block2 = twist_corner_c((c >> 15) & 31);
    let block3 = twist_corner_c((c >> 20) & 31);
    let block4 = twist_corner((c >> 25) & 31);
    c = c & 35183298348031;
    c = c ^ ((block1 << 20) | (block2 << 10) |(block3 << 25) |(block4 << 15));

    let block1 = ((e >> 15) & 31)^16;
    let block2 = ((e >> 30) & 31) ^16;
    let block3 = ((e >> 35) & 31) ^16;
    let block4 = ((e >> 40) & 31) ^16;
    e = e & 1152886321307484159;
    e = e ^ ((block1 << 30) | (block2 << 40) |(block3 << 15) |(block4 << 35));
    return (c,e);

}

pub fn l(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner_c((c >> 5) & 31);
    let block2 = twist_corner((c >> 15) & 31);
    let block3 = twist_corner_c((c >> 25) & 31);
    let block4 = twist_corner((c >>35) & 31);
    c = c & 34118178995231;
    c = c ^ ((block1 << 35) | (block2 << 5) |(block3 << 15) |(block4 << 25));

    let block1 = (e >> 5) & 31;
    let block2 = (e >> 20) & 31;
    let block3 = (e >> 35) & 31;
    let block4 = (e >> 45) & 31;
    e = e & 1151829723887696927;
    e = e ^ ((block1 << 20) | (block2 << 45) |(block3 << 5) |(block4 << 35));


    return (c,e);
}

pub fn lp(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner_c((c >> 5) & 31);
    let block2 = twist_corner((c >> 15) & 31);
    let block3 = twist_corner_c((c >> 25) & 31);
    let block4 = twist_corner((c >>35) & 31);
    c = c & 34118178995231;
    c = c ^ ((block1 << 15) | (block2 << 25) |(block3 << 35) |(block4 << 5));

    let block1 = (e >> 5) & 31;
    let block2 = (e >> 20) & 31;
    let block3 = (e >> 35) & 31;
    let block4 = (e >> 45) & 31;
    e = e & 1151829723887696927;
    e = e ^ ((block1 << 35) | (block2 << 5) |(block3 << 45) |(block4 << 20));

    return (c,e);
}

pub fn r(mut c:u64,mut e: u64)->(u64,u64){

    let block1 = twist_corner_c((c >> 10) & 31);
    let block2 = twist_corner((c >> 20) & 31);
    let block3 = twist_corner_c((c >> 30) & 31);
    let block4 = twist_corner((c) & 31);
    c =  c & 35151053554656;
    c = c ^ ((block1 << 20) | (block2 << 30) |(block3) |(block4 << 10));

    let block1 = (e >> 10) & 31;
    let block2 = (e >> 25) & 31;
    let block3 = (e >> 30) & 31;
    let block4 = (e >> 50) & 31;
    e = e & 1118018573168509951;
    e = e ^ ((block1 << 30) | (block2 << 10) |(block3 << 50) |(block4 << 25));

    return (c,e);
}

pub fn rp(mut c:u64,mut e: u64)->(u64,u64){

    let block1 = twist_corner_c((c >> 10) & 31);
    let block2 = twist_corner((c >> 20) & 31);
    let block3 = twist_corner_c((c >> 30) & 31);
    let block4 = twist_corner((c) & 31);
    c =  c & 35151053554656;
    c = c ^ ((block1) | (block2 << 10) |(block3 << 20) |(block4 << 30));

    let block1 = (e >> 10) & 31;
    let block2 = (e >> 25) & 31;
    let block3 = (e >> 30) & 31;
    let block4 = (e >> 50) & 31;
    e = e & 1118018573168509951;
    e = e ^ ((block1 << 25) | (block2 << 50) |(block3 << 10) |(block4 << 30));

    return (c,e);
}

pub fn b(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner_c((c) & 31);
    let block2 = twist_corner((c >> 5) & 31);
    let block3 = twist_corner((c >> 30) & 31);
    let block4 = twist_corner_c((c >> 35) & 31);
    c = c & 34085934201856;
    c = c ^ ((block1 << 30) | (block2) |(block3 << 35) |(block4 << 5));

    let block1 = ((e) & 31)^16;
    let block2 = ((e >> 20) & 31) ^16;
    let block3 = ((e >> 25) & 31) ^16;
    let block4 = ((e >> 55) & 31) ^16;
    e = e & 36028795946270688;
    e = e ^ ((block1 << 25) | (block2) |(block3 << 55) |(block4 << 20));

    return (c,e);
}

pub fn bp(mut c:u64,mut e: u64)->(u64,u64){
    let block1 = twist_corner_c((c) & 31);
    let block2 = twist_corner((c >> 5) & 31);
    let block3 = twist_corner((c >> 30) & 31);
    let block4 = twist_corner_c((c >> 35) & 31);
    c = c & 34085934201856;
    c = c ^ ((block1 << 5) | (block2 << 35) |(block3 ) |(block4 << 30));

    let block1 = ((e) & 31)^16;
    let block2 = ((e >> 20) & 31) ^16;
    let block3 = ((e >> 25) & 31) ^16;
    let block4 = ((e >> 55) & 31) ^16;
    e = e & 36028795946270688;
    e = e ^ ((block1 << 20) | (block2 << 55) |(block3) |(block4 << 25));

    return (c,e);
}

pub fn m(c:u64,mut e: u64)->(u64,u64){
    let block1 = ((e) & 31)^16;
    let block2 = ((e >> 15) & 31) ^16;
    let block3 = ((e >> 40) & 31) ^16;
    let block4 = ((e >> 55) & 31) ^16;
    e = e & 35994712157487072;
    e = e ^ ((block1 << 55) | (block2) |(block3 << 15 ) |(block4 << 40));
    (c,e)
}

pub fn mp(c:u64,mut e: u64)->(u64,u64){
    let block1 = ((e) & 31)^16;
    let block2 = ((e >> 15) & 31) ^16;
    let block3 = ((e >> 40) & 31) ^16;
    let block4 = ((e >> 55) & 31) ^16;
    e = e & 35994712157487072;
    e = e ^ ((block1 << 15) | (block2 << 40) |(block3 << 55 ) |(block4));
    (c,e)
}

pub fn s(c:u64,mut e: u64)->(u64,u64){
    let block1 = ((e) & 5)^16;
    let block2 = ((e >> 10) & 31) ^16;
    let block3 = ((e >> 45) & 31) ^16;
    let block4 = ((e >> 50) & 31) ^16;
    e = e & 1116927891959939103;
    e = e ^ ((block1 << 45) | (block2 << 5) |(block3 << 50 ) |(block4 << 10));
    (c,e)
}

pub fn sp(c:u64,mut e: u64)->(u64,u64){
    let block1 = ((e) & 5)^16;
    let block2 = ((e >> 10) & 31) ^16;
    let block3 = ((e >> 45) & 31) ^16;
    let block4 = ((e >> 50) & 31) ^16;
    e = e & 1116927891959939103;
    e = e ^ ((block1 << 10) | (block2 << 50) |(block3 << 5 ) |(block4 << 45));
    (c,e)
}

pub fn fw(mut c:u64,mut e: u64)->(u64,u64){
    (c,e) = f(c,e);
    (c,e) = s(c,e);
    (c,e)
}

pub fn fwp(mut c:u64,mut e: u64)->(u64,u64){
    (c,e) = fp(c,e);
    (c,e) = sp(c,e);
    (c,e)
}

pub fn rw(mut c:u64,mut e: u64)->(u64,u64){
    (c,e) = r(c,e);
    (c,e) = mp(c,e);
    (c,e)
}

pub fn rwp(mut c:u64,mut e: u64)->(u64,u64){
    (c,e) = rp(c,e);
    (c,e) = m(c,e);
    (c,e)
}


pub fn perform_move(movee: u8, c: u64, e: u64) -> (u64, u64) {
    let (nc, ne) = match movee{
        1 => r(c,e),
        2 => l(c,e),
        3 => u(c,e),
        4 => d(c,e),
        5 => f(c,e),
        6 => b(c,e),
        7 => rw(c,e),
        8 => fw(c,e),
        9 => s(c,e),
        11 => r2(c,e),
        12 => l2(c,e),
        13 => u2(c,e),
        14 => d2(c,e),
        15 => f2(c,e),
        16 => b2(c,e),
        21 => rp(c,e),
        22 => lp(c,e),
        23 => up(c,e),
        24 => dp(c,e),
        25 => fp(c,e),
        26 => bp(c,e),
        27 => rwp(c,e),
        28 => fwp(c,e),
        29 => sp(c,e),
        0 => (c,e),
        _ => unreachable!()
    };
    return (nc,ne)
}

pub fn convert_scramble(scramble:String) -> Vec<u8>{
    let split_scramble: Vec<_> = scramble.split_ascii_whitespace().map(|f|
        match f {
            "R" => 1,
            "L" => 2,
            "U" => 3,
            "D" => 4,
            "F" => 5,
            "B" => 6,
            "r" => 7,
            "f" => 8,
            "S" => 9,
            "R2" => 11,
            "L2" => 12,
            "U2" => 13,
            "D2" => 14,
            "F2" => 15,
            "B2" => 16,
            "R'" => 21,
            "L'" => 22,
            "U'" => 23,
            "D'" => 24,
            "F'" => 25,
            "B'" => 26,
            "r'" => 27,
            "f'" => 28,
            "S'" => 29,
            _ => unreachable!()
        }).collect();
    split_scramble
}


pub fn mk_inv(sol: &[u8;15])->Vec<u8>{
    let mut reverse_sol = Vec::new();
    for movee in sol.iter().rev(){
        if *movee >0{
            if *movee < 10{
                reverse_sol.push(*movee + 20);
            }
            else if *movee > 20{
                reverse_sol.push(*movee - 20);
            }
            else{
                reverse_sol.push(*movee);
            }
        }
    }
    reverse_sol

}

pub fn mk_scr_inv(sol: Vec<u8>)->Vec<u8>{
    let mut reverse_sol = Vec::new();
    for movee in sol.iter().rev(){
        if *movee >0{
            if *movee < 10{
                reverse_sol.push(*movee + 20);
            }
            else if *movee > 20{
                reverse_sol.push(*movee - 20);
            }
            else{
                reverse_sol.push(*movee);
            }
        }
    }
    reverse_sol

}

pub fn int_move_to_str(scram:&Vec<u8>)->String{
    let mut stuff_str = String::new();
    for imove in scram.iter(){
        stuff_str.push_str(match imove {
            1 => "R ", 
            2 => "L ", 
            3 => "U ", 
            4 => "D ", 
            5 => "F ", 
            6 => "B ", 
            7 => "r ",
            8 => "f ",
            9 => "S ",  
            11 => "R2 ", 
            12 => "L2 ", 
            13 => "U2 ", 
            14 => "D2 ", 
            15 => "F2 ", 
            16 => "B2 ",  
            17 => "r2 ",
            18 => "f2 ",
            19 => "S2 ",
            21 => "R' ", 
            22 => "L' ", 
            23 => "U' ", 
            24 => "D' ", 
            25 => "F' ", 
            26 => "B' ", 
            27 => "r' ",
            28 => "f' ",
            29 => "S' ",  
            0 => "",
                _ => unreachable!()
            })
    }
    stuff_str
}

pub fn get_triggers()->Vec<Vec<u8>>{
    let filename = "triggers.txt";
    let r = read_to_string(filename).expect("Ensure you have a file called triggers.txt");
    let triggers = {
        let v = r.lines()
            .map(|s| {
                let trigger = convert_scramble(s.to_string());
                trigger
            })
        .collect::<Vec<Vec<u8>>>();
        v
    };
    triggers
}

pub fn read_case()->Vec<u8>{
    let filename = "case.txt";
    let r = read_to_string(filename).expect("Ensure you have a file called case.txt");
    let scramble = {
        let v = r.lines()
            .map(|s| {
                let scramble = mk_scr_inv(convert_scramble(s.to_string()));
                scramble
            })
        .collect::<Vec<Vec<u8>>>();
        v.first().expect("Must have at least one line in file").to_vec()
    };
    scramble
}

pub fn get_prune(triggers:&Vec<Vec<u8>>)->HashMap<(u64, u64), [u8; 15]>{
    let mut c: u64 = 247132686368;
    let mut e:u64 = 407901468851537952; 
    let mut q:VecDeque<(u64, u64, [u8;15],u8)> = VecDeque::new();
    let mut overview = HashMap::new();
    for movee in [0,3,3,3]{
        (c, e) = perform_move(movee, c, e);
        overview.insert((c,e),[0;15]);
        q.push_back((c,e,[0;15],0));
    }
    let mut continuee = true;
    let mut prevdepth = 0;
    while !q.is_empty() && continuee{
        let (nc, ne, nlist,depth) = q.pop_front().expect("while loop should never trigger an error");
        if depth > prevdepth{
            println!("{} ",depth);
            prevdepth = depth;
        }
        if depth <= 12{
            for trigger in triggers{
                if depth > 0{
                    if trigger[0].abs_diff(nlist[(depth-1) as usize]) == 20u8{
                        continue;
                    }
                }
                let mut nnlist = nlist.to_owned();
                let mut nnc = nc;
                let mut nne = ne;
                let mut ddepth = depth;
                for movee in trigger{
                    nnlist[ddepth as usize] = *movee;
                    (nnc, nne) = perform_move(*movee, nnc, nne);
                    ddepth = ddepth +1;
                }
                if !overview.contains_key(&(nnc,nne)){
                    overview.insert((nnc,nne),nnlist.clone());
                    q.push_back((nnc,nne,nnlist.clone(),ddepth));
                }
            }
        }
        else{
            continuee = false;
        }
        
    }
    overview

}

// Vec<String>
pub fn solve(triggers:&Vec<Vec<u8>>,prune:&HashMap<(u64, u64), [u8; 15]>)->Vec<Vec<u8>>{
    // let mut solutions:Vec<String> = Vec::new();
    let mut solutions:Vec<Vec<u8>> = Vec::new();
    // let seen:HashSet<(u64,u64)> = HashSet::new();
    
    let mut c: u64 = 247132686368;
    let mut e:u64 = 407901468851537952; 
    let scramble = read_case();
    for movee in scramble{ // Fix
        (c,e) =  perform_move(movee, c, e);
    }
    let mut q:VecDeque<(u64, u64, [u8;15],u8)> = VecDeque::new();
    for movee in [0,3,3,3]{
        (c, e) = perform_move(movee, c, e);
        q.push_back((c,e,[0;15],0));
    }
    let mut continuee = true;
    let mut prevdepth = 0;

    while !q.is_empty() && continuee{
        let (nc, ne, nlist,depth) = q.pop_front().expect("while loop should never trigger an error");
        if depth > prevdepth{
            println!("{} ",depth);
            prevdepth = depth;
        }
        if depth <= 12{
            for trigger in triggers{
                if depth > 0{
                    if trigger[0].abs_diff(nlist[(depth-1) as usize]) == 20u8{
                        continue;
                    }
                }
                let mut nnlist = nlist.to_owned();
                let mut nnc = nc;
                let mut nne = ne;
                let mut ddepth = depth;
                for movee in trigger{
                    nnlist[ddepth as usize] = *movee;
                    (nnc, nne) = perform_move(*movee, nnc, nne);
                    ddepth = ddepth +1;
                }
                if prune.contains_key(&(nnc,nne)){
                    let solution = prune.get(&(nnc,nne)).expect("Already checked that this value is in the hashmap");
                    let mut nvec = nnlist.to_vec();
                    nvec.extend(&mk_inv(solution));
                    // solutions.push(int_move_to_str(&nvec));
                    solutions.push(nvec);
                }
                else {
                    q.push_back((nnc,nne,nnlist.clone(),ddepth));
                }
            }
        
        }
        else{
            continuee = false;
        }
    }
    return solutions;
}

pub fn check_valid_alg(alg: &Vec<u8>)->bool{
    let iter = alg.into_iter();
    let mut prev = 0;
    let mut c = 0;
    // println!("{:?} ",iter);
    for movee in iter{
        // println!("{:?} ",movee);
        if *movee > 0{
            if *movee%10 == prev%10{
                c = c+1;
                // println!("hit");
            }
            else{
                prev = *movee;
                c = 0;
            }
            if c == 3{
                return false;
            }
        }
    }
    return true;
}

pub fn cancel_moves(alg: &Vec<u8>)->String{

    let mut new_alg = Vec::new();
    let mut start = 0;
    if alg[0]%10 == 3{
        start = 1;
    }
    let mut skip = false;
    let nalg = alg.iter().map(|x| *x).filter(|x| *x != 0 ).collect::<Vec<u8>>();
    // let mut prev = 0;
    // let mut curr = 0;
    for i in start..nalg.len()-1{
        let prev = nalg[i];
        let curr = nalg[i+1];
        if !skip{
            if ( curr == prev) && prev > 0{
                if !(prev < 20 && prev > 10){ // Not already double move
                    new_alg.push(prev%10 + 10);
                }
                skip = true;
            }
            else if (curr%10 == prev%10) && prev > 0 {
                if prev < 20 && prev > 10{ // If this is a double move
                    if curr > 20{
                        new_alg.push(curr%10);
                    }
                    else{
                        new_alg.push(curr%10 + 20);
                    }
                }
                else{
                    if curr < 20 && curr > 10{ // If this is a double move
                        if prev > 20{
                            new_alg.push(prev%10);
                        }
                        else{
                            new_alg.push(prev%10 + 20);
                        }
                    }
                }
                skip = true;
            }
            else{
                new_alg.push(prev);
            }
        }
        else{
            skip = false;
        }
        // prev = curr;
    }
    if !skip{
        new_alg.push(*nalg.last().expect("Vec isnt empty, gets last element"));
    }
    int_move_to_str(&new_alg)
    // int_move_to_str(alg)

}