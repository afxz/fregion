use serde::Serialize;

/// 国家
#[derive(Debug, Clone, Serialize)]
pub struct Country {
    pub id: String,
    pub name: String,
}

/// 省份（省/直辖市/自治区）
#[derive(Debug, Clone, Serialize)]
pub struct Province {
    pub id: String,
    pub name: String,
    pub country_id: String,
}

/// 城市（地级市）
#[derive(Debug, Clone, Serialize)]
pub struct City {
    pub id: String,
    pub name: String,
    pub province_id: String,
}

/// 区县
#[derive(Debug, Clone, Serialize)]
pub struct District {
    pub id: String,
    pub name: String,
    pub city_id: String,
}

/// 完整区域数据
#[derive(Debug, Clone)]
pub struct RegionData {
    pub countries: Vec<Country>,
    pub provinces: Vec<Province>,
    pub cities: Vec<City>,
    pub districts: Vec<District>,
}

impl RegionData {
    /// 加载预置的中国行政区划数据
    pub fn load() -> Self {
        Self {
            countries: vec![Country {
                id: "CN".into(),
                name: "中国".into(),
            }],
            provinces: provinces(),
            cities: cities(),
            districts: districts(),
        }
    }
}

/// 省份数据（GB/T 2260 前2位）
fn provinces() -> Vec<Province> {
    vec![
        Province {
            id: "110000".into(),
            name: "北京市".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "120000".into(),
            name: "天津市".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "130000".into(),
            name: "河北省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "140000".into(),
            name: "山西省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "210000".into(),
            name: "辽宁省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "220000".into(),
            name: "吉林省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "230000".into(),
            name: "黑龙江省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "310000".into(),
            name: "上海市".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "320000".into(),
            name: "江苏省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "330000".into(),
            name: "浙江省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "340000".into(),
            name: "安徽省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "350000".into(),
            name: "福建省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "360000".into(),
            name: "江西省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "370000".into(),
            name: "山东省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "410000".into(),
            name: "河南省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "420000".into(),
            name: "湖北省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "430000".into(),
            name: "湖南省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "440000".into(),
            name: "广东省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "450000".into(),
            name: "广西壮族自治区".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "460000".into(),
            name: "海南省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "500000".into(),
            name: "重庆市".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "510000".into(),
            name: "四川省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "520000".into(),
            name: "贵州省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "530000".into(),
            name: "云南省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "540000".into(),
            name: "西藏自治区".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "610000".into(),
            name: "陕西省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "620000".into(),
            name: "甘肃省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "630000".into(),
            name: "青海省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "640000".into(),
            name: "宁夏回族自治区".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "650000".into(),
            name: "新疆维吾尔自治区".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "710000".into(),
            name: "台湾省".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "810000".into(),
            name: "香港特别行政区".into(),
            country_id: "CN".into(),
        },
        Province {
            id: "820000".into(),
            name: "澳门特别行政区".into(),
            country_id: "CN".into(),
        },
    ]
}

