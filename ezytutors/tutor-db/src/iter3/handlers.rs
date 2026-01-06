use super::models::Course;
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

pub async fn get_courses_for_tutor(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32,)>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

pub async fn get_course_details(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32, i32)>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

pub async fn post_new_course(
    _new_course: web::Json<Course>,
    _app_state: web::Data<AppState>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::web;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_courses_for_tutor_success() {
        let app_state = make_app_state().await;
        let tutor_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    async fn make_app_state() -> web::Data<AppState> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::connect(&database_url).await.unwrap();
        web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        })
    }

    #[actix_rt::test]
    async fn get_course_detail_success() {
        let app_state = make_app_state().await;
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_success() {
        let app_state = make_app_state().await;
        let course_param = web::Json(Course {
            tutor_id: 1,
            course_id: 1,
            course_name: "This is the next course".into(),
            posted_time: NaiveDate::from_ymd_opt(2020, 9, 17)
                .unwrap()
                .and_hms_opt(14, 1, 11),
        });
        let resp = post_new_course(course_param, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
