use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;

#[derive(serde::Deserialize)]
struct ScrambleParams {
    length: Option<usize>,
}

#[get("/api/scramble")]
async fn scramble(query: web::Query<ScrambleParams>) -> impl Responder {
    let len = query.length.unwrap_or(25).to_string();
    let bin = std::env::var("COACH_BIN").unwrap_or_else(|_| "/target/release/3bld-coach".into());
    let output = Command::new(&bin)
        .arg("scramble")
        .arg("--length")
        .arg(&len)
        .output();
    match output {
        Ok(o) if o.status.success() => {
            let txt = String::from_utf8_lossy(&o.stdout).trim().to_string();
            HttpResponse::Ok().json(serde_json::json!({"scramble": txt}))
        }
        Ok(o) => {
            let err = String::from_utf8_lossy(&o.stderr).into_owned();
            HttpResponse::InternalServerError().body(err)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/api/memo")]
async fn memo(body: web::Json<serde_json::Value>) -> impl Responder {
    // placeholder: just echo request
    HttpResponse::Ok().json(body.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(scramble)
            .service(memo)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