/// 城市数据（前4位）
fn cities() -> Vec<City> {
    vec![
        // 北京市 - 直辖市，市辖区直接用省级代码
        City {
            id: "110100".into(),
            name: "市辖区".into(),
            province_id: "110000".into(),
        },
        // 天津市
        City {
            id: "120100".into(),
            name: "市辖区".into(),
            province_id: "120000".into(),
        },
        // 河北省
        City {
            id: "130100".into(),
            name: "石家庄市".into(),
            province_id: "130000".into(),
        },
        City {
            id: "130200".into(),
            name: "唐山市".into(),
            province_id: "130000".into(),
        },
        City {
            id: "130300".into(),
            name: "秦皇岛市".into(),
            province_id: "130000".into(),
        },
        City {
            id: "130400".into(),
            name: "邯郸市".into(),
            province_id: "130000".into(),
        },
        // 山西省
        City {
            id: "140100".into(),
            name: "太原市".into(),
            province_id: "140000".into(),
        },
        City {
            id: "140200".into(),
            name: "大同市".into(),
            province_id: "140000".into(),
        },
        // 辽宁省
        City {
            id: "210100".into(),
            name: "沈阳市".into(),
            province_id: "210000".into(),
        },
        City {
            id: "210200".into(),
            name: "大连市".into(),
            province_id: "210000".into(),
        },
        // 吉林省
        City {
            id: "220100".into(),
            name: "长春市".into(),
            province_id: "220000".into(),
        },
        // 黑龙江省
        City {
            id: "230100".into(),
            name: "哈尔滨市".into(),
            province_id: "230000".into(),
        },
        // 上海市
        City {
            id: "310100".into(),
            name: "市辖区".into(),
            province_id: "310000".into(),
        },
        // 江苏省
        City {
            id: "320100".into(),
            name: "南京市".into(),
            province_id: "320000".into(),
        },
        City {
            id: "320200".into(),
            name: "无锡市".into(),
            province_id: "320000".into(),
        },
        City {
            id: "320300".into(),
            name: "徐州市".into(),
            province_id: "320000".into(),
        },
        City {
            id: "320400".into(),
            name: "常州市".into(),
            province_id: "320000".into(),
        },
        City {
            id: "320500".into(),
            name: "苏州市".into(),
            province_id: "320000".into(),
        },
        // 浙江省
        City {
            id: "330100".into(),
            name: "杭州市".into(),
            province_id: "330000".into(),
        },
        City {
            id: "330200".into(),
            name: "宁波市".into(),
            province_id: "330000".into(),
        },
        City {
            id: "330300".into(),
            name: "温州市".into(),
            province_id: "330000".into(),
        },
        // 安徽省
        City {
            id: "340100".into(),
            name: "合肥市".into(),
            province_id: "340000".into(),
        },
        // 福建省
        City {
            id: "350100".into(),
            name: "福州市".into(),
            province_id: "350000".into(),
        },
        City {
            id: "350200".into(),
            name: "厦门市".into(),
            province_id: "350000".into(),
        },
        // 江西省
        City {
            id: "360100".into(),
            name: "南昌市".into(),
            province_id: "360000".into(),
        },
        // 山东省
        City {
            id: "370100".into(),
            name: "济南市".into(),
            province_id: "370000".into(),
        },
        City {
            id: "370200".into(),
            name: "青岛市".into(),
            province_id: "370000".into(),
        },
        // 河南省
        City {
            id: "410100".into(),
            name: "郑州市".into(),
            province_id: "410000".into(),
        },
        // 湖北省
        City {
            id: "420100".into(),
            name: "武汉市".into(),
            province_id: "420000".into(),
        },
        // 湖南省
        City {
            id: "430100".into(),
            name: "长沙市".into(),
            province_id: "430000".into(),
        },
        // 广东省
        City {
            id: "440100".into(),
            name: "广州市".into(),
            province_id: "440000".into(),
        },
        City {
            id: "440300".into(),
            name: "深圳市".into(),
            province_id: "440000".into(),
        },
        City {
            id: "440400".into(),
            name: "珠海市".into(),
            province_id: "440000".into(),
        },
        City {
            id: "440600".into(),
            name: "佛山市".into(),
            province_id: "440000".into(),
        },
        City {
            id: "441900".into(),
            name: "东莞市".into(),
            province_id: "440000".into(),
        },
        // 广西
        City {
            id: "450100".into(),
            name: "南宁市".into(),
            province_id: "450000".into(),
        },
        // 海南省
        City {
            id: "460100".into(),
            name: "海口市".into(),
            province_id: "460000".into(),
        },
        City {
            id: "460200".into(),
            name: "三亚市".into(),
            province_id: "460000".into(),
        },
        // 重庆市
        City {
            id: "500100".into(),
            name: "市辖区".into(),
            province_id: "500000".into(),
        },
        // 四川省
        City {
            id: "510100".into(),
            name: "成都市".into(),
            province_id: "510000".into(),
        },
        // 贵州省
        City {
            id: "520100".into(),
            name: "贵阳市".into(),
            province_id: "520000".into(),
        },
        // 云南省
        City {
            id: "530100".into(),
            name: "昆明市".into(),
            province_id: "530000".into(),
        },
        // 西藏
        City {
            id: "540100".into(),
            name: "拉萨市".into(),
            province_id: "540000".into(),
        },
        // 陕西省
        City {
            id: "610100".into(),
            name: "西安市".into(),
            province_id: "610000".into(),
        },
        // 甘肃省
        City {
            id: "620100".into(),
            name: "兰州市".into(),
            province_id: "620000".into(),
        },
        // 青海省
        City {
            id: "630100".into(),
            name: "西宁市".into(),
            province_id: "630000".into(),
        },
        // 宁夏
        City {
            id: "640100".into(),
            name: "银川市".into(),
            province_id: "640000".into(),
        },
        // 新疆
        City {
            id: "650100".into(),
            name: "乌鲁木齐市".into(),
            province_id: "650000".into(),
        },
        // 台湾
        City {
            id: "710100".into(),
            name: "台北市".into(),
            province_id: "710000".into(),
        },
        // 香港
        City {
            id: "810100".into(),
            name: "香港岛".into(),
            province_id: "810000".into(),
        },
        // 澳门
        City {
            id: "820100".into(),
            name: "澳门半岛".into(),
            province_id: "820000".into(),
        },
    ]
}

