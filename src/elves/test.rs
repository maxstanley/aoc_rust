#[macro_export]
macro_rules! test {
    ($year:literal, $day:literal => $p1_example:literal, $p1:literal, $p2_example:literal, $p2:literal) => {
        #[cfg(test)]
        mod tests {
            use const_format::formatcp;
            use std::fs;

            const INPUT: &'static str = formatcp!("./src/aoc{}/input/{}.txt", $year, $day);
            const INPUT_EXAMPLE: &'static str =
                formatcp!("./src/aoc{}/input/{}_example.txt", $year, $day);

            #[test]
            fn part01_example() {
                let lines = fs::read_to_string(INPUT_EXAMPLE).unwrap();

                assert_eq!(super::part01(&lines).unwrap(), $p1_example);
            }

            #[test]
            fn part01() {
                let lines = fs::read_to_string(INPUT).unwrap();

                assert_eq!(super::part01(&lines).unwrap(), $p1);
            }

            #[test]
            fn part02_example() {
                let lines = fs::read_to_string(INPUT_EXAMPLE).unwrap();

                assert_eq!(super::part02(&lines).unwrap(), $p2_example);
            }

            #[test]
            fn part02() {
                let lines = fs::read_to_string(INPUT).unwrap();

                assert_eq!(super::part02(&lines).unwrap(), $p2);
            }
        }
    };
}
