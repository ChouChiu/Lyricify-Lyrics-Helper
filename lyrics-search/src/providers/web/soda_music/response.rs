use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SearchResponse {
    #[serde(rename = "status_code")]
    #[allow(dead_code)]
    pub(crate) status_code: Option<i32>,
    #[serde(rename = "result_groups")]
    pub(crate) result_groups: Option<Vec<ResultGroup>>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ResultGroup {
    pub(crate) data: Option<Vec<ResultGroupItem>>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ResultGroupItem {
    pub(crate) entity: Option<Entity>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Entity {
    pub(crate) track: Option<Track>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Track {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) duration: Option<i64>,
    pub(crate) artists: Option<Vec<Artist>>,
    pub(crate) album: Option<Album>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Artist {
    pub(crate) name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Album {
    pub(crate) name: String,
}

/// 汽水音乐 track_v2 接口返回（获取歌曲详情/歌词）
#[derive(Debug, Clone, Deserialize)]
pub struct TrackDetailResponse {
    #[serde(rename = "status_code")]
    #[allow(dead_code)]
    pub(crate) status_code: Option<i32>,
    #[serde(default)]
    pub(crate) lyric: Option<LyricData>,
}

/// 歌词数据
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct LyricData {
    /// 歌词正文（LRC 格式）
    #[serde(default)]
    pub(crate) content: Option<String>,
    /// 歌词语言
    #[serde(default)]
    pub(crate) lang: Option<String>,
    /// 歌词类型（如 "lrc"）
    #[serde(rename = "type", default)]
    pub(crate) lyric_type: Option<String>,
    /// 各语言翻译
    #[serde(rename = "lang_translations", default)]
    pub(crate) lang_translations: Option<std::collections::HashMap<String, LyricTranslation>>,
    /// 简体中文翻译（来自 `translations.cn`）
    #[serde(default)]
    pub(crate) translations: Option<LyricTranslationData>,
}

/// 翻译歌词数据
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct LyricTranslation {
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) lang: Option<String>,
    #[serde(rename = "type", default)]
    pub(crate) translation_type: Option<String>,
}

/// 翻译数据包装（`translations` 字段）
#[derive(Debug, Clone, Deserialize)]
pub struct LyricTranslationData {
    /// 简体中文翻译
    #[serde(default)]
    pub(crate) cn: Option<String>,
}
