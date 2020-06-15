pub mod props;
pub mod values;
pub use russ_internal::{
    multiple, CSSDeclaration, CSSValue, CSSWriter, WriteDeclaration, WriteResult, WriteValue,
};
use std::{iter, ops::Deref};

/// Collection of at least one item.
/// Internally this is a vector but with some additional guards.
///
/// See the [`multiple!`] macro.
///
/// Implements [`From<T>`](From) for a single value `T`.
///
/// [`multiple!`]: ./macro.multiple.html
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, CSSValue)]
pub struct Multiple<T>(Vec<T>);
impl<T> Multiple<T> {
    fn unchecked_new(v: Vec<T>) -> Self {
        Self(v)
    }

    /// This should only be used by the macro.
    #[doc(hidden)]
    pub fn __unchecked_new(v: Vec<T>) -> Self {
        Self::unchecked_new(v)
    }

    pub fn new(v: Vec<T>) -> Option<Self> {
        if v.is_empty() {
            None
        } else {
            Some(Self::unchecked_new(v))
        }
    }

    /// # Panics
    ///
    /// Panics if the given vector is empty.
    fn new_must(v: Vec<T>) -> Self {
        Self::new(v).expect("must not be empty")
    }

    pub fn build<IT, V>(v: IT) -> Option<Self>
    where
        IT: IntoIterator<Item = V>,
        V: Into<T>,
    {
        Self::new(v.into_iter().map(Into::into).collect())
    }

    pub fn one(v: impl Into<T>) -> Self {
        Self::unchecked_new(vec![v.into()])
    }

    pub fn one_and_more<V1, IT, V>(v: V1, others: IT) -> Self
    where
        V1: Into<T>,
        IT: IntoIterator<Item = V>,
        V: Into<T>,
    {
        Self::unchecked_new(
            iter::once(v.into())
                .chain(others.into_iter().map(Into::into))
                .collect(),
        )
    }
}
impl<T> Deref for Multiple<T> {
    type Target = <Vec<T> as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> From<T> for Multiple<T> {
    fn from(v: T) -> Self {
        Self::one(v)
    }
}
impl<T> Into<Vec<T>> for Multiple<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}

/// Collection of one to four items as represented by `<item>{1,4}`.
///
/// Implements `From` for tuples with length 1 to 4 where each item implements `Into<T>`.
/// To use a 1-tuple use the following syntax: `(T,)`.
#[allow(clippy::type_complexity)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OneToFour<T>(T, Option<(T, Option<(T, Option<T>)>)>);
impl<T> OneToFour<T> {
    fn unchecked_from_linear(v0: T, v1: Option<T>, v2: Option<T>, v3: Option<T>) -> Self {
        Self(v0, v1.map(|v1| (v1, v2.map(|v2| (v2, v3)))))
    }

    pub fn one(v0: impl Into<T>) -> Self {
        Self(v0.into(), None)
    }

    pub fn two(v0: impl Into<T>, v1: impl Into<T>) -> Self {
        Self(v0.into(), Some((v1.into(), None)))
    }

    pub fn three(v0: impl Into<T>, v1: impl Into<T>, v2: impl Into<T>) -> Self {
        Self::unchecked_from_linear(v0.into(), Some(v1.into()), Some(v2.into()), None)
    }

    pub fn four(v0: impl Into<T>, v1: impl Into<T>, v2: impl Into<T>, v3: impl Into<T>) -> Self {
        Self::unchecked_from_linear(v0.into(), Some(v1.into()), Some(v2.into()), Some(v3.into()))
    }

    pub fn as_linear(&self) -> (&T, Option<&T>, Option<&T>, Option<&T>) {
        let v0 = &self.0;
        match &self.1 {
            None => (v0, None, None, None),
            Some((v1, None)) => (v0, Some(v1), None, None),
            Some((v1, Some((v2, None)))) => (v0, Some(v1), Some(v2), None),
            Some((v1, Some((v2, Some(v3))))) => (v0, Some(v1), Some(v2), Some(v3)),
        }
    }

    pub fn count(&self) -> u8 {
        match self.1 {
            None => 1,
            Some((_, None)) => 2,
            Some((_, Some((_, None)))) => 3,
            Some((_, Some((_, Some(_))))) => 4,
        }
    }
}
impl<T> WriteValue for OneToFour<T>
where
    T: WriteValue,
{
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        let Self(v0, next) = self;
        v0.write_value(f)?;

        if let Some((v1, next)) = next {
            f.write_char(' ')?;
            v1.write_value(f)?;

            if let Some((v2, next)) = next {
                f.write_char(' ')?;
                v2.write_value(f)?;

                if let Some(v3) = next {
                    f.write_char(' ')?;
                    v3.write_value(f)?;
                }
            }
        }

        Ok(())
    }
}
impl<T, T1> From<(T1,)> for OneToFour<T>
where
    T1: Into<T>,
{
    fn from((v0,): (T1,)) -> Self {
        Self::one(v0)
    }
}
impl<T, T1, T2> From<(T1, T2)> for OneToFour<T>
where
    T1: Into<T>,
    T2: Into<T>,
{
    fn from((v0, v1): (T1, T2)) -> Self {
        Self::two(v0, v1)
    }
}
impl<T, T1, T2, T3> From<(T1, T2, T3)> for OneToFour<T>
where
    T1: Into<T>,
    T2: Into<T>,
    T3: Into<T>,
{
    fn from((v0, v1, v2): (T1, T2, T3)) -> Self {
        Self::three(v0, v1, v2)
    }
}
impl<T, T1, T2, T3, T4> From<(T1, T2, T3, T4)> for OneToFour<T>
where
    T1: Into<T>,
    T2: Into<T>,
    T3: Into<T>,
    T4: Into<T>,
{
    fn from((v0, v1, v2, v3): (T1, T2, T3, T4)) -> Self {
        Self::four(v0, v1, v2, v3)
    }
}
