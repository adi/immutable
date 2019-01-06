#[macro_export]
macro_rules! s {
    ($($x:expr),*) => {{
        let mut v = vec![];
        $(v.push($crate::Rc::new($x));)*
        $crate::Set::new(v)
    }};
    ($($x:expr,)*) => (s![$($x),*])
}
