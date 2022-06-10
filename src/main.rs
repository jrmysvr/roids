
mod boid {
    use std::fmt;
    use uuid::Uuid;

    #[derive(Debug, Clone)]
    pub struct UUID {
        value: Uuid,
    }

    impl UUID {
        fn new() -> UUID {
            UUID {
                value: Uuid::new_v4(),
            }
        }
    }

    impl fmt::Display for UUID {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    #[derive(Debug, Clone)]
    pub enum BoidKind {
        Dumb,
    }

    type Length = f64;
    type Speed = f64;

    #[derive(Debug, Clone)]
    pub struct Position {
        pub x: Length,
        pub y: Length,
    }

    impl Default for Position {
        fn default() -> Position {
            Position { x: 0.0, y: 0.0 }
        }
    }

    impl Position {
        pub fn distance_to(&self, other: &Position) -> Length {
            let x = self.x - other.x;
            let y = self.y - other.y;
            (x*x + y*y).sqrt()
        }
    }

    #[derive(Debug, Clone)]
    pub struct Boid {
        pub id: UUID,
        pub kind: BoidKind,
        pub position: Position,
        pub speed: Speed,
    }

    impl Boid {
        fn new() -> Boid {
            Boid {
                id: UUID::new(),
                kind: BoidKind::Dumb,
                position: Position::default(),
                speed: Speed::default(),
            }
        }

        pub fn react_to(&mut self, _boids: &Population) {
        }

        pub fn distance_to(&self, other: &Boid) -> Length{
            self.position.distance_to(&other.position)
        }
    }

    #[derive(Clone)]
    pub struct Population {
        next_ix: usize,
        boids: Vec<Boid>
    }

    impl Population {
        pub fn create(n_boids: u32) -> Population {
            let mut boids = Vec::<Boid>::new();
            for _ in 0..n_boids {
                boids.push(Boid::new());
            }
            Population {
                next_ix: 0,
                boids,
            }
        }

        pub fn get(&self, boid_ix: usize) -> &Boid {
            &self.boids[boid_ix]
        }

        pub fn size(&self) -> usize {
            self.boids.len()
        }
    }


    impl Iterator for Population {

        type Item = Boid;

        fn next(&mut self) -> Option<Self::Item> {
            if self.next_ix < self.boids.len() {
                let next_boid = &self.boids[self.next_ix]; self.next_ix += 1;
                return Some(next_boid.clone());
            }
            None
        }
    }

}

mod boid_rule {
    use super::boid;

    pub trait BoidRule {
        // fn compare
        // fn op
        fn use_on(&self, boid_ix: usize, boids: &boid::Population);
    }

    pub struct Avoid {
        n_nearest: u64,
    }

    impl Avoid {
        const N_NEAREST: u64 = 3;
        pub fn new() -> Avoid {
            Avoid {
                n_nearest: Avoid::N_NEAREST,
            }
        }

    }

    impl BoidRule for Avoid {
        fn use_on(&self, boid_ix: usize, boids: &boid::Population) {
            println!("Avoiding {:#?}!", boids.get(boid_ix));

            let target_boid = boids.get(boid_ix);
            for ix in 0..boids.size() {
                if ix != boid_ix {
                    let boid = boids.get(ix);
                    let distance = boid.distance_to(target_boid);
                    println!("Distance to {}: {}", boid.id, distance);
                }
            }
        }
    }

    pub struct Attract {}

    impl Attract {
        pub fn new() -> Attract {
            Attract { }
        }
    }

    impl BoidRule for Attract {
        fn use_on(&self, boid_ix: usize, boids: &boid::Population) {
            println!("Attracting {:#?}!", boids.get(boid_ix));
        }
    }

    pub trait RuleTrait: BoidRule + Send {}
    impl RuleTrait for Avoid {}
    impl RuleTrait for Attract {}
}

enum Rules {
    Attract,
    Avoid,
}

// The rules applied in the simplest Boids world are as follows:
// - separation: steer to avoid crowding local flockmates
// - alignment: steer towards the average heading of local flockmates
// - cohesion: steer to move towards the average position (center of mass) of local flockmates
struct World {
    // rule: Box<dyn RuleTrait + Send>,
    rule: Box<dyn boid_rule::RuleTrait>,
    boids: boid::Population,
}

impl World {

    const N_WORKERS: usize = 10;

    fn a_whole_new_world(rule_selection: Rules, n_boids: u32) -> World {
        let rule: Box<dyn boid_rule::RuleTrait> = match rule_selection {
            Rules::Attract => Box::new(boid_rule::Attract::new()),
            Rules::Avoid => Box::new(boid_rule::Avoid::new()),
        };
        World {
            rule,
            boids: boid::Population::create(n_boids),
        }
    }

    fn turn(self) {
        /*
        let pool = ThreadPool::new(World::N_WORKERS);
        for i in 0..self.boids.size() {
            pool.execute( move || {
                println!("{:#?}", &self.boids.get(i));
                self.rule.use_on(i, &self.boids);
                // &boid.react_to(&self.boids);
            });
        }
        */

        for i in 0..self.boids.size() {
            self.rule.use_on(i, &self.boids);
        }
    }
}

fn main() {
    let n_boids = 100_000;
    let world = World::a_whole_new_world(Rules::Avoid, n_boids);
    world.turn();
    let world = World::a_whole_new_world(Rules::Attract, n_boids);
    world.turn();
}
