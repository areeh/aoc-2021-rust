pub mod day1 {
    use std::path::Path;
    use std::fs;

    fn read() -> Vec<i32> {
        let path = Path::new("./src/day1/input.txt");

        fs::read_to_string(path)
            .expect("Could not read file")
            .lines()
            .map(|x| x.parse().expect("Could not parse line of file as i32"))
            .collect()
    }

    pub fn main1() -> std::io::Result<usize> {
        let d = read();
        let num_is_greater = d.windows(2).filter(|w| w[1] > w[0]).count();

        Ok(num_is_greater)
    }

    pub fn main2() -> std::io::Result<usize> {
        let d = read();
        let num_is_greater = d
            .windows(3)
            .collect::<Vec<_>>()
            .windows(2)
            .filter(|w| w[1].iter().sum::<i32>() > w[0].iter().sum())
            .count();

        Ok(num_is_greater)
    }
}
