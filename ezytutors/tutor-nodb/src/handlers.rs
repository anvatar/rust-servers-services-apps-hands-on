use super::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    // visit_count is a MutexGuard, which is an RAII implementation of a "scoped lock" of a mutex.
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1; // DerefMut is overridden for MutexGuard
    HttpResponse::Ok().json(&response) // "&" can be omitted because response is no more used here
}
