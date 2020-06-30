/// Macro for creating a [`Multiple`].
///
/// [`Multiple`]: ./struct.Multiple.html
#[macro_export]
macro_rules! multiple {
    ($v:expr, $($others:expr),+ $(,)?) => (
        // SAFETY: we have at least 1 element
        unsafe { ::russ::css::Multiple::unchecked_new(::std::vec![$v, $($others),*]) }
    );
}

/// Macro for creating a [`Vec<T>`](Vec) from elements that implement [`Into<T>`](Into).
/// Apart from this it behaves exactly like the [`vec!`](vec) macro.
#[macro_export]
macro_rules! vec_into {
    () => (
        ::std::vec![]
    );
    ($elem:expr; $n:expr) => (
        ::std::vec![::std::convert::Into::into($elem); $n]
    );
    ($($x:expr),+ $(,)?) => (
        ::std::vec![$(::std::convert::Into::into($x)),*]
    );
}

#[cfg(test)]
mod tests {
    /// this is mostly to make sure that the macro compiles
    #[test]
    fn vec_into_works() {
        let v: Vec<String> = vec_into![];
        assert!(v.is_empty());
        let v: Vec<String> = vec_into!["a"; 5];
        assert_eq!(v.len(), 5);
        let v: Vec<String> = vec_into!["a", "b"];
        assert_eq!(&v[0], "a");
        assert_eq!(&v[1], "b");
    }
}
