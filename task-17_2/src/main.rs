// 17_1 has trivial solution done by hand

use std::ops::Range;

struct Target {
    xs: Range<i32>,
    ys: Range<i32>,
}

impl Target {
    fn new(xs: Range<i32>, ys: Range<i32>) -> Target {
        Target { xs, ys }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        self.xs.contains(&x) && self.ys.contains(&y)
    }
}

// For specified initial speed (vx, vy) check if probe would hit target.
fn would_hit(mut vx: i32, mut vy: i32, target: &Target) -> bool {
    assert!(0 < vx);
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while x < target.xs.end && target.ys.start <= y {
        if target.contains(x, y) {
            break;
        }
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;
    }
    target.contains(x, y)
}

fn solve(target: &Target) -> usize {
    let mut count = 0_usize;
    for vx in 1..target.xs.end {
        for vy in target.ys.start..-target.ys.start + 1 {
            if would_hit(vx, vy, target) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
}

#[test]
fn test_small() {
    let target = Target::new(20..31, -10..-4);
    assert_eq!(112, solve(&target));
}

#[test]
fn test_big() {
    let target = Target::new(102..158, -146..-89);
    assert_eq!(5247, solve(&target));
}