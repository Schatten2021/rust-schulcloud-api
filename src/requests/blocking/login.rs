use crate::errors::Result;
use crate::request_types::login::*;
use crate::requests::blocking::post_request;
use crate::state::State;

pub fn email_password_login(state: &mut State, email: impl ToString, password: impl ToString, app_name: impl ToString) -> Result<()> {
    let body = EmailPasswordLogin::new(email.to_string(), password.to_string(), state.device_id.to_string(), app_name.to_string(), false, false, false);
    let response = post_request::<LoginSuccessResponse>(state, "/auth/login", body)?;
    state.client_key = Some(response.client_key);
    Ok(())
}