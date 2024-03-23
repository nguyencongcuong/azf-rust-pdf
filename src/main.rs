use std::env;

use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, middleware::Logger, web};
use printpdf::Error;

use handler::{Options, print_template_1};

use crate::util::calculate_time;

mod util;

#[get("/api/pdf")]
pub async fn create_pdf(_req: HttpRequest, options: web::Json<Options>) -> HttpResponse {
    let pdf_options = options.into_inner();
    let mut pdf_file: Result<Vec<u8>, Error> = Ok(Vec::new());
    let Options { title, template, .. } = &pdf_options;

    match template.as_str() {
        "1" => pdf_file = calculate_time("PDF Generation", || print_template_1(&pdf_options)),
        _ => {
            return HttpResponse::BadRequest()
                .body(format!("Template {} is not supported at this moment.", template));
        }
    }

    match pdf_file {
        Ok(buffer) => {
            HttpResponse::Ok()
                .content_type("application/pdf")
                .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", &title)))
                .body(buffer)
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(create_pdf)
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}