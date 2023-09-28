use axum::response::Html;

pub mod routes;
pub mod startup;
pub mod config;
pub mod error;



async fn root() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <script src="https://cdn.tailwindcss.com"></script>
                <title>Document</title>
            </head>
            <body class="bg-black">
                <div class="container mx-auto px-4">

                <h1 class="text-3xl font-bold bg-white">Hello World!</h1>

                </div>
    
            </body>
        </html>
    "#)
}