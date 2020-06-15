pub use russ_internal_macro::{CSSDeclaration, CSSValue, FromVariants, VariantConstructors};
use std::io::{self, Write};

pub type WriteResult<T = ()> = io::Result<T>;

pub struct CSSWriter<'a> {
    buf: &'a mut (dyn Write + 'a),
}
impl<'a> CSSWriter<'a> {
    pub fn new(buf: &'a mut (dyn Write + 'a)) -> Self {
        Self { buf }
    }

    pub fn write_char(&mut self, c: char) -> WriteResult {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    pub fn write_str(&mut self, s: &str) -> WriteResult {
        self.write_all(s.as_bytes())
    }
}
impl Write for CSSWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf.flush()
    }
}

pub trait WriteValue {
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult;
}

pub trait WriteDeclaration: WriteValue {
    fn write_property(&self, f: &mut CSSWriter) -> WriteResult;
}

impl<T> WriteValue for Box<T>
where
    T: WriteValue,
{
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        self.as_ref().write_value(f)
    }
}
// TODO this implementation should only exist on Multiple<>
impl<T> WriteValue for Vec<T>
where
    T: WriteValue,
{
    fn write_value(&self, f: &mut CSSWriter) -> WriteResult {
        if let Some((last, rest)) = self.split_last() {
            for v in rest {
                v.write_value(f)?;
                f.write_char(',')?;
            }
            last.write_value(f)?;
        }

        Ok(())
    }
}

/// Macro for creating a [`Multiple`].
///
/// [`Multiple`]: ./struct.Multiple.html
#[macro_export]
macro_rules! multiple {
    ($v:expr, $($others:expr),+ $(,)?) => (
        ::russ::css::Multiple::__unchecked_new(::std::vec![$v, $($others),*])
    );
}
