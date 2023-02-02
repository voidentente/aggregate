use crate::{Amalgamate, Attributes, Fields};
use quote::ToTokens;

impl std::fmt::Debug for Amalgamate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        debug_amalgamate(&mut buf, self, 0);
        f.write_str(&buf)
    }
}

const INDENTATION_GROWTH: u8 = 4;

fn debug_amalgamate(buf: &mut String, amalgamate: &Amalgamate, indentation: u8) {
    debug_attrs(buf, &amalgamate.attrs, indentation);
    indent(buf, indentation);
    buf.push_str("amalgamate:\n");
    debug_fields(buf, &amalgamate.fields, indentation + INDENTATION_GROWTH);
}

fn debug_fields(buf: &mut String, fields: &Fields, indentation: u8) {
    for (ident, field) in fields {
        debug_attrs(buf, &field.attrs, indentation);
        indent(buf, indentation);
        buf.push_str(ident);
        buf.push_str(":\n");
        if let Some(amalgamate) = field.inner {
            debug_amalgamate(buf, amalgamate, indentation + INDENTATION_GROWTH);
        }
    }
}

fn debug_attrs(buf: &mut String, attrs: &Attributes, indentation: u8) {
    for (_, attr) in attrs {
        indent(buf, indentation);
        buf.push_str(&attr.to_token_stream().to_string());
        buf.push('\n');
    }
}

fn indent(buf: &mut String, indentation: u8) {
    for _ in 0..indentation {
        buf.push(' ');
    }
}
