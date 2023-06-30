// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ( $x1:expr, $x2:expr ) => {
        println!("Double argument: {}, {}", $x1, $x2);
    };
    ( $( $x:expr ),* ) => {
        print!("Look at this other macro:");
        $(
            print!(" {}", $x);
        )*
        println!("");
    }
}

fn main() {
    my_macro!();
    my_macro!(7777, 45);
}
