use serde::Deserialize;
use actix_web::{web, App, HttpResponse, HttpServer};

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        let r = n % m;
        n = m;
        m = r;
    }
    n
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit">Compute GCD</button>
                </form>
            "#
        )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html; charset=utf-8")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m),
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving on http://127.0.0.1:3000...");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
