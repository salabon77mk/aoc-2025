pub mod gimme_input;

fn main() {
    let instructions = parse_instructions(gimme_input::INPUT);
    //let counts = part_1(&instructions);
    let counts = part_2(&instructions);
    println!("{counts}");
}

fn part_1(instructions: &[Instruction]) -> u32 {
    let final_count = instructions
        .iter()
        .fold((0, 50), |(zeroes, position), instr| {
            let pos = calculate_position(instr, position);
            if pos == 0 {
                return (zeroes + 1, pos);
            }
            (zeroes, pos)
        });
    final_count.0
}

fn part_2(instructions: &[Instruction]) -> i32 {
    let final_count = instructions.iter().fold((0, 50), |(zeroes, pos), instr| {
        let updated_pos = calculate_position(instr, pos);
        let updated_zeroes = calculate_zero_clicks(instr.steps, pos, &instr.direction, 0);

        (zeroes + updated_zeroes, updated_pos)
    });

    final_count.0
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let dir = &line[0..1];
            let direction = &line[1..]
                .parse::<i32>()
                .expect("Got a not u32 when parsing?");
            Instruction::new(dir, *direction)
        })
        .collect()
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(char: &str) -> Self {
        match char {
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!("What"),
        }
    }
}

struct Instruction {
    direction: Direction,
    steps: i32,
}

impl Instruction {
    fn new(char: &str, steps: i32) -> Self {
        Self {
            direction: Direction::new(char),
            steps,
        }
    }
}

fn calculate_position(instruction: &Instruction, pos: i32) -> i32 {
    match instruction.direction {
        Direction::Left => (pos - instruction.steps).rem_euclid(100),
        Direction::Right => (pos + instruction.steps) % 100,
    }
}

fn calculate_zero_clicks(steps: i32, pos: i32, direction: &Direction, zero_clicks: i32) -> i32 {
    match steps {
        0 => zero_clicks,
        _ => {
            let updated_pos = calculate_position(
                &Instruction {
                    direction: *direction,
                    steps: 1,
                },
                pos,
            );

            if updated_pos == 0 {
                calculate_zero_clicks(steps - 1, updated_pos, direction, zero_clicks + 1)
            } else {
                calculate_zero_clicks(steps - 1, updated_pos, direction, zero_clicks)
            }
        }
    }
}
