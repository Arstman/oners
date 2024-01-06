use drive::list_drive_items;

#[macro_use]
extern crate serde;

mod code_flow;
mod drive;

#[tokio::main]
async fn main() {
//    code_flow::start_server_main().await;
// todo: check the timestamp of acccess token struct
    let mut bear_token: String = String::new();
    let access_token = drive::reload_access_token();
    let at = access_token.clone();
    if access_token.is_expired() {
        println!("expired");
        let refresh_token_option = code_flow::refresh_token(access_token).await;
        if let Some(refresh_token) = refresh_token_option {
            bear_token = refresh_token.bearer_token().to_string();
        }
    } else {
        println!("not expired");
        bear_token = access_token.bearer_token().to_string();
    }

    list_drive_items(&bear_token).await;

    println!("{:#?}", at.as_ref());
}
