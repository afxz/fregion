use serde::{Deserialize, Serialize};

/// 国家
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
}

/// 省份
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Province {
    pub code: String,
    pub name: String,
    pub country_code: String,
}

/// 城市
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub code: String,
    pub name: String,
    pub province_code: String,
}

/// 区县
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct District {
    pub code: String,
    pub name: String,
    pub city_code: String,
}

/// 街道/乡镇
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Street {
    pub code: String,
    pub name: String,
    pub district_code: String,
}

/// 完整区域数据
#[derive(Debug, Clone)]
pub struct RegionData {
    pub countries: Vec<Country>,
    pub provinces: Vec<Province>,
    pub cities: Vec<City>,
    pub districts: Vec<District>,
    pub streets: Vec<Street>,
}

impl RegionData {
    /// 从指定目录加载 JSON 文件
    pub fn load(data_dir: &str) -> Self {
        Self {
            countries: load(&format!("{}/countries.json", data_dir)),
            provinces: load(&format!("{}/provinces.json", data_dir)),
            cities: load(&format!("{}/cities.json", data_dir)),
            districts: load(&format!("{}/districts.json", data_dir)),
            streets: load(&format!("{}/streets.json", data_dir)),
        }
    }
}

fn load<T: serde::de::DeserializeOwned>(path: &str) -> Vec<T> {
    let data = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("无法读取 {}: {}", path, e));
    serde_json::from_str(&data).unwrap_or_else(|e| panic!("无法解析 {}: {}", path, e))
}
