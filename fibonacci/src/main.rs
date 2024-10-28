fn main() {
    let n: u128 = loop {
        println!("Enter position of Fibonacci number: ");
        let mut n = String::new();
        match std::io::stdin().read_line(&mut n) {
            Ok(_) => {},
            Err(e) => println!("Failed to read the line at {e}."),
        }
        match n.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a positive number.");
                continue;
            }
        }
    };
    println!("The value at position {n} is: {}.", nth_fibonacci(n));
}

fn nth_fibonacci(n: u128) -> u128 {
    if n <= 1 {n} else {
        let mut mat1: [[u128; 2]; 2] = [[1, 1], [1, 0]];
        matrix_power(&mut mat1, n - 1);
        mat1[0][0]
    }
}

fn matrix_power(mat1: &mut [[u128; 2]; 2], n: u128) {
    if n == 0 || n == 1 {return} else {
        let mat2: [[u128; 2]; 2] = [[1, 1], [1, 0]];
        matrix_power(mat1, n / 2);
        multiply(mat1, *mat1);
        if n % 2 != 0 {multiply(mat1, mat2);}
    }
}

fn multiply(mat1: &mut [[u128; 2]; 2], mat2: [[u128; 2]; 2]) {
    let x = mat1[0][0] * mat2[0][0] + mat1[0][1] * mat2[1][0];
    let y = mat1[0][0] * mat2[0][1] + mat1[0][1] * mat2[1][1];
    let z = mat1[1][0] * mat2[0][0] + mat1[1][1] * mat2[1][0];
    let w = mat1[1][0] * mat2[0][1] + mat1[1][1] * mat2[1][1];

    mat1[0][0] = x;
    mat1[0][1] = y;
    mat1[1][0] = z;
    mat1[1][1] = w;
}