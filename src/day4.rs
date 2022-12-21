use std::fs::read_to_string;

pub fn get_input() -> Vec<((u8, u8), (u8, u8))> {
    let raw = read_to_string("./inputs/day4").expect("wtf");
    let lines = raw.lines();

    let sc: Vec<((u8, u8), (u8, u8))> = lines
        .map(|x| {
            let spl = x.split(&[',', '-']);
            let par = spl.map(|y| y.parse::<u8>().unwrap()).collect::<Vec<u8>>();

            ((par[0], par[1]), (par[2], par[3]))
        })
        .collect();

    sc
}

pub fn solve_p1() -> usize {
    let inp = get_input();
    let mut c = 0;

    for i in inp.iter() {
        let task1 = ((i.0).0..=(i.0).1);
        let task2 = ((i.1).0..=(i.1).1);

        if task1.clone().all(|x| task2.contains(&x)) || task2.clone().all(|x| task1.contains(&x)) {
            c += 1
        }
    }

    c
}

pub fn solve_p2() -> usize {
    let inp = get_input();
    let mut c = 0;

    for i in inp.iter() {
        let task1 = (i.0).0..=(i.0).1;
        let task2 = (i.1).0..=(i.1).1;

        if !task1
            .clone()
            .filter(|x| task2.contains(&x))
            .collect::<Vec<u8>>()
            .is_empty()
            || !task2
                .clone()
                .filter(|x| task1.contains(&x))
                .collect::<Vec<u8>>()
                .is_empty()
        {
            c += 1
        }
    }

    c
}
