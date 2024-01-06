use std::{env, fs};

use graph_rs_sdk::oauth::AccessToken;

/// This example is meant for testing and is not meant to be production ready or complete.
use graph_rs_sdk::oauth::OAuth;
use reqwest;
use warp::Filter;

// The client_id and client_secret must be changed before running this example.

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
    state: String,
}

// Create OAuth client and set credentials.
pub fn oauth_web_client() -> OAuth {
    let client_id = env::var("DEV_CLIENT_ID").expect("DEV_CLIENT_ID not set");
    let client_secret = env::var("DEV_CLIENT_SECRET").expect("DEV_CLIENT_SECRET not set");
    let mut oauth = OAuth::new();
    oauth
        .client_id(&client_id)
        .client_secret(&client_secret)
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("wl.offline_access")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.live.com/oauth20_token.srf")
        .refresh_token_url("https://login.live.com/oauth20_token.srf")
        .response_mode("query")
        .state("13534298") // Optional
        .logout_url("https://login.live.com/oauth20_logout.srf?") // Optional
        // The redirect_url given above will be used for the logout redirect if none is provided.
        .post_logout_redirect_uri("http://localhost:8000/redirect"); // Optional
    oauth
}

pub async fn refresh_token(at: AccessToken) -> Option<AccessToken> {
    let origin_at = at.clone();
    let mut oauth = oauth_web_client();
    oauth.response_type("token");
    oauth.access_token(at);

    let mut client = oauth.build_async().code_flow();
    let response = client.refresh_token().send().await.unwrap();

    if response.status().is_success() {
        let mut access_token: AccessToken = response.json().await.unwrap();
        access_token.set_refresh_token(origin_at.refresh_token().unwrap().as_str());
        access_token.gen_timestamp();
        println!("Freshed Access Token: {:?}", access_token);

        let at_json = serde_json::to_string(&access_token).unwrap();
        fs::write("token.json", at_json).expect("Unable to write refreshed token file");
        Some(access_token)
    } else {
        // See if Microsoft Graph returned an error in the Response body
        let result: reqwest::Result<serde_json::Value> = response.json().await;
        println!("{result:#?}");
        None
    }
}

pub async fn set_and_req_access_code(access_code: AccessCode) {
    let mut oauth = oauth_web_client();
    oauth.response_type("token");
    oauth.state(access_code.state.as_str());
    oauth.access_code(access_code.code.as_str());

    // Request the access token.
    let mut client = oauth.build_async().code_flow();

    let response = client.access_token().send().await.unwrap();
    println!("{response:#?}");

    if response.status().is_success() {
        let access_token: AccessToken = response.json().await.unwrap();

        println!("{access_token:#?}");
        println!("Access Token Bear: {:#?}", access_token.bearer_token());
        // save to a file for temporary storage, in a real app you would save this to a database.
        let mut at = access_token.clone();
        at.gen_timestamp();
        let at_json = serde_json::to_string(&at).unwrap();
        fs::write("token.json", at_json).expect("Unable to write file");

        oauth.access_token(access_token);
    } else {
        // See if Microsoft Graph returned an error in the Response body
        let result: reqwest::Result<serde_json::Value> = response.json().await;
        println!("{result:#?}");
    }

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("{:#?}", &oauth);
}

async fn handle_redirect(
    code_option: Option<AccessCode>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match code_option {
        Some(access_code) => {
            // Print out the code for debugging purposes.
            println!("{access_code:#?}");

            // Assert that the state is the same as the one given in the original request.
            assert_eq!("13534298", access_code.state.as_str());

            // Set the access code and request an access token.
            // Callers should handle the Result from requesting an access token
            // in case of an error here.
            set_and_req_access_code(access_code).await;

            // Generic login page response.
            Ok(Box::new(
                "Successfully Logged In! You can close your browser.",
            ))
        }
        None => Err(warp::reject()),
    }
}


use std::time::Duration;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::time::sleep;


// async fn token_obtain_server() {
//     let app = Router::new

// }


use futures::channel::oneshot;

pub async fn start_server_main() {
    let query = warp::query::<AccessCode>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<AccessCode>,), std::convert::Infallible>((None,)) });

    let routes = warp::get()
        .and(warp::path("redirect"))
        .and(query)
        .and_then(handle_redirect);

    let mut oauth = oauth_web_client();
    let mut request = oauth.build_async().code_flow();
    request.browser_authorization().open().unwrap();

    // warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    let (tx, rx) = oneshot::channel();

    let (_, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 8000), async {
            let _ = rx.await;
        });

    tokio::spawn(server);

    // 设置一个超时时间
    let timeout = Duration::from_secs(60); // 设置为 60 秒
    tokio::time::sleep(timeout).await;

    // 超时后，发送一个信号来停止服务器
    let _ = tx.send(());
}
