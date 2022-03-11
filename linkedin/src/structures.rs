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
    
    
    fn new ( name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 6,
            propellant: 0.0
        }
    } 
}


#[derive(Debug, Clone)]
struct Rectangle{
    width: f64,
    height: f64
} //width, height

// struct Rectangle(f64, f64); //width, height

impl Rectangle {
    fn get_area(&self) -> f64 {
        // &self.0 * &self.1
        &self.width * &self.height
    }
    
    fn scale(&mut self, scale: f64) {
        self.width *= scale;
        self.height *= scale;
    }
    
    
    fn new(width:f64, height: f64) -> Rectangle {
        // Rectangle(widht, height)
        Rectangle {
            width, 
            height
        }
        
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
    
    let mut vss = Shuttle::new("Essess");
    
    println!("Vss: {:?}", vss);
    
    
    println!("--------------------");
    
    let r  = Rectangle::new(2.0,3.0);
    println!("Rectangle: {:?}", r);
    println!("Area: {}", r.get_area());
    let mut r2 = Rectangle::new(2.0,3.0);
    r2.scale(4.0);
    println!("scaled: {:?}", r2);
    
    
    
}
