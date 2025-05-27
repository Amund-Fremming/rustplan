use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};

pub async fn create_middleware(mut request: Request, next: Next) -> Response {
    // Før: Gjør noe med request
    println!("Før next: legg til header");
    // Legger til header i request (krever at request er mutable)
    request
        .headers_mut()
        .insert("x-my-header", HeaderValue::from_static("verdien"));

    // Send request videre til neste handler/middleware
    let mut response = next.run(request).await;

    // Etter: Gjør noe med response
    println!("Etter next: legg til header i response");
    response
        .headers_mut()
        .insert("x-middleware", HeaderValue::from_static("mitt-middleware"));

    response
}
