#[macro_export]
macro_rules! l {
    ($($x:expr),*) => {{
        let mut v = vec![];
        $(v.push($crate::Rc::new($x));)*
        $crate::List::new(v)
    }};
    ($($x:expr,)*) => (l![$($x),*])
}
