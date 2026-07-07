use super::response::{SearchResponse, TrackDetailResponse};
use crate::providers::web::base_api;
use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::sync::Mutex;

static RNG: Lazy<Mutex<StdRng>> =
    Lazy::new(|| Mutex::new(StdRng::from_os_rng()));

static DEVICE_ID: Lazy<String> = Lazy::new(|| generate_client_id("738"));
static INSTALL_ID: Lazy<String> = Lazy::new(|| generate_client_id("739"));

fn generate_client_id(prefix: &str) -> String {
    let mut rng = RNG.lock().unwrap();
    let a = rng.random_range(10_000_000..99_999_999);
    let b = rng.random_range(10_000_000..99_999_999);
    format!("{}{}{}", prefix, a, b)
}

fn pc_common_params() -> HashMap<&'static str, String> {
    let mut m = HashMap::new();
    m.insert("aid", "386088".to_string());
    m.insert("app_name", "luna_pc".to_string());
    m.insert("device_id", DEVICE_ID.clone());
    m.insert("install_id", INSTALL_ID.clone());
    m.insert("did", DEVICE_ID.clone());
    m.insert("iid", INSTALL_ID.clone());
    m.insert("device_platform", "PC".to_string());
    m.insert("version_code", "2.1.0".to_string());
    m.insert("version_name", "2.1.0".to_string());
    m
}

fn build_pc_url(path: &str, query: &HashMap<&str, String>) -> String {
    let qs: Vec<String> = query
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect();
    format!("https://api.qishui.com/luna/pc/{}?{}", path, qs.join("&"))
}

pub async fn get_detail(track_id: &str) -> Option<TrackDetailResponse> {
    let url = build_pc_url("track_v2", &pc_common_params());
    let form = [
        ("track_id", track_id),
        ("media_type", "track"),
        ("queue_type", ""),
    ];
    let headers = [
        ("Referer", "https://api.qishui.com/"),
        ("User-Agent", "LunaPC/2.1.0(12292405)"),
    ];
    base_api::post_form(&url, &form, &headers).await
}

/// 获取汽水音乐歌词，返回 `(原文歌词, 翻译歌词)`。
pub async fn get_lyrics(
    track_id: &str,
) -> Option<(Option<String>, Option<String>)> {
    let detail = get_detail(track_id).await?;
    let lyric = detail.lyric?;
    let original = lyric.content.filter(|c| !c.is_empty());
    let translation = lyric
        .translations
        .and_then(|t| t.cn)
        .filter(|c| !c.is_empty());
    Some((original, translation))
}

pub(crate) async fn search(keyword: &str) -> Option<SearchResponse> {
    let mut query = pc_common_params();
    for key in &[
        "region", "geo_region", "os_region", "sim_region", "cdid", "channel",
        "build_mode", "network_carrier", "ac", "tz_name", "resolution", "fp",
        "cursor", "search_id", "debug_params", "from_search_id",
    ] {
        query.entry(key).or_default();
    }
    query.insert("device_type", "pc".to_string());
    query.insert("os_version", "".to_string());
    query.insert("q", keyword.to_string());
    query.insert("search_method", "input".to_string());
    query.insert("search_scene", "".to_string());

    let url = build_pc_url("search/track", &query);
    let headers = [
        ("Referer", "https://api.qishui.com/"),
        ("User-Agent", "LunaPC/2.1.0(12292405)"),
    ];
    base_api::get_json_with_headers(&url, &headers).await
}
