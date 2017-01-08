extern crate avec;
use avec::AVec;

fn main() {

    let x = AVec::<u32>::new(32);

    x.push(123);
    x.push(345);
    x.map(4)
        .set(0, 567)
        .set(1, 890)
        .set(2, 234)
        .set(3, 567);

    println!("{:?}", x);
}
