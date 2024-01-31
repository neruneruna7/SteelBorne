use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder, Result,
};
use askama::Template;

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeTemplate<'a> {
    pub name: &'a str,
}

impl<'a> WelcomeTemplate<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[get("/")]
pub async fn trial_askama(req: HttpRequest) -> Result<impl Responder> {
    let welcome = WelcomeTemplate::new("neruneruna7");
    let renderd = welcome.render().unwrap();

    Ok(HttpResponse::Ok().body(renderd))
}

#[derive(Template)]
#[template(path = "list.html")]
pub struct ListTemplate {
    items: Vec<String>,
}

impl ListTemplate {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }
}

#[get("/list")]
pub async fn trial_askama_list(req: HttpRequest) -> Result<impl Responder> {
    let items = vec![
        "item1".to_string(),
        "item2".to_string(),
        "item3".to_string(),
    ];
    let list_template = ListTemplate::new(items);
    let renderd = list_template.render().unwrap();

    Ok(HttpResponse::Ok().body(renderd))
}
