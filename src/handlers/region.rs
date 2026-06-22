use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::models::region::{self, RegionData};

/// 应用共享状态
#[derive(Clone)]
pub struct AppState {
    pub region_data: Arc<RegionData>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            region_data: Arc::new(RegionData::load()),
        }
    }
}

// ── 国家 ──

pub async fn list_countries(State(state): State<AppState>) -> Json<Vec<region::Country>> {
    Json(state.region_data.countries.clone())
}

// ── 省份 ──

pub async fn list_provinces(
    State(state): State<AppState>,
    Path(country_code): Path<String>,
) -> Json<Vec<region::Province>> {
    let result: Vec<_> = state
        .region_data
        .provinces
        .iter()
        .filter(|p| p.country_code == country_code)
        .cloned()
        .collect();
    Json(result)
}

// ── 城市 ──

pub async fn list_cities(
    State(state): State<AppState>,
    Path(province_code): Path<String>,
) -> Json<Vec<region::City>> {
    let result: Vec<_> = state
        .region_data
        .cities
        .iter()
        .filter(|c| c.province_code == province_code)
        .cloned()
        .collect();
    Json(result)
}

// ── 区县 ──

pub async fn list_districts(
    State(state): State<AppState>,
    Path(city_code): Path<String>,
) -> Json<Vec<region::District>> {
    let result: Vec<_> = state
        .region_data
        .districts
        .iter()
        .filter(|d| d.city_code == city_code)
        .cloned()
        .collect();
    Json(result)
}

// ── 街道 ──

pub async fn list_streets(
    State(state): State<AppState>,
    Path(district_code): Path<String>,
) -> Json<Vec<region::Street>> {
    let result: Vec<_> = state
        .region_data
        .streets
        .iter()
        .filter(|s| s.district_code == district_code)
        .cloned()
        .collect();
    Json(result)
}
