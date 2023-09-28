//use axum::response::Html;
use axum::response::Html;


pub async fn health_check() -> Html<&'static str> {
    
    Html(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Health Check</title>
        </head>
        <body>
            <h1>Everything is okay!</h1>
        </body>
        </html>
    "#)  
}

