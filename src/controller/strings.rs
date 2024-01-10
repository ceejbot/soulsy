//! Character encoding shenanigans. Bethesda is very bad at utf-8, I am told.
use cxx::CxxVector;
use encoding::label::encoding_from_whatwg_label;
use encoding::DecoderTrap;

// To test in game: install daegon
// player.additem xxxb15f4 1
// Sacrÿfev Tëliimi

/// C++ should use this for std::string conversions.
pub fn string_to_utf8(bytes_ffi: &CxxVector<u8>) -> String {
    let bytes: Vec<u8> = bytes_ffi.iter().copied().collect();
    convert_to_utf8(bytes)
}

/// Use this for null-terminated C strings.
pub fn cstr_to_utf8(bytes_ffi: &CxxVector<u8>) -> String {
    let bytes: Vec<u8> = bytes_ffi.iter().copied().collect();
    let bytes = if bytes.ends_with(&[0]) {
        let chopped = bytes.len() - 1;
        let mut tmp = bytes.clone();
        tmp.truncate(chopped);
        tmp
    } else {
        bytes
    };
    convert_to_utf8(bytes)
}

/// Get a valid Rust representation of this Windows codepage string data by hook or by crook.
pub fn convert_to_utf8(bytes: Vec<u8>) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    let (encoding, _confidence, _language) = chardet::detect(&bytes);
    if let Some(coder) = encoding_from_whatwg_label(chardet::charset2encoding(&encoding)) {
        if let Ok(utf8string) = coder.decode(&bytes, DecoderTrap::Replace) {
            return utf8string.to_string();
        }
    }

    String::from_utf8_lossy(&bytes).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_data_is_untouched() {
        let example = "Sacrÿfev Tëliimi";
        let converted = convert_to_utf8(example.as_bytes().to_vec());
        assert_eq!(converted, example);
        let ex2 = "おはよう";
        let convert2 = convert_to_utf8(ex2.as_bytes().to_vec());
        assert_eq!(convert2, ex2);
        let ex3 = "Zażółć gęślą jaźń";
        let convert3 = convert_to_utf8(ex3.as_bytes().to_vec());
        assert_eq!(convert3, ex3);
    }

    #[test]
    fn iso8859_is_decoded() {
        // This is the example above (from the Daegon mod), in its expression
        // as windows codepage bytes. This test is the equivalent of me testing
        // that the textcode mod works, but I am feeling timid.
        let bytes: Vec<u8> = vec![
            0x53, 0x61, 0x63, 0x72, 0xff, 0x66, 0x65, 0x76, 0x20, 0x54, 0xeb, 0x6c, 0x69, 0x69,
            0x6d, 0x69,
        ];
        assert!(String::from_utf8(bytes.clone()).is_err());
        let utf8_version = "Sacrÿfev Tëliimi".to_string();
        let converted = convert_to_utf8(bytes.clone());
        assert_eq!(converted, utf8_version);
    }
}
