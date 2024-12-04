use axum::response::{IntoResponse, Response};
use axum::Json;

use proto::Rustacean;

pub async fn handler() -> Response {
    let objects = vec![
        Rustacean {
            name: "Esteban Borai".to_string(),
            image: "https://avatars.githubusercontent.com/u/34756077?v=4".into(),
            github_url: "https://github.com/EstebanBorai".into(),
        },
        Rustacean {
            name: "Luciano Ramello".to_string(),
            image: "https://avatars.githubusercontent.com/u/20213274?v=4".into(),
            github_url: "https://github.com/luchosr".into(),
        },
    ];

    Json(objects).into_response()
}
