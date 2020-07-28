use std::collections::HashMap;
use crate::smtp;
use crate::my_trait::StatusTrait;
use crate::status::user::active::{ActiveStatus, _ActiveStatus};
use crate::status::db_api::{DbAPIStatus, _DbAPIStatus};
use crate::user::tools::check_response;
use crate::smtp::EmailType;

#[derive(Serialize, Deserialize)]
pub struct ActiveCode {
    code: String,
    owner: String,
}

impl ActiveCode {
    pub fn new(code: String, owner: String) -> ActiveCode {
        ActiveCode {
            code,
            owner,
        }
    }

    pub fn to_db_and_email(&self, email: &str) -> Result<ActiveStatus, ActiveStatus> {
        if let Err(_) = smtp::check_email(&email) {
            return Err(ActiveStatus::default().set_status(_ActiveStatus::InvalidEmailAddress));
        }
        if let Err(_) = smtp::send_email(email, EmailType::Active, &self.code) {
            return Err(ActiveStatus::default().set_status(_ActiveStatus::SendEmailError));
        }
        match create(self) {
            Ok(()) => Ok(ActiveStatus::default()),
            Err(e) => Err(ActiveStatus::set_db_api_err_simple(e))
        }
    }

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn owner(&self) -> String {
        self.owner.clone()
    }
}

pub fn read() -> Result<Vec<ActiveCode>, DbAPIStatus> {
    match reqwest::blocking::get("http://localhost:1122/api/user/active_code/read") {
        Ok(response) => {
            match response.json::<Vec<ActiveCode>>() {
                Ok(active_code) => Ok(active_code),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn create(code: &ActiveCode) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    match client.post("http://localhost:1122/api/user/active_code/create").json(code).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}

pub fn delete(active_code: &ActiveCode) -> Result<(), DbAPIStatus> {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let uri = format!("http://localhost:1122/api/user/active_code/delete/{}", active_code.code());
    match client.delete(&uri).json(active_code).send() {
        Ok(response) => {
            match response.json::<HashMap<String, String>>() {
                Ok(status) => check_response(status),
                Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::DataError, e.to_string()))
            }
        }
        Err(e) => Err(DbAPIStatus::new(_DbAPIStatus::ConnectRefused, e.to_string()))
    }
}