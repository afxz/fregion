# 完整的中国行政区划数据

层级 | 数量 | 文件 |
|---|---|---|
| 国家 | 197 | `data/countries.json` |
| 省份 | 34 | `data/provinces.json` |
| 城市 | 342 | `data/cities.json` |
| 区县 | **2,978** | `data/districts.json` |
| 街道/乡镇 | **41,352** | `data/streets.json` |

---
```
GET /api/v1/countries                  → 197 个国家
GET /api/v1/countries/CN/provinces      → 中国 34 省
GET /api/v1/provinces/440000/cities     → 广东省 21 市
GET /api/v1/cities/440300/districts     → 深圳市 9 区
GET /api/v1/districts/440305/streets    → 某区下街道列表
```

@2026
