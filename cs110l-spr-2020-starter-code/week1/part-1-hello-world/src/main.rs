
fn sum(a :i32, b :i32) -> i32 {
    a + b
}
fn fib(n :i32) -> i32 {
    if n <= 1 {
        n
    }
    else {
        fib(n - 1) + fib(n - 2)
    }
}
fn main() {
    let mut arr : [i32; 4] = [0, 2, 4, 8];
    arr[0] = 9;
    print!("{}\n", arr[0] + arr[1]);
    for i in arr.iter() {
        println!("{}", i);
    }
    let mut i = 0;
    loop {
        i += 1;
        if i == 10 {
            break;
        }
        println!("Fib[{}] = {}", sum(i, 0), fib(i));
    }
}
