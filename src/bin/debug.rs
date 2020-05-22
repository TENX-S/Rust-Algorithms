use std::fmt::Debug;

struct Builder;

impl Builder {
    fn build<I: Debug>(&self) -> fn(I) -> I {
        fn input<I: Debug>(x: I) -> I { x }
        input
    }
}

fn test<F, T: Debug>(gen: F)
    where F: Fn(Builder) -> T
{
    let builder = Builder;
    println!("{:?}", gen(builder));
}

fn main() {
    test(|builder| {
        builder.build()(10);
        builder.build()(10.0)
    });
}