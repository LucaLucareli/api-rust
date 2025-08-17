// use axum::{
//     extract::{Json, Path, Query},
//     http::StatusCode,
//     response::Json as JsonResponse,
// };
// use crate::services::video_service::VideoService;
// use crate::dto::{CreateVideoRequestDto, UpdateVideoRequestDto, VideoResponseDto};
// use serde::{Deserialize, Serialize};
// // use sea_orm::{DatabaseConnection, Statement, FromQueryResult};

// #[derive(Debug, Deserialize)]
// pub struct ListVideosQuery {
//     pub limit: Option<u64>,
//     pub offset: Option<u64>,
// }

// #[derive(Debug, Serialize)]
// pub struct ListVideosResponseDto {
//     pub videos: Vec<VideoResponseDto>,
//     pub total: u64,
//     pub limit: u64,
//     pub offset: u64,
// }

// pub struct VideoController {
//     video_service: VideoService,
// }

// impl VideoController {
//     // pub fn new() -> Self {
//     //     // let db = DatabaseConnection::default(); // TODO: Conectar com banco real
        
//     //     Self {
//     //         video_service: VideoService::new(db),
//     //     }
//     // }

//     pub async fn create_video(
//         Json(payload): Json<CreateVideoRequestDto>,
//     ) -> Result<JsonResponse<VideoResponseDto>, StatusCode> {
//         // let controller = Self::new();
        
//         // match controller.video_service.create_video(payload).await {
//         //     Ok(video) => Ok(JsonResponse(video)),
//         //     Err(_) => Err(StatusCode::BAD_REQUEST),
//         // }
//         Ok(())
//     }

//     pub async fn get_video_by_id(
//         Path(video_id): Path<String>,
//     ) -> Result<JsonResponse<VideoResponseDto>, StatusCode> {
//         // let controller = Self::new();
        
//         // match controller.video_service.get_video_by_id(&video_id).await {
//         //     Ok(Some(video)) => Ok(JsonResponse(video)),
//         //     Ok(None) => Err(StatusCode::NOT_FOUND),
//         //     Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
//         // }
//         Ok(())
//     }

//     pub async fn update_video(
//         Path(video_id): Path<String>,
//         Json(payload): Json<UpdateVideoRequestDto>,
//     ) -> Result<JsonResponse<VideoResponseDto>, StatusCode> {
//         // let controller = Self::new();
        
//         // match controller.video_service.update_video(&video_id, payload).await {
//         //     Ok(Some(video)) => Ok(JsonResponse(video)),
//         //     Ok(None) => Err(StatusCode::NOT_FOUND),
//         //     Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
//         // }

//         Ok(())
//     }

//     pub async fn delete_video(
//         Path(video_id): Path<String>,
//     ) -> Result<JsonResponse<serde_json::Value>, StatusCode> {
//         // let controller = Self::new();
        
//         // match controller.video_service.delete_video(&video_id).await {
//         //     Ok(true) => {
//         //         let response = serde_json::json!({
//         //             "message": "Vídeo deletado com sucesso"
//         //         });
//         //         Ok(JsonResponse(response))
//         //     }
//         //     Ok(false) => Err(StatusCode::NOT_FOUND),
//         //     Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
//         // }

//         Ok(())
//     }

//     pub async fn list_videos(
//         Query(query): Query<ListVideosQuery>,
//     ) -> Result<JsonResponse<ListVideosResponseDto>, StatusCode> {
//         // let controller = Self::new();
        
//         // match controller.video_service.list_videos(query.limit, query.offset).await {
//         //     Ok(videos) => {
//         //         let response = ListVideosResponseDto {
//         //             videos: videos,
//         //             // total: videos.len() as u64,
//         //             total: 10,
//         //             limit: query.limit.unwrap_or(10),
//         //             offset: query.offset.unwrap_or(0),
//         //         };
//         //         Ok(JsonResponse(response))
//         //     }
//         //     Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
//         // }

//         Ok(())
//     }
// }
