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

#[cfg(test)]
mod test {
    use super::*;

    enum Treeish {
        Leaf(u32),
        Node(Box<Treeish>, Box<Treeish>),
        AltNode(Box<Treeish>),
    }

    fn next_treeish(input : &Treeish) -> Vec<&Treeish> {
        match input {
            Treeish::Leaf(_) => vec![],
            Treeish::Node(a, b) => vec![a, b],
            Treeish::AltNode(a) => vec![a],
        }
    }

    fn l(input : u32) -> Treeish {
        Treeish::Leaf(input)
    }

    fn n(a : Treeish, b : Treeish) -> Treeish {
        Treeish::Node(Box::new(a), Box::new(b))
    }

    fn alt(a : Treeish) -> Treeish {
        Treeish::AltNode(Box::new(a))
    }

    #[test]
    fn should_return_single_pattern_matches() {
        let f : fn(&Treeish) -> Vec<&Treeish> = all_matches!(next_treeish, Treeish::Leaf(_));

        let input = l(1);

        let results = f(&input);

        assert_eq!(results.len(), 1);
        assert!( matches!( results[0], Treeish::Leaf(1) ) );
    }

    #[test]
    fn should_return_two_pattern_matches() {
        let f : fn(&Treeish) -> Vec<&Treeish> = all_matches!(next_treeish, Treeish::Node(_, _), Treeish::Leaf(_));

        let input = n(l(1), l(2));

        let results = f(&input);

        assert_eq!(results.len(), 2);
        assert!( matches!( results[0], Treeish::Leaf(1) ) );
        assert!( matches!( results[1], Treeish::Leaf(2) ) );
    }

    #[test]
    fn should_follow_path() {
        let f : fn(&Treeish) -> Vec<&Treeish> = all_matches!( next_treeish
                                                            , Treeish::Node(_, _)
                                                            , Treeish::AltNode(_)
                                                            , Treeish::Node(_, _)
                                                            , Treeish::Leaf(_)
                                                            );

        let input = n( alt( n(l(1), l(3)) ), n(l(2), l(4)));

        let results = f(&input);

        assert_eq!(results.len(), 2);
        assert!( matches!( results[0], Treeish::Leaf(1) ) );
        assert!( matches!( results[1], Treeish::Leaf(3) ) );
    }
}