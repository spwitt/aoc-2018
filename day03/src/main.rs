use aoc_util::stdin_to_line_vec;
use std::cmp::{self, Ordering};

fn main() {
    let input = stdin_to_line_vec();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// This implementation will only return less if both coordinates in self
// have value <= the corresponding values in other
impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x == other.x && self.y == other.y {
            return Ordering::Equal;
        }
        if self.x <= other.x && self.y <= other.y {
            return Ordering::Less;
        }
        Ordering::Greater
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

    fn intersection(&self, claim: &Claim) -> Option<Claim> {
        let p1 = cmp::max(self.p1, claim.p1);
        let p2 = cmp::min(self.p2, claim.p2);
        if p1 <= p2 {
            Some(Claim::from_points(p1, p2))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_points() {
        let p1 = Point::new(2, 2);
        let p2 = Point::new(2, 2);
        assert!(p1 == p2);
    }

    #[test]
    fn test_not_equal_points() {
        let p1 = Point::new(2, 2);
        let p2 = Point::new(1, 2);
        assert!(p1 != p2);
    }

    #[test]
    fn test_lt_points() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(2, 2);
        assert!(p1 < p2);
    }

    #[test]
    fn test_gt_points() {
        let p1 = Point::new(1, 3);
        let p2 = Point::new(2, 2);
        assert!(p1 > p2);
    }

    #[test]
    fn intersection_p1_in_p2() {
        let c1 = Claim::new(3, 4, 5, 6);
        let c2 = Claim::new(2, 3, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1, c1.p1);
        assert_eq!(i.p2, c1.p2);
    }

    #[test]
    fn intersection_p2_in_p1() {
        let c1 = Claim::new(2, 3, 6, 7);
        let c2 = Claim::new(3, 4, 5, 6);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1, c2.p1);
        assert_eq!(i.p2, c2.p2);
    }

    #[test]
    fn intersection_edge() {
        let c1 = Claim::new(2, 3, 6, 7);
        let c2 = Claim::new(4, 7, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1, c2.p1);
        assert_eq!(i.p2, c2.p2);
    }

    #[test]
    fn intersection_point() {
        let c1 = Claim::new(2, 3, 6, 7);
        let c2 = Claim::new(6, 7, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1, c2.p1);
        assert_eq!(i.p2, c2.p2);
    }
}
