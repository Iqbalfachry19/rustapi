use http::{StatusCode};
use vercel_lambda::{lambda, error::VercelError, IntoResponse, Request, Response};
use std::error::Error;
use serde::{Serialize};

#[derive(Serialize)]
struct User {
    id:u8,
    name: String,
    age: u8,
}

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {


    let path_and_query = request.uri().path_and_query().ok_or("Invalid URI").unwrap();
    let path_segments = path_and_query.path().split('/').collect::<Vec<_>>();
	

    let id = path_segments[2];
	let id: u8 = id.parse().unwrap();

	
    let users =  vec![
        User {id:1, name: "Alice".to_string(), age: 30 },
        User {id:2, name: "Bob".to_string(), age: 35 },
        User { id:3,name: "Charlie".to_string(), age: 40 },
    ];
    let mut found_user = None;
    for user in &users {
        if user.id == id {
            found_user = Some(user);
            break;
        }
    }
    let json = serde_json::to_string(&found_user).unwrap();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json)
        .expect("Internal Server Error");

    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}