pub fn part_one(input: &str) -> Option<u32> {
    let split = input.split("\n");
    let mut max_calories: u32 = u32::MIN;
    let mut current_calories: u32 = u32::MIN;

    for number in split {
        match number.parse::<u32>() {
            Ok(number) => {
                current_calories += number;
            }
            Err(_) => {
                if current_calories > max_calories {
                    max_calories = current_calories;
                }
                current_calories = u32::MIN;
            }
        }
    }
    Some(max_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_snacks = input.split("\n\n");
    let mut elf_total_calories: Vec<u32> = Vec::new();
    for snacks in elf_snacks {
        let calories = snacks.split("\n");
        let mut total = 0;
        for calorie in calories {
            match calorie.parse::<u32>() {
                Ok(calorie) => {
                    total += calorie;
                }
                Err(_) => {}
            }
        }
        elf_total_calories.push(total);
    }
    elf_total_calories.sort_by(|a, b| b.cmp(a));
    Some(elf_total_calories[0..3].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
