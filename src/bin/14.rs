use std::collections::HashMap;
use std::io::{prelude::*, BufReader, Result};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Material {
    amount: i64,
    material: String
}

fn main() {

//     let INPUT = "10 ORE => 10 A
// 1 ORE => 1 B
// 7 A, 1 B => 1 C
// 7 A, 1 C => 1 D
// 7 A, 1 D => 1 E
// 7 A, 1 E => 1 FUEL"
// .to_owned();

//     let INPUT = "157 ORE => 5 NZVS
// 165 ORE => 6 DCFZ
// 44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
// 12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
// 179 ORE => 7 PSHF
// 177 ORE => 5 HKGWZ
// 7 DCFZ, 7 PSHF => 2 XJWVT
// 165 ORE => 2 GPVTF
// 3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
// .to_owned();

//     let INPUT = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
// 17 NVRVD, 3 JNWZP => 8 VPVL
// 53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
// 22 VJHF, 37 MNCFX => 5 FWMGM
// 139 ORE => 4 NVRVD
// 144 ORE => 7 JNWZP
// 5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
// 5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
// 145 ORE => 6 MNCFX
// 1 NVRVD => 8 CXFTF
// 1 VJHF, 6 MNCFX => 4 RFSQX
// 176 ORE => 6 VJHF"
// .to_owned();

//     let INPUT = "171 ORE => 8 CNZTR
// 7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
// 114 ORE => 4 BHXH
// 14 VRPVC => 6 BMBT
// 6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
// 6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
// 15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
// 13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
// 5 BMBT => 4 WPTQ
// 189 ORE => 9 KTJDG
// 1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
// 12 VRPVC, 27 CNZTR => 2 XDBXC
// 15 KTJDG, 12 BHXH => 5 XCVML
// 3 BHXH, 2 VRPVC => 7 MZWV
// 121 ORE => 7 VRPVC
// 7 XCVML => 6 RJRHP
// 5 BHXH, 4 VRPVC => 5 LTCX"
// .to_owned();

    let INPUT = "13 RXWQR => 4 JXCXB
7 FDGDX => 7 XRWJ
3 JBVN, 25 JFRXJ => 3 TPDSB
13 HZDWS, 11 RZNJR => 3 SVFT
5 FDGDX, 4 RZNJR, 41 ZGXGP => 8 LBVM
1 LJDRB => 9 RXWQR
2 RDPWQ => 8 JBVN
2 CZCB => 8 CXHK
4 JXCXB, 1 FPQRV => 5 TCBSQ
6 FDGDX => 8 TWGNB
1 RJBTL => 5 VRVDQ
2 XRWJ, 3 HZDWS, 12 LBVM => 6 KSJD
15 HPXST, 1 KMKR, 7 SLTX, 1 PRWD, 14 RCLB, 31 TPDSB, 3 GWXJP, 3 TPQZ => 8 XRLZR
1 RBLT, 2 RTFKN, 1 CZCB => 8 DNRP
131 ORE => 8 TFGJ
2 JFRXJ, 1 VRVDQ, 26 TWGNB => 5 CFPZ
2 SMPW, 1 TWGNB => 8 RZNJR
20 HRZP => 6 RDPWQ
1 RCLB, 4 GJNK, 4 QGJL => 4 HZDWS
7 CXHK, 2 XTMRV, 6 WSNPZ, 12 LQXCP, 19 PMWJ, 17 GJNK, 26 XRLZR, 36 LWFQ => 1 FUEL
131 ORE => 8 KMKR
1 LJDRB, 12 TFGJ, 10 RXWQR => 7 RPKZ
10 RVXT, 1 RDPWQ => 8 JFRXJ
1 QXBTX => 9 TPQZ
1 ZGXGP => 5 FZGF
1 RTFKN, 1 DNRP => 2 FDGDX
19 CZCB, 1 RBLT => 4 SMPW
2 DNRP, 1 SMPW => 9 RWSH
1 ZGXGP, 5 TCBSQ, 22 SMPW => 5 GWXJP
1 HBSKF => 3 LQXCP
1 ZGXGP, 2 KSJD, 9 CFPZ => 7 CLGXQ
186 ORE => 8 LJDRB
1 TPQZ, 2 HBSKF => 1 QGJL
8 FZGF, 6 FDGDX => 3 PMWJ
9 KMKR => 1 CZCB
21 TFGJ, 3 RVXT => 5 HRZP
39 FDGDX, 24 TPDSB => 2 RCLB
4 HRZP => 2 GJNK
6 RZNJR => 2 HBSKF
101 ORE => 8 RVXT
1 RCLB => 8 QXBTX
1 RJBTL => 7 RBLT
2 CFPZ, 2 JXCXB, 4 TPQZ => 1 LWFQ
1 QGJL, 24 GJNK, 6 TWGNB, 1 SLTX, 18 JFRXJ, 6 MSNM, 6 FDGDX, 2 JXCXB => 5 WSNPZ
4 RZNJR => 6 FPQRV
12 LJDRB, 10 JFRXJ, 1 ZGXGP => 5 TXZVH
13 KSJD, 11 FXGW => 9 PRWD
11 SVFT, 2 HZDWS, 1 CLGXQ, 1 LQXCP, 6 JXCXB, 11 PRWD => 5 XTMRV
27 TWGNB, 7 FPQRV => 2 SLTX
2 HRZP, 6 RXWQR => 9 RJBTL
2 CXHK, 1 RPKZ => 1 RTFKN
7 RWSH, 12 JBVN, 6 FXGW => 2 ZGXGP
1 TXZVH, 4 FPQRV => 8 MSNM
16 TPDSB, 1 FXGW => 5 HPXST
1 VRVDQ => 2 FXGW"
.to_owned();

    let recipes = parse_input(INPUT);
    let mut queue: Vec<Material> = vec!(
        Material{ amount: 1, material: String::from("FUEL") }
    );
    let mut stock: HashMap<String, i64> = HashMap::new();
    let mut ore_req = 0;

    while queue.len() != 0 {

        let mut req = queue.pop().unwrap();

        if req.material == "ORE" {
            ore_req += req.amount;
            continue;
        }

        if req.amount == 0 {
            continue;
        } else if req.amount < 0 {
            stock.insert(req.material, -req.amount);
            continue;
        }

        match stock.get(&req.material) {
            Some(remainder) => {
                req.amount -= remainder;
                stock.remove(&req.material);
                if req.amount < 0 {
                    stock.insert(req.material, -req.amount);
                    continue;
                } else if req.amount > 0 {
                    queue.push(req);
                }
            },
            None => {
                let recipe = recipes.get(&req.material)
                    .expect("Recipe not found");
                let mut mult = req.amount / recipe.0;
                if req.amount % recipe.0 != 0 {
                    mult += 1;
                    stock.insert(req.material, (recipe.0 * mult) - req.amount);
                }
                let mut reqs = recipe.1.clone();
                for item in &mut reqs {
                    item.amount *= mult;
                }
                queue.append(&mut reqs);
            }
        }
    }

    println!("Ore for single FUEL: {}", ore_req);

    stock = HashMap::new();
    // TODO: Implement actual binary search or something
    queue = vec!(Material{amount: 1376631, material: "FUEL".to_owned()});
    ore_req = 0;
    let ore_stock: i64 = 1000000000000;

    let mut fuel_count = 0;
    while queue.len() != 0 {
        let mut req = queue.pop().unwrap();

        if req.material == "ORE" {
            ore_req += req.amount;
            continue;
        }

        if req.amount == 0 {
            continue;
        } else if req.amount < 0 {
            stock.insert(req.material, -req.amount);
            continue;
        }

        match stock.get(&req.material) {
            Some(remainder) => {
                req.amount -= remainder;
                stock.remove(&req.material);
                if req.amount < 0 {
                    stock.insert(req.material, -req.amount);
                    continue;
                } else if req.amount > 0 {
                    queue.push(req);
                }
            },
            None => {
                let recipe = recipes.get(&req.material)
                    .expect("Recipe not found");
                let mut mult = req.amount / recipe.0;
                if req.amount % recipe.0 != 0 {
                    mult += 1;
                    stock.insert(req.material, (recipe.0 * mult) - req.amount);
                }
                let mut reqs = recipe.1.clone();
                for item in &mut reqs {
                    item.amount *= mult;
                }
                queue.append(&mut reqs);
            }
        }
    }

    println!("Total Ore after fuel: {}", ore_stock - ore_req);
}

fn parse_input(input: String) -> HashMap<String, (i64, Vec<Material>)> {
    let mut recipes: HashMap<String, (i64, Vec<Material>)> = HashMap::new();

    for line in input.lines() {
        let mut line_split = line.split("=>");

        let req_string = line_split.next()
            .expect("Requirements expected");
        let mut reqs: Vec<Material> = vec!();
        for req in req_string.split(",") {
            reqs.push(parse_material(req.to_owned()));
        }

        let res_string = line_split.next()
            .expect("Result expected");
        let res: Material = parse_material(res_string.to_owned());

        recipes.insert(res.material, (res.amount, reqs));
    }
    recipes
}

fn parse_material(input: String) -> Material {
    let mut input_str = input.clone();
    input_str = input_str.trim().to_owned();
    let mut input_split = input_str.split(" ");
    let amount: i64 = input_split.next().unwrap().parse().unwrap();
    let material: String = input_split.next().unwrap().to_owned();
    Material{ amount, material }
}
