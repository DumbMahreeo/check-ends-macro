/// Checks if string starts with any of the `&str`s
#[macro_export]
macro_rules! check_start {
    ($to_check:expr, $( $( $start:literal)|* => $to_return:expr )* ) => {
        if false {None} // This is here to use else if later
        $(
            else if check_start!(@INTERNAL $to_check, $($start),*) {
                Some($to_return)
            }
        )*
        else {None}
    };

    (@INTERNAL $to_check:expr, $($start:literal),*) => {
        $(
            $to_check.starts_with($start)
        )||*
    }
}

/// Checks if string ends with any of the `&str`s
#[macro_export]
macro_rules! check_end {
    ($to_check:expr, $( $( $end:literal)|* => $to_return:expr )* ) => {
        if false {None} // This is here to use else if later
        $(
            else if check_end!(@INTERNAL $to_check, $($end),*) {
                Some($to_return)
            }
        )*
        else {None}
    };

    (@INTERNAL $to_check:expr, $($start:literal),*) => {
        $(
            $to_check.ends_with($start)
        )||*
    }
}
