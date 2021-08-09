#[macro_export]
macro_rules! all_matches_no_call{
    ($ret:ident, $next:expr, $structure:expr, $p:pat) => {
        match $structure {
            it @ $p => $ret.push(it),
            _ => { },
        }
    };

    ($ret:ident, $next:expr, $structure:expr, $p:pat, $($rest:tt)*) => {
        match $structure {
            it @ $p => {
                for item in $next(it) {
                    all_matches_no_call!($ret, $next, item, $($rest)* );
                }
            },
            _ => { },
        }
    };
}

#[macro_export]
macro_rules! all_matches {
    ($next:expr, $($rest:tt)+) => {
        |structure| {
            let mut ret = vec![];
            all_matches_no_call!(ret, $next, structure, $($rest)+);
            ret
        }
    };
}