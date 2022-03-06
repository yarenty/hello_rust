#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: i32,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, litters: f64) {
        self.propellant += litters
    }

    fn increase_populaiton(&mut self, kids: i32) {
        self.crew_size += kids
    }
}

pub fn vehicles() {
    let mut v = Shuttle {
        name: String::from("Star Trek"),
        crew_size: 6,
        propellant: 9.999,
    };

    let v2 = Shuttle {
        name: String::from("Star Wars"),
        ..v
    };

    let v3 = Shuttle { ..v.clone() };
    println!("V1: {:?}", v);
    println!("V2: {:?}", v2);
    println!("V3: {:?}", v3);

    println!("NAME v1: {}", v.get_name());

    v.add_fuel(1.0);

    println!("now: {}", v.propellant);
    
    v.increase_populaiton(3);
    println!("V1: {:?}", v);
    
}
