use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn manhatten(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

// Don't include the first point. It's just the points from point a + 1 -> b
fn points_between(a: Point, b: Point) -> Vec<Point> {
    let xs = match a.x.cmp(&b.x) {
        Ordering::Less => (a.x + 1..=b.x).collect::<Vec<i32>>(),
        Ordering::Greater => (b.x..=a.x - 1).rev().collect::<Vec<i32>>(),
        Ordering::Equal => vec![a.x],
    };

    let ys = match a.y.cmp(&b.y) {
        Ordering::Less => (a.y + 1..=b.y).collect::<Vec<i32>>(),
        Ordering::Greater => (b.y..=a.y - 1).rev().collect::<Vec<i32>>(),
        Ordering::Equal => vec![a.y],
    };

    let mut points = vec![];

    for x in &xs {
        for y in &ys {
            // println!("{} {}", x, y);
            points.push(Point { x: *x, y: *y })
        }
    }

    return points;
}

trait Wirer {
    fn add_base_point(&mut self, next: Point);
    fn add_check_point(&mut self, next: Point);
    fn best_distance(&mut self) -> i32;
}

struct CloseContactWirer {
    matrix: HashSet<Point>,
    base_point: Point,
    check_point: Point,
    intersections: Vec<Point>,
}

impl CloseContactWirer {
    fn new() -> CloseContactWirer {
        return CloseContactWirer {
            matrix: HashSet::new(),
            base_point: Point { x: 0, y: 0 },
            check_point: Point { x: 0, y: 0 },
            intersections: Vec::new(),
        };
    }
}

impl Wirer for CloseContactWirer {
    fn add_base_point(&mut self, next: Point) {
        for point in points_between(self.base_point, next) {
            self.matrix.insert(point);
            println!("PURT IN {:?}", point)
        }
        self.base_point = next;
    }

    fn add_check_point(&mut self, next: Point) {
        for point in points_between(self.check_point, next) {
            if point == (Point { x: 0, y: 0 }) {
                continue;
            }
            if self.matrix.contains(&point) {
                println!("CHECKING {:?}", point);
                self.intersections.push(point)
            }
        }
        self.check_point = next;
    }

    fn best_distance(&mut self) -> i32 {
        let closest_point = self.intersections.iter().fold(
            Point {
                x: 999999,
                y: 999999,
            },
            |closest, intersection| {
                if intersection.manhatten() < closest.manhatten() {
                    println!("CLOSERSD {}. {}", intersection.x, intersection.y);
                    return *intersection;
                } else {
                    println!("CLOSER {}. {}", closest.x, closest.y);
                    return closest;
                }
            },
        );
        return closest_point.x + closest_point.y;
    }
}

// LongDistanceWirer measures how long the wire is to a contact point
struct LongDistanceWirer {
    base_state: (Point, i32),
    check_state: (Point, i32),

    base_distances: HashMap<Point, i32>,
    best_intersection: (Point, i32),
}

impl LongDistanceWirer {
    fn new() -> LongDistanceWirer {
        return LongDistanceWirer {
            base_state: (Point { x: 0, y: 0 }, 0),
            check_state: (Point { x: 0, y: 0 }, 0),

            base_distances: HashMap::new(),
            best_intersection: (Point { x: 0, y: 0 }, 999999999),
        };
    }
}

impl Wirer for LongDistanceWirer {
    fn add_base_point(&mut self, next: Point) {
        let mut distance = self.base_state.1;
        for point in points_between(self.base_state.0, next) {
            distance += 1;
            self.base_distances.insert(point, distance);
            println!("PURT IN {:?}, {}", point, distance);
        }
        self.base_state = (next, distance);
    }

    fn add_check_point(&mut self, next: Point) {
        let mut distance = self.check_state.1;
        for point in points_between(self.check_state.0, next) {
            distance += 1;

            println!("WELL {:?} {}", point, distance);

            if self.base_distances.contains_key(&point) {
                let total_distance = distance + self.base_distances[&point];
                println!("CHECKING {:?}", total_distance);

                if total_distance < self.best_intersection.1 {
                    self.best_intersection = (point, total_distance)
                }
            }
        }
        self.check_state.0 = next;
        self.check_state.1 = distance;
    }

    fn best_distance(&mut self) -> i32 {
        return self.best_intersection.1;
    }
}

const WIRE_ONE: &str = "R999,U626,R854,D200,R696,D464,R54,D246,L359,U57,R994,D813,L889,U238,L165,U970,L773,D904,L693,U512,R126,D421,R732,D441,R453,D349,R874,D931,R103,D794,R934,U326,L433,D593,L984,U376,R947,U479,R533,U418,R117,D395,L553,D647,R931,D665,L176,U591,L346,D199,L855,D324,L474,U251,R492,D567,L97,D936,L683,U192,R198,U706,L339,U66,R726,D102,R274,U351,R653,D602,L695,U921,R890,D654,R981,U351,R15,U672,R856,D319,R102,D234,R248,U169,L863,U375,L412,U75,L511,U298,L303,U448,R445,U638,L351,D312,R768,D303,L999,D409,L746,U266,L16,U415,L951,D763,L976,U342,L505,U770,L228,D396,L992,U3,R243,D794,L496,U611,R587,U772,L306,D119,L470,D490,L336,U518,L734,D654,L150,U581,L874,U691,L243,U94,L9,D582,L402,U563,R468,U96,L311,D10,R232,U762,R630,D1,L674,U685,R240,D907,R394,U703,L64,U397,L810,D272,L996,D954,R797,U789,R790,D526,R103,D367,R143,D41,L539,D735,R51,D172,L33,U241,R814,D981,R748,D699,L716,U647,L381,D351,L381,D121,L52,U601,R515,U713,L404,U45,R362,U670,L235,U102,R373,U966,L362,U218,R280,U951,R371,U378,L10,U670,R958,D423,L740,U888,R235,U899,L387,U167,R392,D19,L330,D916,R766,D471,L708,D83,R749,D696,L50,D159,R828,U479,L980,D613,L182,D875,L307,U472,L317,U999,R435,D364,R737,U550,L233,U190,L501,U610,R433,U470,L801,U52,L393,D596,L378,U220,L967,D807,R357,D179,L731,D54,L804,D865,L994,D151,L181,U239,R794,D378,L487,U408,R817,U809,R678,D599,L564,U480,R525,D189,L641,D771,L514,U72,L248,D334,L859,D318,R590,D571,R453,U732,R911,U632,R992,D80,R490,D234,L710,U816,L585,U180,L399,D238,L103,U605,R993,D539,R330";
const WIRE_TWO: &str = "L996,U383,L962,U100,L836,D913,R621,U739,R976,D397,L262,D151,L12,U341,R970,U123,L713,U730,L52,D223,L190,D81,R484,D777,R374,U755,R640,D522,R603,D815,R647,U279,R810,U942,R314,D19,L938,U335,R890,U578,R273,U338,R186,D271,L230,U90,R512,U672,R666,D328,L970,U17,R368,D302,L678,D508,L481,U12,L783,D409,L315,D579,L517,D729,R961,D602,R253,D746,R418,D972,R195,D270,L46,D128,L124,U875,R632,D788,L576,U695,R159,U704,R599,D597,R28,D703,L18,D879,L417,U633,L56,U302,R289,U916,R820,D55,R213,U712,R250,D265,L935,D171,L680,U738,L361,D939,R547,D606,L255,U880,R968,U255,R902,D624,L251,U452,L412,D60,L996,D140,L971,U196,R796,D761,L54,U54,L98,D758,L521,U578,L861,U365,L901,D495,L234,D124,L121,D329,L38,U481,L491,D938,L840,D311,L993,D954,R654,U925,L528,D891,L994,D681,L879,D476,L933,U515,L292,U626,R348,D963,L145,U230,L114,D11,R651,D929,R318,D672,R125,D827,L590,U338,L755,D925,L577,D52,R131,D465,R657,D288,R22,D363,R162,D545,L904,D457,R987,D389,L566,D931,L773,D53,R162,U271,L475,U666,L594,U733,R279,D847,R359,U320,R450,D704,L698,D173,R35,D267,L165,D66,L301,U879,R862,U991,R613,D489,L326,D393,R915,U718,R667,U998,R554,U199,R300,U693,R753,U938,R444,U12,L844,D912,R297,D668,R366,U710,L821,U384,R609,D493,R233,U898,R407,U683,R122,U790,L1,U834,L76,U572,R220,U752,L728,D85,L306,D805,R282,U507,R414,D687,L577,U174,L211,U308,L15,U483,R741,D828,L588,D192,L409,D605,L931,U260,L239,D424,L846,U429,L632,U122,L266,D544,R248,U188,R465,U721,R621,U3,L884,U361,L322,U504,R999,U381,R327,U555,L467,D849,R748,U175,R356";

fn dst_point_from_cmd(cmd: &str, origin: Point) -> Point {
    let direction = &cmd[0..1];
    let distance = cmd[1..].parse::<i32>().unwrap();

    let next = match direction {
        "U" => Point {
            x: origin.x,
            y: origin.y + distance,
        },
        "D" => Point {
            x: origin.x,
            y: origin.y - distance,
        },
        "L" => Point {
            x: origin.x - distance,
            y: origin.y,
        },
        "R" => Point {
            x: origin.x + distance,
            y: origin.y,
        },
        _ => panic!(),
    };
    return next;
}

fn wire_up(one_in: &str, two_in: &str, mut matrix: impl Wirer) -> i32 {
    let mut last = Point { x: 0, y: 0 };
    for cmd in one_in.split(",") {
        let next = dst_point_from_cmd(cmd, last);
        println!("BASE: {} {:?}", cmd, next);
        matrix.add_base_point(next);
        last = next;
    }

    last = Point { x: 0, y: 0 };
    for cmd in two_in.split(",") {
        let next = dst_point_from_cmd(cmd, last);
        println!("CHECK: {} {:?}", cmd, next);
        matrix.add_check_point(next);
        last = next;
    }

    return matrix.best_distance();
}

pub fn three_a() -> i32 {
    wire_up(WIRE_ONE, WIRE_TWO, CloseContactWirer::new())
}

pub fn three_b() -> i32 {
    wire_up(WIRE_ONE, WIRE_TWO, LongDistanceWirer::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn start() {
        let mut matrix = CloseContactWirer::new();

        matrix.add_base_point(Point { x: 0, y: 22 });
        matrix.add_base_point(Point { x: 10, y: 22 });

        matrix.add_check_point(Point { x: 3, y: 0 });
        matrix.add_check_point(Point { x: 3, y: 30 });

        for foo in matrix.matrix.iter() {
            println!("{:?}", foo)
        }

        assert_eq!(vec![Point { x: 3, y: 22 }], matrix.intersections);
    }
    #[test]
    fn test_points_between() {
        assert_eq!(
            points_between(Point { x: 0, y: 0 }, Point { x: 0, y: 2 }),
            vec![Point { x: 0, y: 1 }, Point { x: 0, y: 2 }]
        );

        assert_eq!(
            points_between(Point { x: 0, y: 2 }, Point { x: 0, y: 0 }),
            vec![Point { x: 0, y: 1 }, Point { x: 0, y: 0 }]
        );
    }

    #[test]
    fn the_basics() {
        assert_eq!(
            wire_up("R8,U5,L5,D3", "U7,R6,D4,L4", CloseContactWirer::new()),
            6
        );
        assert_eq!(
            wire_up(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
                CloseContactWirer::new(),
            ),
            159
        );
        assert_eq!(
            wire_up(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                CloseContactWirer::new(),
            ),
            135
        );
    }

    #[test]
    fn advanced_wireing() {
        assert_eq!(
            wire_up("R8,U5,L5,D3", "U7,R6,D4,L4", LongDistanceWirer::new()),
            30
        );
        assert_eq!(
            wire_up(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
                LongDistanceWirer::new()
            ),
            610
        );
        assert_eq!(
            wire_up(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                LongDistanceWirer::new()
            ),
            410
        );
    }

    #[test]
    fn alpha() {
        assert_eq!(three_a(), 399)
    }
}
