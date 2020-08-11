mod macros;

pub use russ_internal_macro::{CssDeclaration, CssValue, FromVariants, VariantConstructors};
use std::io::{self, Write};

pub type WriteResult<T = ()> = io::Result<T>;

pub struct CssWriter<'a> {
    buf: &'a mut (dyn Write + 'a),
}
impl<'a> CssWriter<'a> {
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
impl Write for CssWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf.flush()
    }
}

pub trait WriteValue {
    fn write_value(&self, f: &mut CssWriter) -> WriteResult;
}

pub fn render_to_vec(value: impl WriteValue) -> WriteResult<Vec<u8>> {
    let mut v = Vec::new();
    value.write_value(&mut CssWriter::new(&mut v))?;
    Ok(v)
}

pub fn render_to_string(value: impl WriteValue) -> WriteResult<String> {
    String::from_utf8(render_to_vec(value)?).map_err(|_| io::ErrorKind::InvalidData.into())
}

pub unsafe fn render_to_string_unchecked(value: impl WriteValue) -> WriteResult<String> {
    let v = render_to_vec(value)?;
    Ok(String::from_utf8_unchecked(v))
}

pub trait WriteDeclaration: WriteValue {
    fn write_property(&self, f: &mut CssWriter) -> WriteResult;

    fn write_declaration(&self, f: &mut CssWriter) -> WriteResult {
        self.write_property(f)?;
        f.write_char(':')?;
        self.write_value(f)
    }
}

impl<T> WriteValue for Box<T>
where
    T: WriteValue,
{
    fn write_value(&self, f: &mut CssWriter) -> WriteResult {
        self.as_ref().write_value(f)
    }
}
