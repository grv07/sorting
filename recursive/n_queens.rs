// 1 1 2 3 5
fn fib(n: i32) -> i64 {
    if n <= 1 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}

fn fib_fast(mut n: i32) -> i64 {
    let mut a = 0;
    let mut b = 1;

    let mut res = 0;
    while n > 0 {
        let temp = b;
        res = a + b;
        a = temp;
        b = res;

        n -= 1;
    }

    res
}

fn main() {
    println!("Hello World!");
    println!("{}", fib_fast(50));
    println!("{}", fib(50));
}
