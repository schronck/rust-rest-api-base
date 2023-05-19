use actix_web::{get, post, web, Responder};

#[tracing::instrument(fields(result))]
#[get("/addTwo/{a}/{b}")]
pub async fn add_two(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();

    web::Json(a + b)
}

#[tracing::instrument(fields(result))]
#[post("/sum")]
pub async fn sum(body: web::Json<Vec<i32>>) -> impl Responder {
    web::Json(body.iter().sum::<i32>())
}
