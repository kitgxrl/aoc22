pub(crate) use std::fs;

fn get_input() -> Vec<Vec<u32>> {
    let raw = fs::read_to_string("./inputs/day1").expect("Unexpected");
    let rs: Vec<&str> = raw.split("\n\n").collect();
    let rs2: Vec<Vec<u32>> = rs
        .iter()
        .map(|x| x.lines().map(|y| y.parse::<u32>().unwrap()).collect())
        .collect();

    rs2
}

pub fn solve_p1() -> u32 {
    let inp = get_input();

    let sums = inp.iter().map(|e| e.iter().sum()).collect::<Vec<u32>>();
    let max = sums.into_iter().max().unwrap();

    max
}

pub fn solve_p2() -> u32 {
    let inp = get_input();

    let mut sums = inp.iter().map(|e| e.iter().sum()).collect::<Vec<u32>>();
    sums.sort_by(|x, y| y.cmp(x));

    sums[0] + sums[1] + sums[2]
}
