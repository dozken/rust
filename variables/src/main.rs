fn main() {
    let mut x = 5;
    println!("The values of x={}", x);

    x = 10;
    println!("The mutated values of x={}", x);

    let tup = (1, 1.5, '😃');

    println!("tupple = {:?}", tup);

    println!("sum is={}", sum(10, 20));
    println!("sum2 is={}", sum2());

    fn sum(a: u32, b: u32) -> u32 {
        a + b
    }
    let a = 10;


    for number in (1..5){
        println!("number={}", number);
    }
}

fn sum2() -> u32{
    let y = {
        let x = 3;
        x + 1;
    };

    println!("{:?}",y);

    10
}
