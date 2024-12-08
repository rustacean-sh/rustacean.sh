use axum::response::{IntoResponse, Response};
use axum::Json;

use proto::Rustacean;

pub async fn handler() -> Response {
    let objects = vec![
        Rustacean {
            name: "Esteban Borai".to_string(),
            image: Some("https://avatars.githubusercontent.com/u/34756077?v=4".into()),
            gh_user: "https://github.com/EstebanBorai".into(),
            social_networks: None,
            location: None,
        },
        Rustacean {
            name: "Luciano Ramello".to_string(),
            image: Some("https://avatars.githubusercontent.com/u/20213274?v=4".into()),
            gh_user: "https://github.com/luchosr".into(),
            social_networks: None,
            location: None,
        },
    ];

    Json(objects).into_response()
}
