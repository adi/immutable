#[macro_export]
macro_rules! h {
    ($($key:expr,$value:expr),*) => {{
        let mut pairs = vec![];
        $(pairs.push($crate::Pair::new($crate::Rc::new($key), $crate::Rc::new($value)));)*
        $crate::Hashmap::new(pairs)
    }};
    ($($key:expr,$value:expr,)*) => (h![$($key,$value),*])
}
