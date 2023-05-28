use crate::matches::{BoyerMooreSearcher, Match};
use actix_web::{
    body::BoxBody, http::header::ContentType, post, web::Json, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

/// The `source` information for for the search using the Boyer-More search.
#[derive(Serialize, Deserialize)]
pub struct Source {
    pub text: String,
    pub pattern: String,
}

impl Responder for Match {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

/// Returns the number of matches of a given `pattern` found against a specified
/// `source text`. See [`BoyerMooreSearcher`].
#[post("/")]
pub async fn matches_ctrl(req: Json<Source>) -> impl Responder {
    let sr = BoyerMooreSearcher::new(req.into_inner());
    sr.search().unwrap_or_default()
}
