use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetParam {
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub min_lon: Option<f64>,
    pub min_lat: Option<f64>,
    pub max_lon: Option<f64>,
    pub max_lat: Option<f64>,
    pub min_elev: Option<i32>,
    pub min_area: Option<i32>,
    pub max_count: Option<u32>,
    pub ref_id: Option<String>,
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub after: Option<i64>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub muni_code: Option<i32>,
    pub distinct_call: Option<bool>,
    pub distinct_ref: Option<bool>,
}
