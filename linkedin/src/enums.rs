#[derive(Debug)]
enum Shape{
    Triange(f64,f64,f64),
    Rectangle(f64,f64),
    Circle(f64)
}
    
impl Shape {
    fn get_perimeter(&self) -> f64{
        match self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(x,y ) => (2.0 * x) + (2.0 * y),
            Shape::Triange(a,b,c) => a + b + c 
        }
    }
}

pub fn testme(){
    let c = Shape::Circle(3.14);
    
    println!("shape is {:?}", c);
    
    
    match c {
        Shape::Triange(a,b,c) => println!("Triangle: a:{} b:{} c:{}",a,b,c),
        Shape::Circle(r) => println!("Circle , radius {}",r),
        Shape::Rectangle(x,y) => println!("Rect [{},{}]",x,y)
    }
    
    
    
    let x = 10u8;
    
    let res = match x {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => {
            println!("{} not match", x);
            "unknown"
        }
    };
    
    println!("Result: {}",res);

    println!("Preimeter {}", c.get_perimeter());
}