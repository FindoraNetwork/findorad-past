use bip0039::{Count, Language, Mnemonic};

use crate::{Error, Result};

pub fn generate_mnemonic_custom(wordslen: u8, lang: &str) -> Result<String> {
    let w = match wordslen {
        12 => Count::Words12,
        15 => Count::Words15,
        18 => Count::Words18,
        21 => Count::Words21,
        24 => Count::Words24,
        _ => {
            return Err(Error::MnemonicFormatError);
        }
    };

    let l = check_lang(lang)?;

    Ok(Mnemonic::generate_in(l, w).into_phrase())
}

pub fn check_lang(lang: &str) -> Result<Language> {
    match lang {
        "en" => Ok(Language::English),
        "zh" => Ok(Language::SimplifiedChinese),
        "zh_traditional" => Ok(Language::TraditionalChinese),
        "fr" => Ok(Language::French),
        "it" => Ok(Language::Italian),
        "ko" => Ok(Language::Korean),
        "sp" => Ok(Language::Spanish),
        "jp" => Ok(Language::Japanese),
        _ => Err(Error::UnsupportMnemonicLanguage),
    }
}
