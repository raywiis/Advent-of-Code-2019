#[derive(Debug, Clone)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32)
}

fn cmp(a: &Moon, b: &Moon) -> bool {
    a as *const _ == b as *const _
}

impl Moon {
    fn new(position: (i32, i32, i32)) -> Moon {
        Moon {
            position,
            velocity: (0, 0, 0)
        }
    }

    fn apply_gravity(&mut self, others: &Vec<Moon>) {
        for other in others {
            self.gravitate_to(other);
        }
    }

    fn gravitate_to(&mut self, other: &Moon) {
        if self.position.0 > other.position.0 {
            self.velocity.0 -= 1;
        } else if self.position.0 < other.position.0 {
            self.velocity.0 += 1;
        }
        if self.position.1 > other.position.1 {
            self.velocity.1 -= 1;
        } else if self.position.1 < other.position.1 {
            self.velocity.1 += 1;
        }
        if self.position.2 > other.position.2 {
            self.velocity.2 -= 1;
        } else if self.position.2 < other.position.2 {
            self.velocity.2 += 1;
        }
    }

    fn apply_velocity(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn total_energy(&self) -> i64 {
        self.kinetic_energy() * self.potential_energy()
    }

    fn kinetic_energy(&self) -> i64 {
        (self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()) as i64
    }

    fn potential_energy(&self) -> i64 {
        (self.position.0.abs() + self.position.1.abs() + self.position.2.abs()) as i64
    }
}

fn main() {
    let mut moons: Vec<Moon> = vec!(

        // Moon::new((-1, 0, 2)),
        // Moon::new((2, -10, -7)),
        // Moon::new((4, -8, 8)),
        // Moon::new((3, 5, -1))

        Moon::new((-8, -10, 0)),
        Moon::new((5, 5, 10)),
        Moon::new((2, -7, 3)),
        Moon::new((9, -8, -3))

        // Moon::new((-10, -13, 7)),
        // Moon::new((1, 2, 1)),
        // Moon::new((-15, -3, 13)),
        // Moon::new((3, 7, -4))

    );


    println!("{:?}", moons);

    let mut history: Vec<Vec<Moon>> = vec!();
    let mut repeat = 0;

    for step in 0..100000 {
        let old_moons = moons.clone();
        history.push(moons.clone());

        for i in 0..moons.len() {
            moons[i].apply_gravity(&old_moons);
        }

        for moon in &mut moons {
            moon.apply_velocity();
        }

        for i in 0..history.len() {
            let time = &history[i];
            if //time[0].velocity == moons[0].velocity
                // time[1].velocity == moons[1].velocity
                // time[2].velocity == moons[2].velocity
                 time[3].position == moons[3].position
            {
                println!("Position match at {}", i);
                println!("{:?}", time);
                println!("{:?}", moons);
                repeat += 1;
                if repeat == 8 {
                    return;
                }
            }
        }
    }

    let energy_sum = moons.iter()
        .fold(0, |acc, m| acc + m.total_energy());

    println!("Total energy: {}", energy_sum);

}
