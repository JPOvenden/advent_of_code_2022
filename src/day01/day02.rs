fn solverpart2() {
    let mut calories: Vec<u32> = include_str!("../inputs/day01/input.txt")
        .split("\r\n\r\n")
        .map(|x| x.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();
    calories.sort_unstable();
    print!("{}", calories.into_iter().rev().take(3).sum::<u32>());
}