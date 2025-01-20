fn main() {

    // var bindings + type annotations + unused vars

    let y:i32 = 123;
    println!("{}", y);

    // using x after first line only refers to second x
    // first x still exists (until scope),
    // can no longer refer to it
    // (shadowing bindings)

    let x = 10;
    let x = x + 3;

    println!("{}", x);

    // tuples (fixed length collections of values of diff types)
    // with type annotation
    
    let pair: (char, i32) = ('a', 17);
    println!("{:?}", pair);
}
