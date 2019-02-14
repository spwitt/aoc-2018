use aoc_util::stdin_to_line_vec;
use std::cmp;
use std::collections::HashSet;

fn main() {
    let input = stdin_to_line_vec();
    let (result1, result2) = run(input);
    println!("Part 1 result is {}", result1);
    println!("Part 2 result is {}", result2);
}

fn run(input: Vec<String>) -> (usize, i32) {
    let claims: Vec<_> = input.iter().map(|i| Claim::from_claim_str(i)).collect();
    let mut overlap_points = HashSet::new();
    let mut claim_ids = claims.iter().fold(HashSet::new(), |mut h, c| {
        h.insert(c.id);
        h
    });
    for (i, c1) in claims.iter().enumerate() {
        for c2 in claims.iter().skip(i + 1) {
            if let Some(overlap) = c1.intersection(&c2) {
                overlap.enumerate_points().iter().for_each(|&p| {
                    overlap_points.insert(p);
                    claim_ids.remove(&c1.id);
                    claim_ids.remove(&c2.id);
                });
            }
        }
    }
    assert_eq!(claim_ids.len(), 1);
    (overlap_points.len(), claim_ids.into_iter().next().unwrap())
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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
    id: i32,
    p1: Point,
    p2: Point,
}

impl Claim {
    fn new(id: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> Claim {
        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);
        Claim::from_points(id, p1, p2)
    }

    fn from_points(id: i32, p1: Point, p2: Point) -> Claim {
        Claim {
            id,
            p1,
            p2,
        }
    }

    fn enumerate_points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        for x in self.p1.x..self.p2.x+1 {
            for y in self.p1.y..self.p2.y+1 {
                points.insert(Point::new(x, y));
            }
        }
        points
    }

    fn from_claim_str(claim: &str) -> Claim {
        let mut v: Vec<&str> = claim.split(|c| {
            c == ' ' ||
            c == '#' ||
            c == '@' ||
            c == ',' ||
            c == ':' ||
            c == 'x'
        }).collect();
        v.retain(|s| !s.is_empty());
        assert_eq!(v.len(), 5);
        let id: i32 = v[0].parse().unwrap();
        let x: i32 = v[1].parse().unwrap();
        let y: i32 = v[2].parse().unwrap();
        let w: i32 = v[3].parse().unwrap();
        let h: i32 = v[4].parse().unwrap();
        Claim::new(id, x, y, x + w - 1, y + h - 1)
    }

    fn intersection(&self, other: &Claim) -> Option<Claim> {
        let x1 = cmp::max(self.p1.x, other.p1.x);
        let y1 = cmp::max(self.p1.y, other.p1.y);
        let x2 = cmp::min(self.p2.x, other.p2.x);
        let y2 = cmp::min(self.p2.y, other.p2.y);
        if x1 <= x2 && y1 <= y2 {
            Some(Claim::new(1, x1, y1, x2, y2))
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
        let c1 = Claim::new(1, 3, 4, 5, 6);
        let c2 = Claim::new(1, 2, 3, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1.x, 3);
        assert_eq!(i.p1.y, 4);
        assert_eq!(i.p2.x, 5);
        assert_eq!(i.p2.y, 6);
    }

    #[test]
    fn intersection_p2_in_p1() {
        let c1 = Claim::new(1, 2, 3, 6, 7);
        let c2 = Claim::new(1, 3, 4, 5, 6);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1.x, 3);
        assert_eq!(i.p1.y, 4);
        assert_eq!(i.p2.x, 5);
        assert_eq!(i.p2.y, 6);
    }

    #[test]
    fn intersection_bottom_edge() {
        let c1 = Claim::new(1, 2, 3, 6, 7);
        let c2 = Claim::new(1, 4, 7, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1.x, 4);
        assert_eq!(i.p1.y, 7);
        assert_eq!(i.p2.x, 6);
        assert_eq!(i.p2.y, 7);
    }

    #[test]
    fn intersection_right_edge() {
        let c1 = Claim::new(1, 2, 3, 6, 7);
        let c2 = Claim::new(1, 6, 2, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1.x, 6);
        assert_eq!(i.p1.y, 3);
        assert_eq!(i.p2.x, 6);
        assert_eq!(i.p2.y, 7);
    }

    #[test]
    fn intersection_point() {
        let c1 = Claim::new(1, 2, 3, 6, 7);
        let c2 = Claim::new(1, 6, 7, 6, 7);
        let i = c1.intersection(&c2).unwrap();
        assert_eq!(i.p1.x, 6);
        assert_eq!(i.p1.y, 7);
        assert_eq!(i.p2.x, 6);
        assert_eq!(i.p2.y, 7);
    }

    #[test]
    fn claim_from_str() {
        let c = Claim::from_claim_str("#1 @ 1,3: 4x4");
        assert_eq!(c.p1.x, 1);
        assert_eq!(c.p1.y, 3);
        assert_eq!(c.p2.x, 4);
        assert_eq!(c.p2.y, 6);
    }

    #[test]
    fn claim_enumerate_points() {
        let c = Claim::new(1, 2, 3, 5, 6);
        let points = c.enumerate_points();
        assert_eq!(points.len(), 16);
    }
}
