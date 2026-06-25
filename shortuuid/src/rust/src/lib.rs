use anyhow::anyhow;
use extendr_api::prelude::*;
use short_uuid::{CustomAlphabet, CustomTranslator, ShortUuid, ShortUuidCustom};

#[extendr]
fn short_uuid(n: usize) -> Strings {
    (0..n)
        .into_iter()
        .map(|_| ShortUuid::generate().to_string())
        .collect()
}

#[extendr]
fn is_short_uuid(uuid: Strings) -> Logicals {
    uuid.into_iter()
        .map(|v| {
            if v.is_na() {
                Rbool::na()
            } else {
                match ShortUuid::from_uuid_str(v) {
                    Ok(_) => Rbool::from(true),
                    Err(_) => Rbool::from(false),
                }
            }
        })
        .collect()
}

#[extendr]
fn as_uuid(uuid: Strings) -> Strings {
    uuid.into_iter()
        .map(|v| {
            if v.is_na() {
                return Rstr::na();
            }

            match ShortUuid::from_uuid_str(v) {
                Ok(suid) => suid.to_uuid().to_string().into(),
                Err(_) => Rstr::na(),
            }
        })
        .collect()
}

#[extendr]
pub struct CustomUuidTranslator {
    inner: CustomTranslator,
}

#[extendr]
impl CustomUuidTranslator {
    fn new(alphabet: &'static str) -> anyhow::Result<Self> {
        let inner = CustomTranslator::new(alphabet)
            .map_err(|e| anyhow!("Failed to create custom translator with error: {e}"))?;
        Ok(Self { inner })
    }

    fn generate(&self, n: usize) -> Strings {
        (0..n)
            .into_iter()
            .map(|_| ShortUuidCustom::generate(&self.inner).to_string())
            .collect()
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod shortuuid;
    fn is_short_uuid;
    fn as_uuid;
    fn short_uuid;
    impl CustomUuidTranslator;
}
