use salvo::{writing::{Json, Text}, Depot, Response, oapi::endpoint, handler, Request, hyper::StatusCode};

use crate::{models::Cabin, db_pool};


/// The homepage of the application
#[handler]
pub async fn home(res: &mut Response) {
    res.render(Text::Html("<html><head></head><body>Welcome to cabin booking. <a href='/app'> Please sign in at this link.</a> or <a href='/swagger-ui'>view the OpenAPI spec here </a></body></html>"));
}

/// Returns a list of all cabins the user is allowd to see
#[endpoint]
pub async fn get_cabins(depot: &mut Depot) -> Json<Vec<Cabin>> {
    let db = db_pool(); // Get database
    match depot.get::<bool>("admin") {  // Check if current user is admin
        Ok(admin) => {
            match admin {
                true => {
                    let res: Vec<Cabin> = db.select("cabin").await.unwrap();    // If admin, return all cabins 
                    Json(res)
                },
                false => {  // if not admin, return all free cabins as well as the cabin booked by the user 
                    match depot.get::<String>("username") {
                        Ok(username) => {
                            let res: Vec<Cabin> = db.select("cabin")
                                .await.unwrap();
                            let res = res.into_iter()
                                .filter(|c: &Cabin| match c.booked.clone() {
                                    Some(name) => name == *username,
                                    None => true,
                                }).collect();
                            Json(res)

                        },
                        Err(_) => Json(Vec::new()),
                    }
                },
            }
        }
        Err(_) => Json(Vec::new())
    }
}

/// Use this to book a cabin when signed in
#[endpoint]
pub async fn book_cabin(req: &mut Request, depot: &mut Depot) -> StatusCode {
    let db = db_pool();

    match depot.get::<String>("username") {
        Ok(username) => {
            let cabin = req.param::<String>("cabin").unwrap();

            let updated: Option<surrealdb::Response> = db.query("UPDATE ONLY cabin SET booked = $username WHERE name = $cabinname")
                .bind(("username", username))
                .bind(("cabinname", cabin))
                .await.ok();

            match updated {
                Some(_) =>  StatusCode::OK,
                None => StatusCode::BAD_REQUEST,
            }
        },
        Err(_) => StatusCode::FORBIDDEN,
    }

}
