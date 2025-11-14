use num_bigint::BigInt;
use num_traits::Zero;
//use num_traits::One;

#[derive(Debug, Clone)]
struct Point {
    x: BigInt,
    y: BigInt,
}

struct EC {
    a1: BigInt,
    a2: BigInt,
    a3: BigInt,
    a4: BigInt,
    a6: BigInt,
    p: BigInt,
}

impl EC {
    fn double_point(&self, pt: &Point) -> Option<Point> {
        let num = 3 * &pt.x * &pt.x + 2 * &self.a2 * &pt.x - &self.a1 * &pt.y + &self.a4;
        let u: BigInt = 2 * &pt.y + &self.a1 * &pt.x + &self.a3;
        match u.modinv(&self.p) {
            Some(den) => {
                let l = num * den % &self.p;
                let x3 = &l * &l + &l * &self.a1 - &self.a2 - 2 * &pt.x;
                let y3 = - &self.a1 * &x3 - &self.a3 - &l * &x3 + &l * &pt.x - &pt.y;
                let pt3 = Point {
                    x: if &x3 % &self.p >= BigInt::from(0) {x3 % &self.p} else {x3 % &self.p + &self.p},
                    y: if &y3 % &self.p >= BigInt::from(0) {y3 % &self.p} else {y3 % &self.p + &self.p},
                };
                Some(pt3)
            },
            None => {
                println!("gcd({}, {}) > 1", u, self.p);
                None
            },
        }
    }

    fn add_points(&self, pt1: &Point, pt2: &Point) -> Option<Point> {
        let num = &pt2.y - &pt1.y;
        let u = &pt2.x - &pt1.x;
        match u.modinv(&self.p) {
            Some(den) => {
                let l = num * den % &self.p;
                let x3 = &l * &l + &self.a1 * &l - &self.a2 - &pt1.x - &pt2.x;
                let y3 = - &self.a1 * &x3 - &self.a3 - &l * &x3 + &l * &pt1.x - &pt1.y;
                let pt3 = Point {
                    x: if &x3 % &self.p >= BigInt::from(0) {x3 % &self.p} else {x3 % &self.p + &self.p},
                    y: if &y3 % &self.p >= BigInt::from(0) {y3 % &self.p} else {y3 % &self.p + &self.p},
                };
                Some(pt3)
            },
            None => {
                println!("gcd({}, {}) > 1", &pt2.x - &pt1.x, &self.p);
                None
            },
        }
    }

    fn power_point(&self, pt: &Point, n: u32) -> Option<Point> {
        let mut num = n;
        let mut n_pt: Option<Point> = if n % 2 == 1 {Some(pt.clone())} else {None};
        let mut pt_t = pt.clone();

        num = num >> 1;
        while num != 0 {
            match self.double_point(&pt_t) {
                Some(pt_d) => {
                    pt_t = pt_d;
                    if num % 2 == 1 {
                        match n_pt {
                            Some(pt_s0) => {
                                match self.add_points(&pt_s0, &pt_t) {
                                    Some(pt_s1) => {
                                        n_pt = Some(pt_s1);
                                    },
                                    None => {
                                        return None;
                                    },
                                }
                            },
                            None => {
                                n_pt = Some(pt_t.clone());
                            },
                        }
                    }
                },
                None => {
                    return None
                },
            }
            num = num >> 1;
        }

        n_pt
    }

    fn test(&self, pt: &Point) -> bool {
        (&pt.y * &pt.y + &self.a1 * &pt.x * &pt.y + &self.a3 * &pt.y - &pt.x * &pt.x * &pt.x - &self.a2 * &pt.x * &pt.x - &self.a4 * &pt.x - &self.a6) % &self.p == BigInt::zero()
    }
}

fn main() {
    let pt = Point {
        x: BigInt::from(125),
        y: BigInt::from(18),
    };
    let a = BigInt::from(301);
    let b = &pt.y * &pt.y - &pt.x * &pt.x * &pt.x - &a * &pt.x;

    let ec = EC {
        a1: BigInt::from(0),
        a2: BigInt::from(0),
        a3: BigInt::from(0),
        a4: a,
        a6: b,
        p: BigInt::from(829),
    };

    let pt_n = ec.power_point(&pt, 633 as u32);
    if ec.test(&pt_n.clone().unwrap()) {
        println!("{:?}", pt_n.unwrap());
    }
    
   /*
    loop {
        if !ec.test(&pt) {
            break;
        }
        println!("{:?}", pt);
        pt = ec.double_point(&pt);
    }
    */
}
