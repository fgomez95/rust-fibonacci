use std::io;

fn main() {
    println!("give me an integer");
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Failed to read input");
    
    let temp: i32 = num.trim().parse()
        .expect("please input a number");

    fibo(temp);
}

fn fibo(num: i32){
    let mut counter = num;
    if num < 0 {
        println!("incorrect output");
    } else if num == 0 {
        println!("result: 0");
    }else if num == 1 {
        println!("1");
    }else {
        let mut x: i32 = 1;
        let mut y: i32 = 0;
        let mut t: i32; 
        while counter > 0 {
            t = x;
            x = x + y;
            y = t;
            counter = counter - 1;
        }
        println!("result: {}", y);
    }
}
