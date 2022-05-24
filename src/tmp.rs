
#[macro_export]
#[allow(unused_macros)]
macro_rules! minput {
    // terminator
    (@from [$source:expr] @rest) => {};

    // parse mutability
    (@from [$source:expr] @rest mut $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut [mut]
            @rest $($rest)*
        }
    };
    (@from [$source:expr] @rest $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut []
            @rest $($rest)*
        }
    };

    // parse variable pattern
    (@from [$source:expr] @mut [$($mut:tt)?] @rest $var:tt: $($rest:tt)*) => {
        $crate::input! {
            @from [$source]
            @mut [$($mut)*]
            @var $var
            @kind []
            @rest $($rest)*
        }
    };

    // parse kind (type)
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest) => {
        let $($mut)* $var = $crate::read_value!(@source [$source] @kind [$($kind)*]);
    };
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest, $($rest:tt)*) => {
        $crate::input!(@from [$source] @mut [$($mut)*] @var $var @kind [$($kind)*] @rest);
        $crate::input!(@from [$source] @rest $($rest)*);
    };
    (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
        $crate::input!(@from [$source] @mut [$($mut)*] @var $var @kind [$($kind)* $tt] @rest $($rest)*);
    };

    (from $source:expr, $($rest:tt)*) => {
        #[allow(unused_variables, unused_mut)]
        let mut s = $source;
        $crate::input! {
            @from [&mut s]
            @rest $($rest)*
        }
    };
    ($($rest:tt)*) => {
        let mut locked_stdin = $crate::STDIN_SOURCE.lock().expect(concat!(
            "failed to lock the stdin; please re-run this program.  ",
            "If this issue repeatedly occur, this is a bug in `proconio`.  ",
            "Please report this issue from ",
            "<https://github.com/statiolake/proconio-rs/issues>."
        ));
        $crate::input! {
            @from [&mut *locked_stdin]
            @rest $($rest)*
        }
        drop(locked_stdin); // release the lock
    };
}
