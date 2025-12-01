advent_of_code::solution!(1);

static START_POSITION: i32 = 50;
static NUMBER_OF_CLICKS: i32 = 100; // 0 to 99

pub fn part_one(input: &str) -> Option<u64> {
    let mut number_of_times_zero = 0;

    let mut current_position = START_POSITION;
    for line in input.lines() {
        let (direction, steps) = parse_direction_steps(line);

        current_position += steps * direction;
        current_position = current_position.rem_euclid(NUMBER_OF_CLICKS);

        if current_position == 0 {
            number_of_times_zero += 1;
        }
    }
    Some(number_of_times_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut number_of_times_zero = 0;

    let mut current_position = START_POSITION;
    for line in input.lines() {
        let (direction, steps) = parse_direction_steps(line);
        let old_position = current_position;
        current_position += steps * direction;

        let times_clicked_zero = if current_position > 0 {
            current_position / NUMBER_OF_CLICKS
        } else {
            // when negative we need to account for starting at 0, else we end up with off by one
            (current_position * -1 / NUMBER_OF_CLICKS) + if old_position == 0 {0} else {1}
        };

        number_of_times_zero += times_clicked_zero;

        // new position inside the 0-99 range
        current_position = current_position.rem_euclid(NUMBER_OF_CLICKS);
    }

    Some(number_of_times_zero as u64)
}

fn parse_direction_steps(line: &str) -> (i32, i32) {
    let (direction_letter, steps_letter) = line.split_at(1);

    let steps = steps_letter.parse::<i32>().unwrap();
    let direction = if direction_letter.eq("R") { 1 } else { -1 };

    return (steps, direction);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
