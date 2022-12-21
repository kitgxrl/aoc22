use std::fs::read_to_string;

pub enum Turn {
    Rock,
    Paper,
    Scissors,
}

pub fn get_input() -> Vec<(Turn, Turn)> {
    let raw = read_to_string("./inputs/day2").unwrap();

    let lines: Vec<Vec<&str>> = raw
        .lines()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .collect();

    lines
        .into_iter()
        .map(|x| {
            let elf = match x.first().unwrap() {
                &"A" => Turn::Rock,
                &"B" => Turn::Paper,
                &"C" => Turn::Scissors,
                _ => panic!("wtf"),
            };

            let me = match x.last().unwrap() {
                &"X" => Turn::Rock,
                &"Y" => Turn::Paper,
                &"Z" => Turn::Scissors,
                _ => panic!("wtf"),
            };

            (elf, me)
        })
        .collect()
}

pub fn solve_p1() -> u32 {
    let inp = get_input();

    let mut score = 0;

    for i in inp {
        match i.1 {
            Turn::Rock => score += 1,
            Turn::Paper => score += 2,
            Turn::Scissors => score += 3,
        }

        match i {
            (Turn::Rock, Turn::Paper) => score += 6,
            (Turn::Paper, Turn::Scissors) => score += 6,
            (Turn::Scissors, Turn::Rock) => score += 6,
            (Turn::Rock, Turn::Rock) => score += 3,
            (Turn::Paper, Turn::Paper) => score += 3,
            (Turn::Scissors, Turn::Scissors) => score += 3,
            _ => (),
        }
    }

    score
}

pub fn solve_p2() -> u32 {
    let inp = get_input();

    let mut score = 0;

    for i in inp {
        match i.1 {
            Turn::Rock => match i.0 {
                Turn::Rock => score += 3,
                Turn::Paper => score += 1,
                Turn::Scissors => score += 2,
            },

            Turn::Paper => {
                score += 3;

                match i.0 {
                    Turn::Rock => score += 1,
                    Turn::Paper => score += 2,
                    Turn::Scissors => score += 3,
                }
            }

            Turn::Scissors => {
                score += 6;

                match i.0 {
                    Turn::Rock => score += 2,
                    Turn::Paper => score += 3,
                    Turn::Scissors => score += 1,
                }
            }
        }
    }

    score
}
