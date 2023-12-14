use salvo::{async_trait, basic_auth::BasicAuthValidator, Depot};

use crate::models::{User, db_pool};


// Setup for BasicAuth middleware
pub struct Validator;
#[async_trait]
impl BasicAuthValidator for Validator {
    async fn validate(&self, username: &str, password: &str, depot: &mut Depot) -> bool {

        let db = db_pool(); // Get database connection

        depot.insert("username", username.to_string());         // Save the username of the person signing in

        let res: Vec<User> = db.select("user").await.unwrap();  // Get users from DB
        let res: Vec<User> = res.into_iter()
            .filter(|u| u.name == username && u.password == password)   // Uppenbart inte säkert, simple lösning för demo
            .collect();
        let user = res.first();
        
        match user {
            Some(u) => {
                depot.insert("admin", u.admin);
                true
            },
            None => false
        }
    }
}
