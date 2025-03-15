#[cfg(test)]
mod test{
    use rocket::http::{Status};
    use rocket::local::asynchronous::Client;
    use crate::rocket;

    #[rocket::async_test]
    async fn create_user(){
        let client:Client = Client::tracked(rocket()).await.expect("valid rocket instance");
        let mut response = client
            .post("/users")
            .body(r#"{"username":"user1","email":"mm@mail.com","password":"pass"}"#)
            .dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), r#"{"id":null,"username":"user1","email":"mm@mail.com","password":"pass"}"#);
    }
}