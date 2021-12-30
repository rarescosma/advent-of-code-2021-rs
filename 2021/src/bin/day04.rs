use aoc_prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day04-bingo.pest"]
pub struct BingoParser;

fn parse_input(input: Vec<&str>) -> (Vec<usize>, Vec<Table>) {
    let bingo_table = input[2..].join(&'\n'.to_string());

    let draws_parse = BingoParser::parse(Rule::draws, input[0])
        .expect("failed parse")
        .next()
        .unwrap();

    let bingo_parse = BingoParser::parse(Rule::bingo, &bingo_table)
        .expect("failed parse")
        .next()
        .unwrap();

    let draws = extract_numbers(draws_parse.into_inner());

    let tables = bingo_parse
        .into_inner()
        .filter(|x| x.as_rule() == Rule::bingo_table)
        .map(|bt| Table::from_lines(extract_lines(bt.into_inner())))
        .collect();

    (draws, tables)
}

fn extract_lines<'r>(rules: impl Iterator<Item = Pair<'r, Rule>>) -> Vec<Vec<usize>> {
    rules
        .filter(|x| x.as_rule() == Rule::bingo_line)
        .map(|x| extract_numbers(x.into_inner()))
        .collect()
}

fn extract_numbers<'r>(rules: impl Iterator<Item = Pair<'r, Rule>>) -> Vec<usize> {
    rules
        .filter(|x| x.as_rule() == Rule::number)
        .flat_map(|x| x.as_str().parse::<usize>())
        .collect()
}

#[derive(Debug)]
struct Table {
    lines: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
}

impl Table {
    fn from_lines(lines: Vec<Vec<usize>>) -> Self {
        let height = lines.len();
        assert_ne!(height, 0);
        let width = lines[0].len();

        let mut cols = vec![Vec::<usize>::with_capacity(height); width];
        for line in lines.iter() {
            for (col_no, col) in cols.iter_mut().enumerate() {
                col.push(line[col_no]);
            }
        }
        Self { lines, cols }
    }

    fn draw(&mut self, number: &usize) {
        for line in &mut self.lines {
            line.retain(|x| x != number);
        }
        for col in &mut self.cols {
            col.retain(|x| x != number);
        }
    }

    fn is_winner(&self) -> bool {
        let winner_lines = self.lines.iter().find(|&x| x.is_empty());
        let winner_cols = self.cols.iter().find(|&x| x.is_empty());
        winner_lines.is_some() || winner_cols.is_some()
    }

    fn score(&self) -> usize {
        self.lines.iter().flatten().sum::<usize>()
    }
}

aoc_2021::main! {
    let input = include_str!("../../inputs/day04.txt").lines().collect::<Vec<_>>();
    let (draws, mut tables) = parse_input(input);

    let mut num_winners = 0;
    let num_tables = tables.len();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut got_winner = false;

    'outer: for draw in &draws {
        for table in &mut tables {
            let has_won = table.is_winner();
            if !has_won {
                table.draw(draw);
                if table.is_winner() {
                    if !got_winner {
                        got_winner = true;
                        p1 = table.score() * draw;
                    }
                    num_winners += 1;
                    if num_winners == num_tables {
                        p2 = table.score() * draw;
                        break 'outer;
                    }
                }
            }
        }
    }

    (p1, p2)
}
