use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// impl<"a","b"> PartialEq<TypeB<"a">> {}

macro_rules! tern {
    ($a:expr=>$b:expr, $c:expr) => {
        if $a {
            $b
        } else {
            $c
        }
    };
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct Point {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct interres {
    code: i32,
    pts: Point,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Quad {
    points: [Point; 4],
    eqs: Vec<Eq>,
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct Eq {
    start: Point,
    end: Point,
    m: f32,
    b: f32,
    vert: bool,
}

#[wasm_bindgen]
pub fn add(a: i32) -> i32 {
    a * 2
}

#[wasm_bindgen]
impl Quad {
    #[wasm_bindgen(constructor)]
    pub fn new(p1: Point, p2: Point, p3: Point, p4: Point) -> Self {
        Self {
            points: [p1, p2, p3, p4],
            eqs: Quad::get_eqs([p1, p2, p3, p4]),
        }
    }
    fn get_eqs(points: [Point; 4]) -> Vec<Eq> {
        let mut eqs: Vec<Eq> = Vec::new();
        for (i, key) in points.iter().enumerate() {
            if i + 1 < points.len() {
                eqs.push(Eq::new_from_points(*key, points[i + 1]));
            } else {
                eqs.push(Eq::new_from_points(*key, points[0]));
            }
        }
        return eqs;
    }
}

#[wasm_bindgen]
pub fn test(pt: Point) -> f32 {
    pt.x
}

#[wasm_bindgen]
impl Eq {
    pub fn new_from_points(start: Point, end: Point) -> Self {
        if start.x == end.x {
            return Self {
                m: f32::INFINITY,
                b: 0.0,
                start,
                end,
                vert: true,
            };
        }
        let m = (start.y - end.y) / (start.x - end.x);
        let b = start.y - (m * start.x);
        return Self {
            m,
            b,
            start,
            end,
            vert: false,
        };
    }
    pub fn f(&self, x: f32) -> Point {
        if self.vert {
            panic!("Vertical Line")
        }
        return Point {
            x,
            y: (self.m * x) + self.b,
        };
    }
    fn count(vec: &Vec<f32>, val: f32) -> usize {
        vec.iter().filter(|&n| *n == val).count()
    }

    pub fn intersections(&self, other: Eq) -> interres {
        // code 0: normal
        // code -1: out of bounds
        // code: -2: parallel
        // code 1: colinear
        let pts: Point;
        if self.m == other.m {
            pts = Point { x: 0.0, y: 0.0 };
            if self.b == other.b {
                return interres { pts, code: 1 };
            }
            return interres { pts, code: -2 };
        }
        if self.vert {
            pts = Point {
                x: self.start.x,
                y: other.f(self.start.x).y,
            };
        } else if other.vert {
            pts = Point {
                x: self.start.x,
                y: self.f(other.start.x).y,
            };
        } else {
            let det = other.m - self.m;

            let x = (self.b - other.b) / det;
            let y = ((-other.m * -self.b) + (self.m * -self.b)) / det;
            pts = Point { x, y };
        }
        let mut vec_s = vec![self.start.x, self.end.x, pts.x];
        let mut vec1 = vec![other.start.x, other.end.x, pts.x];
        let mut vec2 = vec![other.start.y, other.end.y, pts.y];
        let mut vec3 = vec![self.start.y, self.end.y, pts.y];

        vec_s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        vec1.sort_by(|a, b| a.partial_cmp(b).unwrap());
        vec2.sort_by(|a, b| a.partial_cmp(b).unwrap());
        vec3.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // clg(&vec1);

        if Eq::count(&vec1, pts.x) == 1 && vec1[1] != pts.x
            || Eq::count(&vec_s, pts.x) == 1 && vec_s[1] != pts.x
        {
            return interres { code: -1, pts };
        }
        if Eq::count(&vec2, pts.y) == 1 && vec2[1] != pts.y
            || Eq::count(&vec3, pts.x) == 1 && vec3[1] != pts.x
        {
            return interres { code: -1, pts };
        }

        return interres { code: 0, pts };
    }
}

#[wasm_bindgen]
pub fn pip(sh: Quad, pt: Point) -> bool {
    let eqs = sh.eqs;
    // log(format!("{:?}", eqs).as_str());
    let horiz = Eq {
        m: 0.0,
        b: pt.y,
        start: pt,
        end: Point {
            x: f32::INFINITY,
            y: pt.y,
        },
        vert: false,
    };
    // println!("{:?}", horiz);
    let mut ct = 0;
    for line in eqs {
        let u = horiz.intersections(line);
        if u.code > -1 {
            ct += 1;
        }
        // log(format!("{:?}", u).as_str())
    }

    if ct % 2 == 0 {
        return false;
    }

    return true;
}

// fn main() {
//     let shape = Quad {
//         points: [
//             Point { x: 0.0, y: 0.0 },
//             Point { x: 2.0, y: 0.0 },
//             Point { x: 2.0, y: 5.0 },
//             Point { x: 0.0, y: 5.0 },
//         ],
//     };
//     let point = Point { x: 1.0, y: 1.0 };
//     let r = pip(shape, point);
//     println!("{:?}", r);
// }