/// 区县数据（完整6位）
fn districts() -> Vec<District> {
    vec![
        // 北京市辖区
        District {
            id: "110101".into(),
            name: "东城区".into(),
            city_id: "110100".into(),
        },
        District {
            id: "110102".into(),
            name: "西城区".into(),
            city_id: "110100".into(),
        },
        District {
            id: "110105".into(),
            name: "朝阳区".into(),
            city_id: "110100".into(),
        },
        District {
            id: "110106".into(),
            name: "丰台区".into(),
            city_id: "110100".into(),
        },
        District {
            id: "110107".into(),
            name: "石景山区".into(),
            city_id: "110100".into(),
        },
        District {
            id: "110108".into(),
            name: "海淀区".into(),
            city_id: "110100".into(),
        },
        // 上海市辖区
        District {
            id: "310101".into(),
            name: "黄浦区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310104".into(),
            name: "徐汇区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310105".into(),
            name: "长宁区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310106".into(),
            name: "静安区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310107".into(),
            name: "普陀区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310109".into(),
            name: "虹口区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310110".into(),
            name: "杨浦区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310112".into(),
            name: "闵行区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310113".into(),
            name: "宝山区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310114".into(),
            name: "嘉定区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310115".into(),
            name: "浦东新区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310116".into(),
            name: "金山区".into(),
            city_id: "310100".into(),
        },
        District {
            id: "310117".into(),
            name: "松江区".into(),
            city_id: "310100".into(),
        },
        // 南京市
        District {
            id: "320102".into(),
            name: "玄武区".into(),
            city_id: "320100".into(),
        },
        District {
            id: "320104".into(),
            name: "秦淮区".into(),
            city_id: "320100".into(),
        },
        District {
            id: "320105".into(),
            name: "建邺区".into(),
            city_id: "320100".into(),
        },
        District {
            id: "320106".into(),
            name: "鼓楼区".into(),
            city_id: "320100".into(),
        },
        District {
            id: "320111".into(),
            name: "浦口区".into(),
            city_id: "320100".into(),
        },
        District {
            id: "320113".into(),
            name: "栖霞区".into(),
            city_id: "320100".into(),
        },
        District {
            id: "320114".into(),
            name: "雨花台区".into(),
            city_id: "320100".into(),
        },
        District {
            id: "320115".into(),
            name: "江宁区".into(),
            city_id: "320100".into(),
        },
        // 苏州市
        District {
            id: "320505".into(),
            name: "虎丘区".into(),
            city_id: "320500".into(),
        },
        District {
            id: "320506".into(),
            name: "吴中区".into(),
            city_id: "320500".into(),
        },
        District {
            id: "320507".into(),
            name: "相城区".into(),
            city_id: "320500".into(),
        },
        District {
            id: "320508".into(),
            name: "姑苏区".into(),
            city_id: "320500".into(),
        },
        District {
            id: "320509".into(),
            name: "吴江区".into(),
            city_id: "320500".into(),
        },
        // 杭州市
        District {
            id: "330102".into(),
            name: "上城区".into(),
            city_id: "330100".into(),
        },
        District {
            id: "330103".into(),
            name: "下城区".into(),
            city_id: "330100".into(),
        },
        District {
            id: "330104".into(),
            name: "江干区".into(),
            city_id: "330100".into(),
        },
        District {
            id: "330105".into(),
            name: "拱墅区".into(),
            city_id: "330100".into(),
        },
        District {
            id: "330106".into(),
            name: "西湖区".into(),
            city_id: "330100".into(),
        },
        District {
            id: "330108".into(),
            name: "滨江区".into(),
            city_id: "330100".into(),
        },
        District {
            id: "330109".into(),
            name: "萧山区".into(),
            city_id: "330100".into(),
        },
        District {
            id: "330110".into(),
            name: "余杭区".into(),
            city_id: "330100".into(),
        },
        // 广州市
        District {
            id: "440103".into(),
            name: "荔湾区".into(),
            city_id: "440100".into(),
        },
        District {
            id: "440104".into(),
            name: "越秀区".into(),
            city_id: "440100".into(),
        },
        District {
            id: "440105".into(),
            name: "海珠区".into(),
            city_id: "440100".into(),
        },
        District {
            id: "440106".into(),
            name: "天河区".into(),
            city_id: "440100".into(),
        },
        District {
            id: "440111".into(),
            name: "白云区".into(),
            city_id: "440100".into(),
        },
        District {
            id: "440112".into(),
            name: "黄埔区".into(),
            city_id: "440100".into(),
        },
        District {
            id: "440113".into(),
            name: "番禺区".into(),
            city_id: "440100".into(),
        },
        // 深圳市
        District {
            id: "440303".into(),
            name: "罗湖区".into(),
            city_id: "440300".into(),
        },
        District {
            id: "440304".into(),
            name: "福田区".into(),
            city_id: "440300".into(),
        },
        District {
            id: "440305".into(),
            name: "南山区".into(),
            city_id: "440300".into(),
        },
        District {
            id: "440306".into(),
            name: "宝安区".into(),
            city_id: "440300".into(),
        },
        District {
            id: "440307".into(),
            name: "龙岗区".into(),
            city_id: "440300".into(),
        },
        District {
            id: "440308".into(),
            name: "盐田区".into(),
            city_id: "440300".into(),
        },
        // 成都市
        District {
            id: "510104".into(),
            name: "锦江区".into(),
            city_id: "510100".into(),
        },
        District {
            id: "510105".into(),
            name: "青羊区".into(),
            city_id: "510100".into(),
        },
        District {
            id: "510106".into(),
            name: "金牛区".into(),
            city_id: "510100".into(),
        },
        District {
            id: "510107".into(),
            name: "武侯区".into(),
            city_id: "510100".into(),
        },
        District {
            id: "510108".into(),
            name: "成华区".into(),
            city_id: "510100".into(),
        },
    ]
}
