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

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 6,
            propellant: 0.0,
        }
    }
}


fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a + *b)
}



pub fn boxing() {
    let v = Shuttle::new("Hello");
    println!("struct stack size: {} bytes", std::mem::size_of_val(&v));

    let boxed: Box<Shuttle> = Box::new(v);

    println!("boxed stack size: {} bytes", std::mem::size_of_val(&boxed));
    println!("boxed heap size: {} bytes", std::mem::size_of_val(&*boxed));
    
    let unbox: Shuttle = *boxed;
    println!("UNboxed stack size: {} bytes", std::mem::size_of_val(&unbox));
    
    println!("------------");
    
    let a = Box::new(1);
    let b = Box::new(2);
    
    let c = *sum_boxes(a,b);
    println!("Sume 1+2 box = {}", c);

    let pi = Box::new(3.14);
    let e = Box::new(2.718);

    let c = *sum_boxes(pi,e);
    println!("Sume pi+e box = {}", c);
    
}
