// use axum::{
//     extract::{Path, Multipart, State},
//     response::{IntoResponse, Response},
//     routing::{get, post, delete},
//     Json, Router,
//     http::{StatusCode, header},
//     body::Body,
// };
// use std::{fs, io::Write, path::PathBuf, sync::{Arc, Mutex}};
// use uuid::Uuid;
// use crate::models::filemeta::FileMeta;
// use tokio_util::io::ReaderStream;
// use tokio::fs::File;

// pub type FileStore = Arc<Mutex<Vec<FileMeta>>>;
// const FILE_DIR: &str = "./files/";

// pub fn file_routes(store: FileStore) -> Router {
//     fs::create_dir_all(FILE_DIR).unwrap();

//     Router::new()
//         .route("/files", post(upload_file).get(list_files))
//         .route("/files/:id", get(download_file).delete(delete_file))
//         .with_state(store)
// }

// async fn upload_file(
//     State(store): State<FileStore>,
//     mut multipart: Multipart,
// ) -> Result<Json<FileMeta>, StatusCode> {
//     while let Ok(Some(field)) = multipart.next_field().await {
//         let filename = field.file_name().unwrap_or("file").to_string();
//         let id = Uuid::new_v4().to_string();
//         let saved_path = format!("{FILE_DIR}{id}");

//         // Read file content
//         let data = match field.bytes().await {
//             Ok(d) => d,
//             Err(e) => {
//                 eprintln!("❌ Failed to read uploaded file: {:?}", e);
//                 return Err(StatusCode::INTERNAL_SERVER_ERROR);
//             }
//         };

//         if data.len() > 50 * 1024 * 1024 {
//             eprintln!("❌ File too large: {} bytes", data.len());
//             return Err(StatusCode::PAYLOAD_TOO_LARGE);
//         }

//         // Save file to disk
//         match std::fs::File::create(&saved_path) {
//             Ok(mut file) => {
//                 if let Err(e) = file.write_all(&data) {
//                     eprintln!("❌ Failed to write file: {:?}", e);
//                     return Err(StatusCode::INTERNAL_SERVER_ERROR);
//                 }
//             }
//             Err(e) => {
//                 eprintln!("❌ Failed to create file: {:?}", e);
//                 return Err(StatusCode::INTERNAL_SERVER_ERROR);
//             }
//         }

//         // Save metadata
//         let meta = FileMeta { id: id.clone(), filename };
//         store.lock().unwrap().push(meta.clone());

//         return Ok(Json(meta));
//     }

//     eprintln!("❌ No file found in multipart form.");
//     Err(StatusCode::BAD_REQUEST)
// }

// async fn download_file(Path(id): Path<String>) -> Result<Response, StatusCode> {
//     let path = PathBuf::from(format!("{FILE_DIR}{id}"));
//     if !path.exists() {
//         return Err(StatusCode::NOT_FOUND);
//     }

//     let file = File::open(path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//     let stream = ReaderStream::new(file);
//     Ok(Response::builder()
//         .status(StatusCode::OK)
//         .header(header::CONTENT_TYPE, "application/octet-stream")
//         .body(Body::from_stream(stream))
//         .unwrap())
// }

// async fn delete_file(Path(id): Path<String>, State(store): State<FileStore>) -> StatusCode {
//     let path = PathBuf::from(format!("{FILE_DIR}{id}"));
//     let mut files = store.lock().unwrap();

//     if let Some(index) = files.iter().position(|f| f.id == id) {
//         files.remove(index);
//         if path.exists() {
//             fs::remove_file(path).ok();
//         }
//         StatusCode::NO_CONTENT
//     } else {
//         StatusCode::NOT_FOUND
//     }
// }

// async fn list_files(State(store): State<FileStore>) -> Json<Vec<FileMeta>> {
//     Json(store.lock().unwrap().clone())
// }