use serde::{Deserialize, Serialize};
use specta::Type;

pub mod prelude {
    pub use crate::events::{DownloadEvent, SetProxyEvent, UpdateDownloadedFavoriteComicEvent};
}

#[derive(Debug, Clone, Serialize, Deserialize, Type,)]
#[serde(tag = "event", content = "data")]
pub enum DownloadEvent {
    #[serde(rename_all = "camelCase")]
    ChapterPending {
        chapter_id: i64,
        comic_title: String,
        chapter_title: String,
    },

    #[serde(rename_all = "camelCase")]
    ChapterStart { chapter_id: i64, total: u32 },

    #[serde(rename_all = "camelCase")]
    ChapterEnd {
        chapter_id: i64,
        err_msg: Option<String>,
    },

    #[serde(rename_all = "camelCase")]
    ImageSuccess {
        chapter_id: i64,
        url: String,
        current: u32,
    },

    #[serde(rename_all = "camelCase")]
    ImageError {
        chapter_id: i64,
        url: String,
        err_msg: String,
    },

    #[serde(rename_all = "camelCase")]
    OverallUpdate {
        downloaded_image_count: u32,
        total_image_count: u32,
        percentage: f64,
    },

    #[serde(rename_all = "camelCase")]
    OverallSpeed { speed: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, )]
#[serde(tag = "event", content = "data")]
pub enum SetProxyEvent {
    #[serde(rename_all = "camelCase")]
    Error { err_msg: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, )]
#[serde(tag = "event", content = "data")]
pub enum UpdateDownloadedFavoriteComicEvent {
    #[serde(rename_all = "camelCase")]
    GettingFolders,

    #[serde(rename_all = "camelCase")]
    GettingComics { total: i64 },

    #[serde(rename_all = "camelCase")]
    ComicGot { current: i64, total: i64 },

    #[serde(rename_all = "camelCase")]
    DownloadTaskCreated,
}
