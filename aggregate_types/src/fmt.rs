use crate::Amalgamate;
use quote::ToTokens;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Debug for Amalgamate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        debug(&mut buf, self, 0);
        write!(f, "{buf}")
    }
}

fn debug(buf: &mut String, amalgamate: &Amalgamate, mut indentation: u8) {
    const INDENTATION_GROWTH: u8 = 4;

    for (_, attr) in &amalgamate.attrs {
        let attr = attr.to_token_stream().to_string();
        indent(buf, indentation);
        buf.push_str(&format!("{attr}\n"));
    }

    indent(buf, indentation);
    buf.push_str("amalgamate:\n");
    indentation += INDENTATION_GROWTH;

    for (ident, field) in &amalgamate.fields {
        for (_, attr) in &field.attrs {
            let attr = attr.to_token_stream().to_string();
            indent(buf, indentation);
            buf.push_str(&format!("{attr}\n"));
        }
        indent(buf, indentation);
        buf.push_str(&format!("{ident}:\n"));
        if let Some(inner) = field.inner {
            debug(buf, inner, indentation + INDENTATION_GROWTH);
        }
    }
}

fn indent(buf: &mut String, indentation: u8) {
    for _ in 0..indentation {
        buf.push(' ');
    }
}
