fn main() {

    println!("hello, world");

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

    // destructuring (breaking down a tuple)
    let (some_char, some_int) = pair;
    println!("{} {}", some_char, some_int);

    // btw, statements can obviously span multiple lines, semi colon marks
    // EOS

    // functions, arrow indicates return type
    fn _fair_dice_roll() -> i32 {
        return 4;
    }

    // blocks, pair of brackets declares it
    // has its own scope
    // are also expressions, so evaluates to a value

    let z = {
        let z = 1; // different x as scopes are different
        let y = 2;
        z + y // so called 'tail', what the whole block will evaluate to
    };
    println!("{}", z);

}
