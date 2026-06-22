#!/usr/bin/env python3
"""
从 GitHub 获取中国行政区划数据并生成带 GB/T 2260 编码的扁平 JSON 文件。
用法: python3 scripts/generate_region_data.py
"""

import json
import urllib.request

# 省份编码对照表 (GB/T 2260 前2位)
PROVINCE_CODES = {
    "北京市": "110000", "天津市": "120000", "河北省": "130000",
    "山西省": "140000", "内蒙古自治区": "150000", "辽宁省": "210000",
    "吉林省": "220000", "黑龙江省": "230000", "上海市": "310000",
    "江苏省": "320000", "浙江省": "330000", "安徽省": "340000",
    "福建省": "350000", "江西省": "360000", "山东省": "370000",
    "河南省": "410000", "湖北省": "420000", "湖南省": "430000",
    "广东省": "440000", "广西壮族自治区": "450000", "海南省": "460000",
    "重庆市": "500000", "四川省": "510000", "贵州省": "520000",
    "云南省": "530000", "西藏自治区": "540000", "陕西省": "610000",
    "甘肃省": "620000", "青海省": "630000", "宁夏回族自治区": "640000",
    "新疆维吾尔自治区": "650000", "台湾省": "710000",
    "香港特别行政区": "810000", "澳门特别行政区": "820000",
}

# 直辖市
DIRECT_MUN = {"110000", "120000", "310000", "500000"}

print("正在获取数据...")
url = "https://raw.githubusercontent.com/modood/Administrative-divisions-of-China/master/dist/pcas.json"
req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
with urllib.request.urlopen(req) as resp:
    provinces = json.loads(resp.read().decode("utf-8"))
print(f"获取到 {len(provinces)} 个省级行政区")

countries = [{"code": "CN", "name": "中国"}]
provinces_out = []
cities_out = []
districts_out = []
streets_out = []

# 直辖市/省直辖 名称
SPECIAL_NAMES = {"省直辖县级行政区划", "省直辖", "自治区直辖县级行政区划", "自治区直辖", "县"}

for pname, pdata in provinces.items():
    pcode = PROVINCE_CODES.get(pname)
    if not pcode:
        print(f"  ⚠ 未找到省份编码: {pname}")
        continue

    provinces_out.append({"code": pcode, "name": pname, "country_code": "CN"})
    is_direct = pcode in DIRECT_MUN
    c_seq = 1

    for cname, cdata in pdata.items():
        if is_direct and cname == "市辖区":
            ccode = f"{pcode[:2]}0100"
        elif is_direct and cname == "县":
            ccode = f"{pcode[:2]}0200"
            c_seq += 1
        elif cname in SPECIAL_NAMES:
            ccode = f"{pcode[:2]}9000"
            c_seq += 1
        else:
            ccode = f"{pcode[:2]}{c_seq:02d}00"

        cities_out.append({"code": ccode, "name": cname, "province_code": pcode})
        d_seq = 1

        for dname, slist in cdata.items():
            # 海南省直辖县级市特殊处理
            if cname in ("省直辖县级行政区划", "省直辖"):
                dcode = f"46{d_seq + 90:02d}00"
            else:
                dcode = f"{ccode[:4]}{d_seq:02d}"

            districts_out.append({"code": dcode, "name": dname, "city_code": ccode})
            s_seq = 1
            for sname in slist:
                scode = f"{dcode}{s_seq:03d}"
                streets_out.append({"code": scode, "name": sname, "district_code": dcode})
                s_seq += 1

            d_seq += 1

        if not (is_direct and cname == "市辖区"):
            c_seq += 1

# 写入
for fname, data in [
    ("data/provinces.json", provinces_out),
    ("data/cities.json", cities_out),
    ("data/districts.json", districts_out),
    ("data/streets.json", streets_out),
]:
    with open(fname, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)
    print(f"✓ {fname}: {len(data)} 条")

total = f"国家 {len(countries)}, 省份 {len(provinces_out)}, 城市 {len(cities_out)}, 区县 {len(districts_out)}, 街道 {len(streets_out)}"
print(f"\n生成完成: {total}")
