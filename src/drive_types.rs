
// ParentReference pub struct
#[derive(Debug, Deserialize)]
pub struct ParentReference {
    pub driveId: String,
    pub driveType: String,
    pub id: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct ParentReferenceForRoot {
    pub driveId: String,
    pub driveType: String,
}

#[derive(Debug, Deserialize)]
pub struct RootDriveItem {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub folder: Folder,
    pub parentReference: ParentReferenceForRoot,
    pub webUrl: String,
}

#[derive(Debug, Deserialize)]
pub struct DriveItem {
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    pub download_url: Option<String>,
    pub id: String,
    pub name: String,
    pub size: i64,
    pub file: Option<File>,
    pub folder: Option<Folder>,
    pub parentReference: ParentReference,
    pub webUrl: String,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub mimeType: String,
}

#[derive(Debug, Deserialize)]
pub struct Folder {
    pub childCount: i64,
}

#[derive(Debug, Deserialize)]
pub struct RootChildren {
    pub value: Vec<DriveItem>,
}
