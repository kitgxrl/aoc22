mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args[1].parse::<u8>().unwrap() {
        1 => {
            let p1 = day1::solve_p1();
            let p2 = day1::solve_p2();

            println!("Part 1: {}\nPart 2: {}", p1, p2)
        }
        2 => {
            let p1 = day2::solve_p1();
            let p2 = day2::solve_p2();

            println!("Part 1: {}\nPart 2: {}", p1, p2)
        }
        3 => {
            let p1 = day3::solve_p1();
            let p2 = day3::solve_p2();

            println!("Part 1: {}\nPart 2: {:?}", p1, p2)
        }
        _ => panic!("wtf"),
    }
}
