use aoc_util::stdin_to_line_vec;
use std::cmp;

fn main() {
    let input = stdin_to_line_vec();
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
        }
    }
}

struct Claim {
    p1: Point,
    p2: Point,
}

impl Claim {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Claim {
        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);
        Claim::from_points(p1, p2)
    }

    fn from_points(p1: Point, p2: Point) -> Claim {
        Claim {
            p1,
            p2,
        }
    }

    fn top_right(&self) -> Point {
        Point::new(self.p2.x, self.p1.y)
    }

    fn bot_left(&self) -> Point {
        Point::new(self.p1.x, self.p2.y)
    }

    fn intersection(&self, other: &Claim) -> Option<Claim> {
        let x1 = cmp::max(self.p1.x, other.p1.x);
        let y1 = cmp::max(self.p1.y, other.p1.y);
        let x2 = cmp::min(self.p2.x, other.p2.x);
        let y2 = cmp::min(self.p2.y, other.p2.y);
        if x1 <= x2 && y1 <= y2 {
            Some(Claim::new(x1, y1, x2, y2))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_p1_in_p2() {
        let c1 = Claim::new(3, 4, 5, 6);
        let c2 = Claim::new(2, 3, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1.x, 3);
        assert_eq!(i.p1.y, 4);
        assert_eq!(i.p2.x, 5);
        assert_eq!(i.p2.y, 6);
    }

    #[test]
    fn intersection_p2_in_p1() {
        let c1 = Claim::new(2, 3, 6, 7);
        let c2 = Claim::new(3, 4, 5, 6);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1.x, 3);
        assert_eq!(i.p1.y, 4);
        assert_eq!(i.p2.x, 5);
        assert_eq!(i.p2.y, 6);
    }

    #[test]
    fn intersection_bottom_edge() {
        let c1 = Claim::new(2, 3, 6, 7);
        let c2 = Claim::new(4, 7, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1.x, 4);
        assert_eq!(i.p1.y, 7);
        assert_eq!(i.p2.x, 6);
        assert_eq!(i.p2.y, 7);
    }

    #[test]
    fn intersection_right_edge() {
        let c1 = Claim::new(2, 3, 6, 7);
        let c2 = Claim::new(6, 2, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1.x, 6);
        assert_eq!(i.p1.y, 3);
        assert_eq!(i.p2.x, 6);
        assert_eq!(i.p2.y, 7);
    }

    #[test]
    fn intersection_point() {
        let c1 = Claim::new(2, 3, 6, 7);
        let c2 = Claim::new(6, 7, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1.x, 6);
        assert_eq!(i.p1.y, 7);
        assert_eq!(i.p2.x, 6);
        assert_eq!(i.p2.y, 7);
    }
}
