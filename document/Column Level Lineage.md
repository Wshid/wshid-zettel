---
date: 2024-07-06
datetime: 2024-07-06 21:46:43
book: 
page: 
tags: 
references: 
aliases:
---
### DataHub에서 지원하는 Lineage
- https://datahubproject.io/docs/api/tutorials/lineage/#add-column-level-lineage
- show columns를 확인하여 컬럼 레벨 리니지를 확인할 수 있음
- 특정 python 코드를 통해 의존성 주입은 필요함

### DBT Column Level Lineage
- CLL 이라고 부름
- https://docs.getdbt.com/docs/collaborate/column-level-lineage
- DBT 유료버전에서만 지원 하는 기능(DBT Explorer)
	- Team or Enterprise plan
	- https://docs.getdbt.com/docs/collaborate/explore-projects#prerequisites
- Web UI 상으로 관련 내용 확인 가능
- 개발을 따로 하면 가능할만한 기능으로 보임

### DBT vscode plugin CLL
- https://marketplace.visualstudio.com/items?itemName=innoverio.vscode-dbt-power-user#lineage
- https://docs.myaltimate.com/test/lineage/
- 리니지 참조 목적으로 테스트는 가능할 것으로 기대

### Trino Empty Partition 생성 쿼리
```sql
CALL system.create_empty_partition(
    schema_name => 'web',
    table_name => 'page_views',
    partition_columns => ARRAY['ds', 'country'],
    partition_values => ARRAY['2016-08-09', 'US']);
```
- system 스키마 참조 필요