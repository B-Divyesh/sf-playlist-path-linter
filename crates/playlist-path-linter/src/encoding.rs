use crate::LintError;
use encoding_rs::{Encoding, UTF_16BE, UTF_16LE, UTF_8, WINDOWS_1252};
use std::borrow::Cow;

pub(crate) struct PlaylistText {
    pub text: String,
    encoding: &'static Encoding,
    bom: &'static [u8],
}

impl PlaylistText {
    pub fn decode(bytes: &[u8], strict_utf8: bool) -> Result<Self, LintError> {
        let (encoding, bom, content) = if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
            (UTF_8, &[0xef, 0xbb, 0xbf][..], &bytes[3..])
        } else if bytes.starts_with(&[0xff, 0xfe]) {
            (UTF_16LE, &[0xff, 0xfe][..], &bytes[2..])
        } else if bytes.starts_with(&[0xfe, 0xff]) {
            (UTF_16BE, &[0xfe, 0xff][..], &bytes[2..])
        } else if std::str::from_utf8(bytes).is_ok() {
            (UTF_8, &[][..], bytes)
        } else if strict_utf8 {
            return Err(LintError(
                "M3U8 must contain valid UTF-8 (a UTF-8 BOM is allowed)".into(),
            ));
        } else {
            (WINDOWS_1252, &[][..], bytes)
        };
        let (decoded, _, had_errors) = encoding.decode(content);
        if had_errors {
            return Err(LintError(format!(
                "playlist could not be decoded as {}",
                encoding.name()
            )));
        }
        Ok(Self {
            text: decoded.into_owned(),
            encoding,
            bom,
        })
    }

    pub fn encode(&self, text: &str) -> Result<Vec<u8>, LintError> {
        if self.encoding == UTF_16LE || self.encoding == UTF_16BE {
            let mut output = Vec::with_capacity(self.bom.len() + text.len() * 2);
            output.extend_from_slice(self.bom);
            for unit in text.encode_utf16() {
                let bytes = if self.encoding == UTF_16LE {
                    unit.to_le_bytes()
                } else {
                    unit.to_be_bytes()
                };
                output.extend_from_slice(&bytes);
            }
            return Ok(output);
        }
        let (encoded, _, had_errors) = self.encoding.encode(text);
        if had_errors {
            return Err(LintError(format!("a corrected path is not representable in the source {} encoding; use an M3U8 source", self.encoding.name())));
        }
        let mut output = Vec::with_capacity(self.bom.len() + encoded.len());
        output.extend_from_slice(self.bom);
        match encoded {
            Cow::Borrowed(bytes) => output.extend_from_slice(bytes),
            Cow::Owned(bytes) => output.extend(bytes),
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retains_utf16_bom() {
        let input = [0xff, 0xfe, b'A', 0, b'\n', 0];
        let text = PlaylistText::decode(&input, false).unwrap();
        assert_eq!(text.text, "A\n");
        assert_eq!(text.encode(&text.text).unwrap(), input);
    }

    #[test]
    fn m3u8_rejects_legacy_bytes() {
        assert!(PlaylistText::decode(&[0x80], true).is_err());
    }
}
