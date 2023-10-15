#[macro_export]
macro_rules! error {
    (@preamble) => {{
        print!(
            "{}:{},{}",
            file!(),
            line!(),
            column!(),
        );
    }};
    () => {
        error!(@preamble);
        println!();
    };
    ($first:expr $(, $rest:expr)* $(,)*) => {
        error!(@preamble);
        print!(" (");
        print!("{} = {:?}", stringify!($first), $first);
        $(print!(", {} = {:?}", stringify!($rest), $rest);)*
        print!(")");
        println!();
    };
}

#[macro_export]
macro_rules! dump {
    ($first:expr $(, $rest:expr)* $(,)*) => {
        print!("DUMP: ");
        print!("{}", $first);
        println!();
    };
}
