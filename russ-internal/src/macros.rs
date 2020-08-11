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
