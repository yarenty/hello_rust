pub fn min_max() {
    let arr = [1, 2, 3, 4, 4];
    let mut min: i32;
    let mut max: i32;
    let mut mean: f64;

    mean = 0.0;
    min = arr[0];
    max = arr[0];
    // for n in arr {
    for &n in arr.iter() {
        if n < min {
            min = n;
        }
        if n > max {
            max = n;
        }
        mean += n as f64;
    }

    mean /= arr.len() as f64;

    println!(" min = {};  max = {}; mean = {}", min, max, mean);
}
