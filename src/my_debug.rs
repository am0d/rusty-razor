#![feature(macro_rules)]

#[macro_export]
macro_rules! dump(
    ($a:expr) => (
        println!(concat!(file!(), ":", line!(), " ", stringify!($a), " = {:?}"), $a);
        );
    ($a:expr, $($b:expr),+) => (
        println!(
            concat!(
                file!(), ":", line!(), " ",
                stringify!($a), " = {:?}",
                $(", ", stringify!($b), " = {:?}"),+
            ),
            $a,
            $($b),+
        );
    );
)


