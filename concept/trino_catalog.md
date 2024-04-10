---
date: 2024-04-10
datetime: 2024-04-10 16:47:44
book: 
page: 
tags: 
references: 
aliases:
  - Trino Catalog
---
- 스키마를 포함하고, Connector를 통해 데이터 소스 참조
- **Trino는 `Catalog`에 마운트 된 Connector를 통해 데이터 접근**
- 예시
  - `HiveConnector`로 Hive warehouse에 접근하기 위해
  - `/etc/catalog/hive.properties`를 구성해야 함
- Catalog내에 schema들이 존재