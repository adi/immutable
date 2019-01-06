#[macro_export]
macro_rules! v {
    ($($x:expr),*) => {{
        let mut v = vec![];
        $(v.push($crate::Rc::new($x));)*
        $crate::Vector::new(v)
    }};
    ($($x:expr,)*) => (v![$($x),*])
}
