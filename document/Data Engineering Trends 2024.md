## 2024 데이터 엔지니어링 기술 트렌드

### 주요 트렌드
- **인공지능(AI)과 기계학습의 영향**: 데이터 엔지니어링에 인공지능과 기계학습이 통합되면서, 데이터 파이프라인과 처리가 한층 똑똑해지고 있습니다. [[1]](https://www.montecarlodata.com/blog-2024-data-engineering-trends)
- **실시간 데이터 처리**: 실시간 데이터 처리 기술은 이제 현실이 되어 데이터 흐름과 분석 방식에 혁신을 가져오고 있습니다. [[3]](https://medium.com/illumination/data-engineers-prepare-for-takeoff-top-trends-shaping-2024-b4b4c74222aa)
- **데이터 레이크하우스의 성장**: 데이터 저장 및 접근성이 데이터 레이크하우스를 통해 변모하고 있으며, 이는 클라우드 기술과의 결합을 통해 진행되고 있습니다. [[3]](https://medium.com/illumination/data-engineers-prepare-for-takeoff-top-trends-shaping-2024-b4b4c74222aa)
- **저코드/무코드 도구의 채택 증가**: 비기술 사용자도 데이터 엔지니어링에 참여할 수 있게 하는 저코드/무코드 도구의 사용이 늘어나고 있습니다. [[3]](https://medium.com/illumination/data-engineers-prepare-for-takeoff-top-trends-shaping-2024-b4b4c74222aa)

### 추가적인 인사이트
- **리소스 최적화**: 2024년 기술 산업은 자원을 최적화하는 데 압박을 받고 있으며, 이는 데이터 엔지니어링 팀의 운영 방식에 영향을 미치고 있습니다. [[2]](https://kestra.io/blogs/2024-01-24-2024-data-engineering-trends)
- **데이터 팀의 수익 중심 전환**: 데이터 팀이 기업 내에서 수익 중심의 역할을 하게 될 가능성이 있습니다. [[2]](https://kestra.io/blogs/2024-01-24-2024-data-engineering-trends)
- **이벤트 기반 시스템의 보편화**: 데이터 엔지니어링에서 이벤트 기반 시스템이 일반적인 표준으로 자리 잡을 것입니다. [[2]](https://kestra.io/blogs/2024-01-24-2024-data-engineering-trends)
- **GitOps의 채택 증가**: 데이터 팀에서 GitOps를 채택하는 경우가 더욱 많아질 것으로 보입니다. [[2]](https://kestra.io/blogs/2024-01-24-2024-data-engineering-trends)

## Data Warehouse보다 Data LakeHouse?
- [[data_lakehouse|데이터 레이크하우스]]는 Cloud [[data_warehouse|데이터 웨어하우스]]보다 경제적
```md
The new S3 Express One Zone Storage Class, the flexibility of DuckDB, the ability to lazily process dataframes in Polars, the growing adoption of Apache Iceberg, and the significantly simplified event-driven orchestration capabilities enabled by tools such as Kestra enable high-throughput data processing over data stored in the lakehouse.
```
- DuckDB의 유연성(Flexiblity)
  - https://kestra.io/blogs/2023-07-28-duckdb-vs-motherduck
- Polars(데이터를 lazy하게 처리하기)
  - https://kestra.io/blogs/2023-08-11-dataframes
- Apache Iceberg 채택 증가
  - https://kestra.io/blogs/2023-08-05-iceberg-for-aws-users
- Kestra
  - 단순화된 event 기반 Orchestration

## Commercial vs. Open Source Battles among LLMs and Data Tools
```md
Data tools face similar challenges of balancing open-source and commercial offerings. Tools like Snowplow and Terraform adjusted their licenses and operating models to focus on serving enterprise clients and generating revenue
```
- opensource와 commercials의 균형
- snowplow
  - https://snowplow.io/blog/introducing-snowplow-limited-use-license/
- terraform
  - https://www.hashicorp.com/blog/hashicorp-adopts-business-source-license

## Keywords
- Kestra
  - GitOps와 연동
  - 단순한 event driven orchestration
- Polars의 Data Lazy Evaluation
- Iceberg
- DuckDB

## References
- https://kestra.io/blogs/2024-01-24-2024-data-engineering-trends