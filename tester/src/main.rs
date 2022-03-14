use linkedin;

fn main() {
    println!("Hello, world!");
    linkedin::min_max();
    linkedin::str();
    linkedin::str_trim();
    println!("ARGUMENTS::");
    linkedin::args();

    linkedin::read_file();
    linkedin::write_file();
    linkedin::finder();
    linkedin::structs();
    linkedin::threads();

    linkedin::maxes();
    linkedin::boxing();


    linkedin::describe();

    linkedin::lifetime();
    linkedin::test_enums();
    
    linkedin::test_errors();
}
