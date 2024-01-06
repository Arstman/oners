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
    // let drive_objec: DriveItem = serde_json::from_value(drive_item).unwrap();


    println!("{drive_item:#?}");

}

// the root response object struct 

// ParentReference struct
#[derive(Debug, Deserialize)]
struct ParentReference {
    driveId: String,
    driveType: String,
    id: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct ParentReferenceForRoot {
    driveId: String,
    driveType: String,
}

#[derive(Debug, Deserialize)]
struct RootDriveItem {
    id: String,
    name: String,
    size: i64,
    folder: Folder,
    parentReference: ParentReferenceForRoot,
    webUrl: String,
}

#[derive(Debug, Deserialize)]
struct DriveItem {
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    download_url: Option<String>,
    id: String,
    name: String,
    size: i64,
    file: Option<File>,
    folder: Option<Folder>,
    parentReference: ParentReference,
    webUrl: String,
}

#[derive(Debug, Deserialize)]
struct File {
    mimeType: String,
}

#[derive(Debug, Deserialize)]
struct Folder {
    childCount: i64,
}

#[derive(Debug, Deserialize)]
struct RootChildren {
    value: Vec<DriveItem>,
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

    let drive_objects: RootChildren = serde_json::from_value(drive_item).unwrap();


    println!("{drive_objects:#?}");
    // println!("{drive_item:#?}");
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