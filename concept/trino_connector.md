---
date: 2024-04-10
datetime: 2024-04-10 16:47:01
book: 
page: 
tags: 
references: 
aliases:
  - Trino Connector
---
- Hive, RDBMS 등에 Trino 연결
- Database와 driver와 동일한 역할
- 데이터소스에서 데이터를 읽을 수 있도록
  - Coordinator, Worker와 데이터 소스를 연결
- Catalog는 특정 connector와 연결 됨
  - Catalog config 파일로 필요한 속성 값 확인 가능
  - e.g. `connector.name=mysql`