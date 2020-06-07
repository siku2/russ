pub use russ_css_macro::{CSSDeclaration, CSSValue, FromVariants, VariantConstructors};
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

pub struct Declaration {
    inner: Box<dyn WriteDeclaration>,
}
impl Declaration {
    fn write_declaration(&self, f: &mut CSSWriter) -> WriteResult {
        self.inner.write_property(f)?;
        f.write_char(':')?;
        self.inner.write_value(f)
    }
}

pub struct DeclarationBlock(Vec<Declaration>);
impl DeclarationBlock {
    fn write_block(&self, f: &mut CSSWriter) -> WriteResult {
        f.write_char('{')?;
        for decl in &self.0 {
            decl.write_declaration(f)?;
            f.write_char(';')?;
        }
        f.write_char('}')
    }
}

pub trait MaybeWriteValue {
    fn maybe_write_value(&self, f: &mut CSSWriter) -> WriteResult<bool>;
}

impl<T> MaybeWriteValue for T
where
    T: WriteValue,
{
    fn maybe_write_value(&self, f: &mut CSSWriter) -> WriteResult<bool> {
        self.write_value(f).map(|_| true)
    }
}

impl<T> MaybeWriteValue for Option<T>
where
    T: MaybeWriteValue,
{
    fn maybe_write_value(&self, f: &mut CSSWriter) -> WriteResult<bool> {
        if let Some(v) = self {
            v.maybe_write_value(f)
        } else {
            Ok(false)
        }
    }
}
