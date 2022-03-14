use std::any;
use std::fmt;

struct Satellite {
    name: String,
    velocity: f64
}

struct SpaceStation {
    name: String,
    crew_size: i8,
    altitude: u32 //miles
}

//how to use traits:
trait Describe{
    fn describe(&self) -> String;
}

// we can have default implementations

impl dyn Describe {
    fn describe(&self) -> String {
        String::from("flying object")
    }
}

// this need to be done for all
impl Describe for Satellite {
    fn describe(&self) -> String {
        format!("the {} fying at {} miles/sec", self.name, self.velocity)
    }
}


impl Describe for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} fying at {} miles high with  {} srew members", self.name, self.altitude, self.crew_size)
    }
}





// fn print_type<T: fmt::Display>(item: T) {
fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {}",item, any::type_name::<T>())
}



fn compare_and_print<T: fmt::Display + PartialEq + From<U>,U: fmt::Display+ PartialEq + Copy>(a:T, b:U) {
    if a == T::from(b) {
        println!("{} is equal to {}", a,b);
    } else {
        println!("{} is NOT equal to {}", a,b);
    }
}



impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Satelite: {} on {} m/sec", self.name, self.velocity)
    }
}


fn compare_and_print2<T,U>(a:T, b:U) 
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display+ PartialEq + Copy
{
    if a == T::from(b) {
        println!("{} is equal to {}", a,b);
    } else {
        println!("{} is NOT equal to {}", a,b);
    }
}





pub fn describe(){
    let hubble = Satellite {
        name : String::from("Hubble Telescope"),
        velocity:  4.72
    };
    
    let iss = SpaceStation {
        name: String::from("IESSESS"),
        crew_size: 9,
        altitude: 12
    };
    
    println!("Hubble: {}", hubble.describe());
    println!("Station: {}", iss.describe());
    
    print_type(3);
    print_type(3.14);
    print_type("3333");
    print_type([33,3]);
    
    compare_and_print(3.0,3);
    compare_and_print2(3.0,3);

    println!("Hubble: {}", hubble);
}