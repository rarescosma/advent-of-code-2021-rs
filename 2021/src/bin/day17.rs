use aoc_prelude::prelude::*;

lazy_static! {
    static ref X_MIN: isize = 137;
    static ref X_MAX: isize = 171;
    static ref Y_MIN: isize = -98;
    static ref Y_MAX: isize = -73;
    static ref XRANGE: HashSet<isize> = (*X_MIN..=*X_MAX).collect();
    static ref YRANGE: HashSet<isize> = (*Y_MIN..=*Y_MAX).collect();
}

#[derive(Default, Debug, Copy, Clone)]
struct State {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl State {
    fn new(vx: isize, vy: isize) -> Self {
        Self {
            vx,
            vy,
            ..State::default()
        }
    }

    fn step(self) -> Self {
        let x = self.x + self.vx;
        let y = self.y + self.vy;

        let vx = match self.vx {
            v if v < 0 => v + 1,
            v if v > 0 => v - 1,
            v => v,
        };
        let vy = self.vy - 1;
        Self { x, y, vx, vy }
    }

    fn overshot(&self) -> bool {
        self.x > *X_MAX || self.y < *Y_MIN
    }

    fn hit(&self) -> bool {
        XRANGE.contains(&self.x) && YRANGE.contains(&self.y)
    }
}

fn solve_triangle(sum: isize) -> isize {
    ((sum * 2 + 1) as f32).sqrt().floor() as isize
}

aoc_2021::main! {
    let p1 = *Y_MIN * (*Y_MIN + 1) / 2;

    let vx_min = solve_triangle(*X_MIN);
    let vx_max = *X_MAX;

    let vy_min = *Y_MIN;
    let vy_max = Y_MIN.abs();

    let mut hits = 0;
    for vx in vx_min..=vx_max {
        for vy in vy_min..=vy_max {
            let mut s = State::new(vx, vy);
            while !s.overshot() {
                s = s.step();
                if s.hit() {
                    hits += 1;
                    break;
                }
            }
        }
    }

    (p1, hits)
}
