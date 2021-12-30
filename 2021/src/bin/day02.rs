use aoc_prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day02-command.pest"]
pub struct CommandParser;

#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Default, Debug)]
struct State {
    aim: usize,
    position: usize,
    depth: usize,
}

impl State {
    fn execute_simple(&mut self, command: &Command) {
        match command {
            Command::Forward(q) => {
                self.position += q;
            }
            Command::Down(q) => {
                self.depth += q;
            }
            Command::Up(q) => {
                self.depth -= q;
            }
        }
    }

    fn execute_aimed(&mut self, command: &Command) {
        match command {
            Command::Forward(q) => {
                self.position += q;
                self.depth += self.aim * q
            }
            Command::Down(q) => {
                self.aim += q;
            }
            Command::Up(q) => {
                self.aim -= q;
            }
        }
    }
}

fn parse_line(s: &str) -> Result<Command, ()> {
    let parsed = CommandParser::parse(Rule::line, s)
        .expect("failed parse")
        .next()
        .unwrap();

    let rules: Vec<_> = parsed.into_inner().collect();
    let quant = rules[1].as_str().parse::<usize>().unwrap();
    Ok(match rules[0].as_str() {
        "forward" => Command::Forward(quant),
        "up" => Command::Up(quant),
        "down" => Command::Down(quant),
        _ => unreachable!(),
    })
}

aoc_2021::main! {
    let mut p1_state = State::default();
    let mut p2_state = State::default();

    include_str!("../../inputs/day02.txt")
        .lines()
        .flat_map(parse_line).for_each(|command| {
            p1_state.execute_simple(&command);
            p2_state.execute_aimed(&command);
        });

    let p1 = p1_state.depth * p1_state.position;
    let p2 = p2_state.depth * p2_state.position;
    (p1, p2)
}
