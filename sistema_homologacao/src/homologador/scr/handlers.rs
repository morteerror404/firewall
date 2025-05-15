use actix_web::{
    get, post, 
    web, HttpResponse, 
    Responder
};
use askama::Template;
use crate::models::{HomologationRequest, HomologationResult};

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    results: Vec<HomologationResult>,
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../static/index.html"))
}

#[post("/submit")]
pub async fn submit_homologation(
    data: web::Json<HomologationRequest>,
    state: web::Data<std::sync::Mutex<models::HomologationData>>
) -> impl Responder {
    let result = models::process_homologation(data.into_inner());
    
    let mut db = state.lock().unwrap();
    db.add_result(result.clone());
    
    HttpResponse::Ok().json(result)
}

#[get("/results")]
pub async fn get_results(
    state: web::Data<std::sync::Mutex<models::HomologationData>>
) -> impl Responder {
    let db = state.lock().unwrap();
    HttpResponse::Ok().json(db.get_all())
}

#[get("/report")]
pub async fn get_report(
    state: web::Data<std::sync::Mutex<models::HomologationData>>
) -> impl Responder {
    let db = state.lock().unwrap();
    let template = DashboardTemplate {
        results: db.get_all(),
    };
    
    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}