fn main() {
    // variable binding
    let x = 5u32;

    // expression;
    let y = {
        let x_sqrd = x * x;
        let x_cube = x_sqrd * x;

        // This expression will be assigned to `y`
        x_cube + x_sqrd + x
    };

    let z = {
        2 * x;
    };

    // println!("x is {:?}", x);
    // println!("y is {:?}", y);
    // println!("z is {:?}", z);

    // counter variable
    let mut n = 1;

    // loop while 'n' is less than 1001
    while n < 101 {
        if n % 15 = 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        }
        
        n += 1;
    }
}
