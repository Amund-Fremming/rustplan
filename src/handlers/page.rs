use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use uuid::Uuid;

pub async fn index(Path(id): Path<Uuid>) -> impl IntoResponse {
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <link rel="stylesheet" href="/static/participant.css">
            <script type="module" src="/static/participant.js" defer></script>
        </head>
        <body>
            <dialog id="modal">
                <h2>Name</h2>
                <div id="modal-div">
                    <input id="name-input" />
                    <button id="modal-button">Join</button>
                </div>
            </dialog>
            <div id="group-id" hidden>{}</div>
            <h1 id="header">Rust</h1>
            <button id="copy">Copy link</button>
        </body>
        </html>
        "#,
        id
    );

    Html(html)
}

pub async fn create() -> impl IntoResponse {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <link rel="stylesheet" href="/static/host.css">
            <script type="module" src="/static/host.js" defer></script>
        </head>
        <body>
            <h1>Rust</h1>
            <div>
                <label>Group name</label>
                <input id="group-name" />
                <label>Group description</label>
                <input id="group-desc" />
                <label>Year</label>
                <input id="group-year" />
                <button id="create">Create</button>
            </div>
        </body>
        </html>
    "#;

    Html(html)
}
