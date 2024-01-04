use graph_rs_sdk::*;
use graph_rs_sdk::oauth::AccessToken;

pub fn reload_access_token() ->  AccessToken {
    let access_token_json = std::fs::read_to_string("token.json").expect("Unable to read file");
    let access_token: AccessToken = serde_json::from_str(&access_token_json).unwrap();
    // access_token.bearer_token()
    access_token
}

pub async fn list_drive_items(access_token: &str) {
    drive_root(access_token).await;
    drive_root_children(access_token).await;
    special_docs(access_token).await;
}

pub async fn drive_root(access_token: &str) {
    let client = Graph::new(access_token);

    let response = client.me().drive().get_root().send().await.unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}

pub async fn drive_root_children(access_token: &str) {

    let client = Graph::new(access_token);

    let response = client
        .me()
        .drive()
        .list_root_children()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}

pub async fn special_docs(access_token: &str) {

    let client = Graph::new(access_token);


    let response = client
        .me()
        .drive()
        .get_special("documents")
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}