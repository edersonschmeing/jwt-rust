//https://www.youtube.com/watch?v=p2ljQrRl0Mg&t=143s
//https://www.youtube.com/watch?v=O0jJnBhDsuQ

//curl -X POST -H "Authorization: Bearer {token}" -H "Content-Type: application/json" -d '{"username":"admin","password":"admin"}' http://localhost:3000/login

//curl -X GET -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsImV4cCI6MTc0MDMxNDM2Mn0.YRq5N3fZCZfBEVLWmKsW7RjDidCqEjMwHtfOPA4dDaM" http://localhost:3000/info


use axum::{Router, routing::post, routing::get };

mod model;

mod controller;
use controller::get_info_handler;
use controller::login_handler;
use controller::middleware;


#[tokio::main]
async fn main() {

   /*  let login = Router::new()
    //.route("/", post());
    .route("/login", post(c));

    let info = Router::new()
    .route("/info", get(get_info_handler))
    .layer(axum::middleware::from_fn(middleware));
    
    let app = login.merge(info); */

    let app: Router = Router::new()
        .route("/login", post(login_handler))
        .route_layer(axum::middleware::from_fn(middleware))
        .route("/info", get(get_info_handler));
    

    let listener = 
    tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Listening....!");

    axum::serve(listener, app)
        .await
        .unwrap();
}
