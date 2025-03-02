use super::{
    id::{LogId, UserId},
    Maidenhead,
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

#[derive(Debug)]
pub struct ParkCode(String);
impl ParkCode {
    pub fn new(code: String) -> Self {
        Self(code)
    }
    pub fn inner_ref(&self) -> &String {
        &self.0
    }
}

#[derive(Debug)]
pub struct POTAReference {
    pub pota_code: String,
    pub wwff_code: String,
    pub park_name: String,
    pub park_name_j: String,
    pub park_location: String,
    pub park_locid: String,
    pub park_type: String,
    pub park_inactive: bool,
    pub park_area: i32,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub maidenhead: Maidenhead,
    pub update: DateTime<Utc>,
}

#[derive(Debug)]
pub struct POTAReferenceWithLog {
    pub pota_code: String,
    pub wwff_code: String,
    pub park_name: String,
    pub park_name_j: String,
    pub park_location: String,
    pub park_locid: String,
    pub park_type: String,
    pub park_inactive: bool,
    pub park_area: i32,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub maidenhead: Maidenhead,
    pub attempts: Option<i32>,
    pub activations: Option<i32>,
    pub first_qso_date: Option<NaiveDate>,
    pub qsos: Option<i32>,
}

#[derive(Debug)]
pub struct POTAActivatorLog {
    pub log_id: LogId,
    pub dx_entity: String,
    pub location: String,
    pub hasc: String,
    pub pota_code: String,
    pub park_name: String,
    pub first_qso_date: NaiveDate,
    pub attempts: i32,
    pub activations: i32,
    pub qsos: i32,
}

#[derive(Debug)]
pub struct POTAHunterLog {
    pub log_id: LogId,
    pub dx_entity: String,
    pub location: String,
    pub hasc: String,
    pub pota_code: String,
    pub park_name: String,
    pub first_qso_date: NaiveDate,
    pub qsos: i32,
}

#[derive(Debug, Clone)]
pub enum POTALogKind {
    ActivatorLog,
    HunterLog,
}

impl From<POTALogKind> for String {
    fn from(kind: POTALogKind) -> Self {
        match kind {
            POTALogKind::ActivatorLog => "activator".to_string(),
            POTALogKind::HunterLog => "hunter".to_string(),
        }
    }
}

impl From<String> for POTALogKind {
    fn from(kind: String) -> Self {
        match kind.as_str() {
            "activator" => POTALogKind::ActivatorLog,
            "hunter" => POTALogKind::HunterLog,
            _ => panic!("Invalid POTALogKind"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct POTALogUser {
    pub user_id: Option<UserId>,
    pub log_id: LogId,
    pub log_kind: Option<POTALogKind>,
    pub update: NaiveDateTime,
}

impl POTALogUser {
    pub fn new(user_id: Option<UserId>) -> Self {
        let update = Utc::now().naive_utc();
        Self {
            user_id,
            log_id: LogId::new(),
            log_kind: None,
            update,
        }
    }
}
