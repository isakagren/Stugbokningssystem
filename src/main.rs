use auth::Validator;
use models::*;
use endpoints::*;
use salvo::basic_auth::BasicAuth;
use salvo::logging::Logger;
use salvo::prelude::*;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

mod models;
mod endpoints;
mod auth;


#[tokio::main]
async fn main() {
    // Setup tracing
    tracing_subscriber::fmt().init();

    // Setup DB
    let db: Surreal<surrealdb::engine::local::Db> = Surreal::new::<Mem>(()).await.unwrap();    // Create in-memory database
    db.use_ns("test").use_db("test").await.unwrap();
    DB_POOL.set(db).unwrap();

    // Populate the DB with data
    let db = db_pool();
    populate_db_at_startup(db).await;
    
    // Start server with the router below
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(route()).await;
}

// returns a router that handles all the incoming requests
fn route() -> Router {
    // Authentication middlewre
    let auth_handler = BasicAuth::new(Validator);

    // Set up the routes for the application
    let router = Router::new()
        .hoop(Logger::new())
        .get(home)
        .push(Router::with_path("app")
            .push(Router::with_hoop(auth_handler)
            .push(Router::with_path("cabins").get(get_cabins))
            .push(Router::with_path("book/<cabin>").get(book_cabin))
                .push(Router::with_path("<**path>")
                    .get(
                        StaticDir::new(["static"])
                        .defaults("index.html")
                        .auto_list(true)
                    )
                )
            )
        );

    // Create a OpenAPI specification from the router above 
    let doc = OpenApi::new("Stugbokningens API", "0.0.1").merge_router(&router);

    // Add endpoints for the swagger-ui page into the router 
    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));

    // return the router
    router
}


