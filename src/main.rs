fn main() {
    fn main() {
        let num = 0x2a;
        println!("num is {}", num);
        let num = 0o106;
        println!("num is {}", num);
        let num = 0b1011_1110;
        println!("num is {}", num);
        println!("{:?}", std::f32::INFINITY);
        println!("{:?}", std::f32::NEG_INFINITY);
        println!("{:?}", std::f32::NAN);
        println!("{:?}", std::f32::MAX);
        println!("{:?}", std::f32::MIN);
        #assert_eq!((1..10), std::ops::Range(start: 1, end: 10));
    }

}
