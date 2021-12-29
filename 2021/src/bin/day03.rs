use std::cmp::Ordering;

trait Commonality {
    fn oxy(&self) -> char;
    fn scrubber(&self) -> char;
}

impl Commonality for Ordering {
    fn oxy(&self) -> char {
        match self {
            Ordering::Greater | Ordering::Equal => '1',
            Ordering::Less => '0',
        }
    }

    fn scrubber(&self) -> char {
        match self {
            Ordering::Greater | Ordering::Equal => '0',
            Ordering::Less => '1',
        }
    }
}

fn most_common(xs: &[&str], pos: &usize) -> Ordering {
    let mut one_count = 0;
    let mut zero_count = 0;
    xs.iter().for_each(|&x| {
        let digit = x.chars().nth(*pos).unwrap();
        match digit.eq(&'1') {
            true => one_count += 1,
            false => zero_count += 1,
        }
    });
    one_count.cmp(&zero_count)
}

fn bin_to_int(x: &str) -> usize {
    usize::from_str_radix(x, 2).unwrap()
}

aoc_2021::main! {
    let lines = include_str!("../../inputs/day03.txt").lines().collect::<Vec<_>>();
    let mut oxy_numbers = lines.clone();
    let mut scrubber_numbers = lines.clone();

    let num_len = lines[0].len();
    let mut sums = vec![0; num_len];
    let half = (lines.len() / 2) as u16;

    lines.iter().for_each(|&x| {
        for (i, c) in x.chars().enumerate() {
            let add = c as u8 - b'0';
            sums[i] += add as u16;
        }
    });

    let gamma_str: String = sums
        .iter()
        .map(|&x| if x > half { "1" } else { "0" })
        .collect();

    let epsilon_str: String = gamma_str
        .chars()
        .map(|x| if x == '0' { "1" } else { "0" })
        .collect();

    let gamma = bin_to_int(&gamma_str);
    let epsilon = bin_to_int(&epsilon_str);
    let p1 = gamma * epsilon;

    let mut pos = 0;
    while oxy_numbers.len() > 1 {
        let haystack = most_common(&oxy_numbers, &pos).oxy();
        let new_numbers = oxy_numbers
            .into_iter()
            .filter(|x| x.chars().nth(pos).unwrap() == haystack)
            .collect();

        oxy_numbers = new_numbers;
        pos += 1;
    }
    dbg!(&oxy_numbers);

    let mut pos = 0;
    while scrubber_numbers.len() > 1 {
        let haystack = most_common(&scrubber_numbers, &pos).scrubber();
        let new_numbers = scrubber_numbers
            .into_iter()
            .filter(|x| x.chars().nth(pos).unwrap() == haystack)
            .collect();

        scrubber_numbers = new_numbers;
        pos += 1;
    }
    dbg!(&scrubber_numbers);

    let p2 = bin_to_int(oxy_numbers[0]) * bin_to_int(scrubber_numbers[0]);

    (p1, p2)
}
