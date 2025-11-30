use super::state::AppState;
use crate::models::Course;
use actix_web::{HttpResponse, web};
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    // visit_count is a MutexGuard, which is an RAII implementation of a "scoped lock" of a mutex.
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1; // DerefMut is overridden for MutexGuard
    HttpResponse::Ok().json(&response) // "&" can be omitted because response is no more used here
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count();
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some((course_count_for_user + 1) as i32),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added course")
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    let tutor_id = params.0;

    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == tutor_id)
        .collect::<Vec<Course>>();

    if filtered_courses.is_empty() {
        // .to_string() is not needed
        HttpResponse::NoContent().json("No courses found for tutor".to_string())
    } else {
        HttpResponse::Ok().json(filtered_courses)
    }
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = (params.0, params.1);
    let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|course| course.tutor_id == tutor_id && course.course_id == Some(course_id))
        .ok_or("Course not found");

    match selected_course {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(err) => {
            // .to_string() is not needed
            HttpResponse::NotFound().json(err.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(make_test_course(None));
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    fn make_test_course(course_id: Option<i32>) -> Course {
        Course {
            tutor_id: 1,
            course_name: "Hello, this is test course".into(),
            course_id: course_id,
            posted_time: None,
        }
    }

    #[actix_rt::test]
    async fn get_courses_for_tutor_no_content() {
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let tutor_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[actix_rt::test]
    async fn get_courses_for_tutor_success() {
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![make_test_course(Some(1))]),
        });
        let tutor_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_not_found() {
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn get_course_detail_success() {
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![make_test_course(Some(1))]),
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
