// _ + _ * _^2 + _^3 - _ = 399
use itertools::Itertools;
//  2 3 5 7 9

#[derive(Debug)]
pub enum Coin {
    Blue,
    Red,
    Corroded,
    Concave,
    Shiny,
}

impl Coin {
    pub fn value(&self) -> u16 {
        match self {
            Coin::Blue => 9,
            Coin::Red => 2,
            Coin::Corroded => 3,
            Coin::Concave => 7,
            Coin::Shiny => 5,
        }
    }
}

pub fn solve_equation() {
    for v in vec![
        Coin::Blue,
        Coin::Red,
        Coin::Corroded,
        Coin::Concave,
        Coin::Shiny,
    ]
    .iter()
    .permutations(5)
    {
        let a = v[0];
        let b = v[1];
        let c = v[2];
        let d = v[3];
        let e = v[4];
        if a.value() + b.value() * c.value().pow(2) + d.value().pow(3) - e.value() == 399 {
            println!("{:?} {:?} {:?} {:?} {:?}", a, b, c, d, e);
        }
    }
}
