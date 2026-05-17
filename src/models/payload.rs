use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct IdPayload {
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct OnboardingPayload {
    pub name: String,
    pub email: String,
    pub role: String,
    pub start_date: String,
}

#[derive(Deserialize, Debug)]
pub struct OdooUser {
    pub id: i32,
    pub name: String,
    pub work_email: String,
}

#[derive(Serialize)]
pub struct Claims {
    pub iss: String,
    pub scope: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
}   