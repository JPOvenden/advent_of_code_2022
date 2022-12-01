fn solvepart1() {
    let calories: Vec<u32> = include_str!("../inputs/day01/input.txt")
            .split("\r\n\r\n")
            .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
            .max()
            .unwrap();
    print!("{}", calories)
}