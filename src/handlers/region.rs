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

/// 获取国家列表
pub async fn list_countries(State(state): State<AppState>) -> Json<Vec<region::Country>> {
    Json(state.region_data.countries.clone())
}

/// 根据国家ID获取省份列表
pub async fn list_provinces(
    State(state): State<AppState>,
    Path(country_id): Path<String>,
) -> Json<Vec<region::Province>> {
    let provinces: Vec<_> = state
        .region_data
        .provinces
        .iter()
        .filter(|p| p.country_id == country_id)
        .cloned()
        .collect();
    Json(provinces)
}

/// 根据省份ID获取城市列表
pub async fn list_cities(
    State(state): State<AppState>,
    Path(province_id): Path<String>,
) -> Json<Vec<region::City>> {
    let cities: Vec<_> = state
        .region_data
        .cities
        .iter()
        .filter(|c| c.province_id == province_id)
        .cloned()
        .collect();
    Json(cities)
}

/// 根据城市ID获取区县列表
pub async fn list_districts(
    State(state): State<AppState>,
    Path(city_id): Path<String>,
) -> Json<Vec<region::District>> {
    let districts: Vec<_> = state
        .region_data
        .districts
        .iter()
        .filter(|d| d.city_id == city_id)
        .cloned()
        .collect();
    Json(districts)
}
