use actix_web::{get, web, HttpResponse};
use handlebars::Handlebars;

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
	let data = json!({
		"name": "HandlebarsUser"
	});
	let body = hb.render("home", &data).unwrap();

	HttpResponse::Ok().body(body)
}
