use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Selectable, Insertable, Debug, Serialize, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::employee_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub employee_id: i32,
    pub employee_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDateTime>,
    pub hire_date: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub department_id: Option<i32>,
    pub job_title: Option<String>,
    pub salary: Option<i32>,
    pub manager_id: Option<i32>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub valid: Option<bool>,
    pub last_update: Option<chrono::NaiveDateTime>,
}

impl Employee {
    pub fn new_empty(name: &'static str) -> Self {
        Employee {
            employee_id: 0,
            employee_name: name.to_string(),
            first_name: None,
            last_name: None,
            gender: None,
            date_of_birth: None,
            hire_date: None,
            email: None,
            phone_number: None,
            department_id: None,
            job_title: None,
            salary: None,
            manager_id: None,
            address: None,
            city: None,
            state: None,
            postal_code: None,
            valid: None,
            last_update: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::employee_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEmployee {
    pub employee_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDateTime>,
    pub hire_date: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub department_id: Option<i32>,
    pub job_title: Option<String>,
    pub salary: Option<i32>,
    pub manager_id: Option<i32>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub valid: Option<bool>,
}

impl NewEmployee {
    pub fn new_empty(name: &'static str) -> Self {
        NewEmployee {
            employee_name: name.to_string(),
            first_name: None,
            last_name: None,
            gender: None,
            date_of_birth: None,
            hire_date: None,
            email: None,
            phone_number: None,
            department_id: None,
            job_title: None,
            salary: None,
            manager_id: None,
            address: None,
            city: None,
            state: None,
            postal_code: None,
            valid: None,
        }
    }
}
