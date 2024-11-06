use struct_convert::Convert;

#[derive(Convert)]
#[convert(remote_from = "remote::B")]
pub struct A {
    i: i64,
}

pub mod remote {
    pub struct B {
        pub i: i64,
    }
}

fn main() {
    let a = A { i: 0 };
    let _ = remote::B::from(a);
}
