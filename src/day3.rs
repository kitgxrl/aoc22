use std::fs::read_to_string;

fn get_input() -> Vec<String> {
    let raw = read_to_string("./inputs/day3").unwrap();
    let lines = raw.lines();

    lines.map(|x| x.to_string()).collect()
}

pub fn solve_p1() -> u32 {
    let inps = get_input();
    let inp = inps.into_iter().map(|x| x.split_at(x.len() / 2));
    let mut t: u32 = 0;

    for i in inp {
        let mut dupe = '\n';

        'outer: for c1 in i.0.chars() {
            for c2 in i.1.chars() {
                if c1 == c2 {
                    dupe = c1;
                    break 'outer;
                } else {
                    ()
                }
            }
        }

        let n = match dupe {
            'a'..='z' => dupe as u8 - 'a' as u8 + 1,
            'A'..='Z' => dupe as u8 - 'A' as u8 + 27,
            _ => 0,
        };

        t += n as u32
    }

    t
}

pub fn solve_p2() -> u32 {
    let inp = get_input();
    let t: u32 = 0;

    t
}
