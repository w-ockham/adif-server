use domain::model::locator::{CenturyCode, MunicipalityCenturyCode};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CenturyCodeResponse {
    pub muni_code: i32,
    pub prefecture: String,
    pub municipality: String,
    pub jcc_code: Option<String>,
    pub ward_code: Option<String>,
    pub jcc_text: Option<String>,
    pub jcg_code: Option<String>,
    pub jcg_text: Option<String>,
    pub hamlog_code: Option<String>,
    pub maidenhead: Option<String>,
}

impl From<MunicipalityCenturyCode> for CenturyCodeResponse {
    fn from(mcc: MunicipalityCenturyCode) -> CenturyCodeResponse {
        let MunicipalityCenturyCode {
            muni_code,
            prefecture,
            municipality,
            code,
        } = mcc;
        match code {
            CenturyCode::JCC {
                jcc_code,
                ward_code,
                jcc_text,
            } => CenturyCodeResponse {
                muni_code,
                prefecture,
                municipality,
                jcc_code: Some(jcc_code),
                ward_code,
                jcc_text: Some(jcc_text),
                jcg_code: None,
                jcg_text: None,
                hamlog_code: None,
                ..Default::default()
            },
            CenturyCode::JCG {
                jcg_code,
                jcg_text,
                hamlog_code,
            } => CenturyCodeResponse {
                muni_code,
                prefecture,
                municipality,
                jcc_code: None,
                ward_code: None,
                jcc_text: None,
                jcg_code: Some(jcg_code),
                jcg_text: Some(jcg_text),
                hamlog_code,
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapcodeResponse {
    pub mapcode: String,
}

impl From<String> for MapcodeResponse {
    fn from(mapcode: String) -> Self {
        Self { mapcode }
    }
}
