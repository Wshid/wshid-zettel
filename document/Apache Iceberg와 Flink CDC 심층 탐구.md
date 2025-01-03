---
date: 2024-10-27
datetime: 2024-10-27 11:48:51
book: 
page: 
tags: 
references:
  - https://tech.kakao.com/posts/656
aliases:
---
Flink를 사용하여 Mysql의 테이블을 Apache Iceberg로 CDC 운영하기
서비스 팀 데이터, Data 팀 데이터에 연동
- Target으로 Mysql 사용시 성능 제약 발생
- 이에 Iceberg 도입

[[Apache Iceberg]]
- ACID 지원
- Time Trabeling 지원
- Hidden Partitioning
	- Apache Hive와 대비되는 부분
	- Hive의 경우 partition column을 쿼리의 조건 구분에 항상 명시하여야 함
		- Hive 설정 변경으로 partition col 없이도 쿼리 수행 가능하나
		- 성능 하락 존재
	- Iceberg의 경우 
		- 쿼리 작성시 사용자가 partition을 따로 명시하지 않아도 됨
		- Iceberg table에 파티션이 적절히 설정되어 있다면
			- 메타데이터에 저장된 파티션 정보를 참고하여
			- 자동으로 최적화된 데이터 접근 지원

## Iceberg 도입 필요성
- Flink를 사용한 MysqlCDC 및 Daily Batch로 hadoop 적재
	- 비효율 작업 반복
	- Mysql db 부하가 spark 성능 제약
- Iceberg 사용
	- CDC를 통해 hadoop iceberg 테이블에 실시간 적재
	- 중간 비용 감소

## Iceberg Catalog
- `iceberg_catalog`
	- `namespace`
		- 그룹화된 테이블 관리
		- 테이블 생성, 제거, 변경 등의 작업 처리
		- `current metadata pointer`
			- 현재 메타데이터에 대한 포인터
			- 사용자가 테이블에 작업 수행시 테이블 최신 메타데이터 진입점 역할
	- 테이블에 수행중인 트랜잭션 상태 확인 가능
		- 트랜잭션은 동일한 유형의 카탈로그에서 보장
		- 하나의 iceberg 테이블에 여러 유형 카탈로그 사용 => 일관적인 뷰 제공 불가
- Iceberg와 호환되는 카탈로그 유형
	- `service catalog`
		- on-premise | Cloud
		- Git과 유사하게 버전 관리 기능에 목적을 주sms Nessi
		- **Hive Metastore**
			- 실서비스시 사용
	- `file-system catalog`
		- Hadoop catalog
			- `version-hint.txt` 파일을 사용하여 테이블 최신 버전 추적
			- 단순한 저장소로서의 기능. 실서비스 환경에서 권장 x

## Iceberg Metadata, Data Layer

### 메타 데이터 계층
- 실제 데이터를 제외한 모든 정보들이 존재
- iceberg 주요 기능의 핵심
- `metadata file`
	- iceberg 테이블에 commit이 성공할 때마다 생성
	- `current metadata pointer`도 새로 생성된 `metadata file`을 가리킴
	- `commit`은 원자적으로 수행됨
	- 새로운 메타 데이터 파일이 이전 버전의 메타데이터 파일을 기반으로 생성, 교체되도록 보장
	- 테이블에 대한 기본적인 정보 & 추적중인 스냅샷들에 대한 정보 존재
		- 기본적인 정보
			- table unique id, table 설정, 스키마 정보 및 파일 저장 경로
		- sequence number: 생성된 스냅샷들의 나이
		- 통계정보, `manifest list`의 저장 경로
	- 사용자가 특정 시점의 테이블 조회시
		- 해당 스냅샷의 `manifest list`를 확인하고 필요한 파일을 읽어 테이블 형태 제공
- `manifest list`
	- 특정 스냅샷에 대한 정보
	- 스냅샷: 특정 시점의 테이블 상태
	- `manifest list`: 스냅샷에 대응되는 실제 정보를 가진 물리 파일
	- 생성된 `manifest file`에 대한 정보를 리스트 형태로 가짐
	- `manifest file`관련 정보에는 `manifest file type`, 추가 또는 삭제된 레코드 수와 같은 통계 및 파티션 정보 존재
- `manifest file`
	- 데이터 파일들과 삭제 파일들에 매칭되는 파일
	- 매칭되는 파일 타입 정보
	- 파일 타입
		- `data file`
		- `equality delete file`: 컬럼의 삭제 정보
		- `position delete file`: 파일 경로 및 위치 기반의 삭제 정보
	- 각 매니페스트 파일의 경로, 컬럼별 max, min, null 등의 통계 정보
	- 테이블 조회시 필요한 특정 매니페스트 파일만을 확인하는데 사용

### 데이터 계층
- 실제 데이터들과 변화분들이 파일로 저장됨
- 삭제 메세지를 제외한 모든 데이터 존재
- 삭제 파일
	- `equality delete file`, `position delete file`


## Iceberg 테이블 주요 설정
- Flink를 통해 Iceberg 테이블 생성
- Mysql 테이블 데이터 -> Iceberg 테이블 적재
- Spark를 통해 Iceberg 테이블 조회

### Table write mode 설정
- `write.update.mode`
- `write.delete.mode`
- `write.merge.mode`
- 데이터 적재 및 조회 방식이 달라짐
- `COW(Copy-on-Write)`
	- 데이터 적재시 기존 파일을 갱신
	- 적재 시점에 데이터 변경 사항이 기존 파일에 반영
	- 데이터 기록: 많은 비용
	- 데이터 조회: 낮은 비용
		- 이미 변화 완료된 파일을 읽기 때문
- `MOR(Merge-on-Read)`
	- 실제 데이터와 변화분(삭제)가 개별 파일에 저장
	- 테이블 조회 시점에 합쳐서 보여짐
	- 데이터 기록: 낮은 비용
	- 데이터 조회: 많은 비용

### 파티션
- Iceberg는 특정 칼럼에 파티션 설정
	- 파티션 설정시 파티션 별로 구분된 경로에 데이터 저장
- `Bucket`
	- 파티션 컬럼 값 해싱
	- 사용자가 설정한 모듈로 `modular` 연산 수행. 파티션 구분
	- 필자가 사용중인 방식
- `Identity`
	- 파티션 컬럼에 존재하는 고유 값들을 각각의 파티션으로 구분
- `Truncate`
	- 파티션 컬럼의 값을 사용자가 넘겨준 정수 길이 만큼 자른 후, `Identify` 파티션 수행
- `Hour, Day, Month, Year`
	- 파티션 컬럼의 시간 정보 값을 기준으로 파티션 구분

### 파티션 유의 사항
- 파티션은 단순히 데이터 물리적 저장 외에도 Iceberg 성능에 중요한 영향을 미침
	- 특히 `MOR`모드로 데이터가 적재되어 테이블 조회 시점에 변화분 반영시,
		- 파티션을 설정하는 것이 주요한 성능 개선 포인트가 될 수 있음
	- 파티션 설정시 테이블 조회할 때 각 파일들을 비교, 합치는 작업이 **파티션 별**로 수행되어
		- **조회 성능**을 향상 시킬 수 있음
	- Iceberg의 `Compaction`도 파티션 별로 수행됨
	- 너무 세분화된 파티션은 작은 크기의 파티션을 만들기 때문에 성능에 부정적 영향
- Iceberg가 동일 컬럼에 여러 파티션을 허용하지 않음
	- 시간 관련 컬럼에 여러 라벨 파티션 설정시 파티션 세분화에 따른 성능 저하 발생
		- `/ts_column_year=.../ts_column_month=.../`
	- `SPARK`의 `ALTER TABLE`등으로 강제 설정은 가능하나,
		- 테이블 조회시 에러 발생 가능성 존재
		- 권장하지 않음

## Flink에서 Iceberg로 준비 과정
- 팀의 미션
	- 지표 계산에 필요한 데이터 수집
		- 지표 계산에 문제가 없는 선에서 Mysql Table Column Type - Iceberg Table Column Type 매핑
	- 하둡 파일 시스템 저장소, 하이브 메타스토어 카탈록 
		- 지표 계산을 위해 사내 hadoop/hive를 이미 활용중
	- 재처리시 멱등성 보장을 위해 `upsert` 모드 사용
	- Flink Job은 오직 하나의 Mysql Table CDC 연동
		- 하나의 Flink Job에서 여러 테이블의 변화분을 읽어오는 기능은 제공하나,
		- Iceberg Flink 적재 API는 여러 테이블에 데이터를 적재하는 것을 지원하지 x
### Flink 설정
#### 하둡 인증 관련 설정
- hadoop 인증 설정을 flink에 반영
- `flink-conf.yaml`에 설정
	- flink 1.19 이상 사용시, `conf.yaml`에 지정
	- `flink-conf.yaml` -> `conf.yaml`로 설정 파일 변경됨
```yaml
hadoop.security.authentication: kerberos  
security.kerberos.login.principal: seungmin-lee@HADOOP  
security.kerberos.login.keytab: /../../seungmin-lee.keytab  
security.kerberos.access.hadoopFileSystems: hdfs://hadoop-cluster
```

#### Flink Checkpoint 주기
- Checkpoint
	- Flink가 주기적으로 저장하는 상태 정보
	- Flink Job이 중단될 경우 Job 복구에 사용되는 정보
- 일반적으로 안정성 측면에서 **주기는 작게 설정하는 것이 좋음**
- 단, Iceberg에 적재시 고려사항 존재
	- Flink -> Iceberg Table로 적재시
	- Flink Checkpoint 주기에 맞춰 Iceberg Table에 commit 수행
		- 새로운 파일들이 생성됨
	- 파일들이 작게, 많이 생성 되는 상황 => 테이블 조회 시간 증가
		- 조회시간이 늘어나는 이유?
			- 스캔플래닝 참조
- 체크포인트 주기를 너무 늘려도 좋지 않음
	- Iceberg 테이블 조회 관점에서,
		- 테이블에 적재된 데이터는 커밋이 수행되야 조회가 가능해짐
		- 실시간성이 떨어짐
- **최종적으로 10min을 체크포인트 주기로 설정**

### Hive 설정
- DDL과 연관있는 `hive.metastore.disallow.incompatible.col.type.changes` 설정
- Hive Server에서 전역 설정
	- Column Type이 변경되는 DDL의 허용 여부를 결정
- 기본적으로 Flink는
	- Mysql -> Iceberg CDC 연동시 DDL 이벤트를 지원하지 않음
	- Mysql의 데이터 변환분, 이벤트 타입에 맞춰 변경하는 코드는 `before|after` 키를 검증하나
		- DDL 이벤트에는 해당 키 값이 존재하지 x
		- DDL 정보만이 존재
	- 따라서 Mysql 연동시, DDL 이벤트가 발생하면
		- DDL과 이후 이벤트를 스킵하고
		- Iceberg Table에 Spark SQL을 통한 DDL을 별도로 수행했었음
- 단, SparkSQL로 DDL을 수정해도
	-  `hive.metastore.disallow.incompatible.col.type.changes=true` 이기 때문에
	- **컬럼 타입 변화가 기본 설정으로 지원되지 않음**
- DDL 이벤트가 컬럼 타입 변화를 일으키는지 판단하는 기준
	- Iceberg Metadata file의 관점에서 생각해보기
	- e.g. metadata file에 존재하는 파티션 정보에서 n번째 컬럼 타입이 DDL 이벤트 후 변경시
		- 타입 변화로 판단함
	- 컬럼 순서 변경 역시 metadata file 관점에서 n 번째 컬럼 타입이 변경되는 것 -> 허용되지 x
	- 만약 동일 타입의 컬럼들로만 테이블이 구성되었다면, 순서 변경이 성공할 수도 있음
- 컬럼 타입변경을 위해 해당 값을 `false`로 지정해주어야 함

### 카탈로그와 네임스페이스 설정
- 카탈로그에는 **테이블 타입**, **카탈로그 타입** 및 파일들이 저장될 경로를 설정
	- 테이블 타입: `iceberg`
	- 카탈로그 타입: `hive`
- `WAREHOUSE_LOCATION`: 모든 테이블들의 데이터 및 메타데이터 관련 파일이 저장되는 경로
- `namespace`와 `table명`을 혼합하여 **테이블별 경로**를 구분하여 설정하는 것을 권장
- hive를 카탈로그로 사용하려면 `hive metastore`의 thrift uri를 `uri` 설정에 추가해야 하나
	- hadoop, hive 관련 설정이 flink cluster에 사용하는 container image에 포함되어 스킵
- namespace에는 필수로 지정해야할 설정이 없음
	- 단, `namespace`와 `table`의 소유주를 미리 지정한다면, 차후 운영 및 권한 관리 관점에서 유용
	- 그에 따라 아래와 같이 설정
```scala
import org.apache.iceberg.flink.CatalogLoader  
  
hiveCatalogProps.put("type", "iceberg")  
hiveCatalogProps.put("catalog-type", "hive")  
hiveCatalogProps.put("warehouse", s"hdfs://hadoop-cluster/../../${namespace}/${table}")  
val hivecatalog = CatalogLoader.hive("catalog_name", hadoopConf, hiveCatalogProps) // 기존 카탈로그 로드 또는 생성  
  
namespaceProps.put(HiveCatalog.HMS_TABLE_OWNER, "seungmin-lee")  
namespaceProps.put(HiveCatalog.HMS_DB_OWNER, "seungmin-lee")  
hiveCatalog.createNamespace(Namespace.of("namespace_name"), namespaceProps) // 기존 네임스페이스 로드 또는 생성
```

### 테이블 설정
- Iceberg 테이블 설정은 팀 내부 정책에 맞춰 설정해야할 부분도 있음
	- 멱등성 보장을 위한 옵션
		- `write.upsert.enabled=True`
		- `format-version>=2`
			- 2024.10월 기준 `format-version=3`은 개발중
	- 사내 hive, trino 엔진을 통한 iceberg 테이블 조회 고려
		- `engine.hive.enabled=true`
- 파일 타입과 압축 방법은 기본값 사용
	- `parquet`, `zstd`
	- iceberg v1.4 이전까지는 `gzip` 
	- Apache Trino를 통한 조회시 `zstd`가 조회나 압축 관점에서 더 좋은 성능과 `gc` 안정성을 보여줌
		- 그에 따라 v1.4 버전 이후부터 `zstd` 압축 방식 사용
- 메타 데이터 파일의 롤링
	- `write.metadata.delete-after-commit.enabled=true`
		- hadoop fs를 위해, hadoop block size보다 작은 파일 생성은 hadoop fs의 IO 성능에 좋지 않음
	- 또한 최신의 metadata file에는 snapshot 만료 기능 미사용시
		- 과거의 스냅샷 정보들이 전부 남아 있음
		- 그에 따라 과거 metadata file은 보관할 필요가 없음
	- 파일 유지 개수, `100`(default) 사용
- 커밋과 쓰기 관련 설정
	- `commit` 실패에 대한 안정성 향상
		- retry 4 -> 60
		- 시간 기준으로는 `5min`
	- `isolation` 레벨 설정
		- 쓰기 관련 옵션
		- 동시에 여러 연산자가 데이터를 지우거나 갱신시, 어느정도 순서로 통제할지 결정
			- 현재 구현상 CDC 연동시 증분 스냅샷 단계를 제외하면,
				- iceberg 테이블에 유효한 쓰기 연산은 항상 하나만 존재
		- 쓰기 연산이 하나이기 때문에 연산들의 순서가 엉키는 경우는 없으나
			- 만약의 경우를 대비하여 `serializable`을 사용
			- default, 가장 강하게 동시성 제한
```scala
tableProperties.put(TableProperties.COMMIT_NUM_RETRIES, "60") // 기본값 4에서 60으로 증가
tableProperties.put(TableProperties.COMMIT_TOTAL_RETRY_TIME_MS, "300000") // 기본값 30분에서 5분으로 감소
tableProperties.put(TableProperties.DEFAULT_FILE_FORMAT, "parquet")  
tableProperties.put(TableProperties.ENGINE_HIVE_ENABLED, "true")  
tableProperties.put(TableProperties.FORMAT_VERSION, "2")  
tableProperties.put(TableProperties.METADATA_DELETE_AFTER_COMMIT_ENABLED, "true")  
tableProperties.put(TableProperties.PARQUET_COMPRESSION, "zstd")  
tableProperties.put(TableProperties.UPSERT_ENABLED, "true")
```
- 파티션
	- 반드시 설정해야 하는 기능
	- 기본키에 대해 버킷 파티션 설정
	- 기본키가 만약 2개 이상의 column일 경우, `cardinality`가 더 높은 컬럼을 사용함
	- 버킷 파티션의 `modulo` 값을 정할 때는
		- 5, 10, 25, 50의 값을 테스트 함
	- 너무 세분화된 파티션은 오히려 조회 성능에 영향
		- 실제로 `50` 이상부터는 `spark`를 통한 테이블 조회시 수행시간이 늘어남
	- 최종 `5`로 설정
- 쓰기 모드 설정
	- Iceberg Flink API 테이블 생성시 기본적으로 설정되는 `COW`를 사용
	- 사용 환경, 방식에 따라 성능에 영향을 미치는 설정이나,
		- Flink에서 iceberg table로 데이터를 적재하는 로직이 항상 `MOR`로 동작
	- 또한 현재 Flink만이 쓰기 연산을 수행하고,
		- 읽기 연산은 Spark를 통해 수행하기 때문에 해당 설정은 실질적인 의미가 x

## 플링크에서 아이스버그로 적재 과정
- Flink -> Iceberg로 바로 데이터를 적재하면,
	- Kafka Connect시 활용되는 Debezium의 `Change Event` 포맷이 아닌 `RowData` 포캣을 사용
- 하기 설명 내용
	- `RowData` 포맷 사용시 이점
	- Flink Dynamic Table
	- Iceberg 테이블을 어떻게 동적으로 생성하여 사용하는가
	- Source, Writer, Committer 3개의 연산자들의 동작 과정
### Source, Writer, Committer
- Source: MySQL 테이블에서 데이터를 읽고 RowData 포맷의 메세지 생성
- Writer: 상위 연산자가 건네준 메세지의 이벤트 타입에 맞춰 Iceberg 테이블에 데이터 적재
- Committer: Flink Job이 Checkpoint 수행시 Iceberg 테이블에 Commit 수행

### RowData Format
- Debezium을 기반으로 하는 CDC 연동 -> 대부분 Kafka Connect를 사용
	- `Change event` 포맷을 사용
#### Change Event format
- `Change event` 포맷은 아래와 같이 다양한 정보를 담은 Json 포맷
	- 스키마 정보
	- 쿼리 수행 전/후의 데이터
	- 원천 데이터베이스 정보
	- 라이브러리 정보
- 다양한 정보를 담고 있기 때문에, 무거워진 메세지는 메세지 처리율에 부정적인 영향을 줌
- 메세지를 가볍게 하고 싶다면 `Schema Registry`를 사용하여 스키마 정보 생략 가능
	- 단, Registry 목적의 서버 구축 및 운영해야하는 단점 존재
- Kafka와 Kafka Connect를 통해 Target MySQL 테이블에 데이터를 적재하고 있어,
	- `Change event` 포맷을 사용하고 있었음

### 기존 운영 방식과 RowData Format
- `Flink`에서 `Target Mysql` 테이블로 바로 데이터를 적재하지 못하는 이유
	- Flink CDC는 `Source Connector`위주로 데이터 수집 기능을 제공하기 때문
- 하지만 Iceberg, Flink에서 바로 데이터 적재 가능하도록 API 제공
	- Flink CDC의 Source Connector를 사용하여
	- MySQL의 데이터를 가져온 후
	- Iceberg의 Flink 적재 기능을 통해 Iceberg 테이블 적재 가능
- 그에 따라 `Kafka` `Change event` 포맷을 사용하지 않아됨
	- 대신 Flink의 `RowData` 포맷을 사용

#### RowData Forrmat의 형태
- `+I`, `+U`, `-D`
	- INSERT, UPDATE, DELETE
```sql
+I(23,seungmin,lee,seungmin@example.com,010-4321-9876,Employee,30000.00,1,1)
+U(24,Unknown,Unknown,null,None,None,0.00,-1,0)
-D(25,gildong,hong,gildong@example.com,010-1234-5678,Employee,50000.00,1,1)
```
- 간단한 포맷으로 CDC가 가능한 이유?
	- Flink Job에 필요한 모든 테이블의 정보가 이미 존재하기 때문
	- 하나의 Flink Job: 하나의 Iceberg 테이블 적재 구조
		- Change event 포맷처럼 db, table에 대한 정보를 메세지에 넣을 필요 x
- 가벼워진 메세지 포맷은
	- 메세지 처리율 향상
	- 사내 카프카에 대한 의존성이 사라짐
		- 메세지 처리율 상한 필요 x
- 결과적으로 데이터베이스 부하만 조율하면 메세지 처리율을 원하는 만큼 증가 시킬 수 있음
	- CDC 연동의 증분 스냅샷 단계를 더 빠르게 처리

#### Iceberg 테이블 적재 구조의 처리율
- 카프카 의존성이 사라진 상태
- 가벼워진 메세지 포맷
- 병렬성 1당 평균 `15k msg/s` 정도의 처리량을 가짐
- `DBA`와 부하 테스트 진행시 최대 `45`까지의 병렬성 확인
- 버퍼를 두어 `40`으로 설정 하더라도 `600k msg/s`의 처리량을 가짐
- 현재 `20`으로 설정하여 2개의 flink job 운영

### 테이블 동적 생성
- Flink -> Iceberg 테이블 적재하는 Flink Job 수행시
	- 동적으로 2개의 테이블을 생성해야 함
		- Flink Dynamic Table
		- Iceberg Table
	- 두 테이블을 생성하는데 필요한 스키마를 어떻게 동적으로 사용하는가?
	- Iceberg table을 Flink를 통해 생성하는 이유는 무엇인가?
	- 각 시스템별로 정의된 테이블들의 Column Type을 매핑한 이유는 무엇인가

#### Flink Dynamic Table
- Flink -> Iceberg 테이블 적재시 반드시 사용하는 기능
- Flink의 Table API 사용
	- Table API의 핵심적인 기능
- Mysql Table의 각 컬럼 값들을 어떤 타입으로 가져올지 결정하는 역할 수행
- Flink Dynamic Table의 Column Type에 맞추어 Mysql 테이블의 각 컬럼 데이터를 읽어와 변환
- 예시
	- Mysql Table Int type, flink dynamic table 문자열 대응
		- 정수형 값을 문자열로 변환함
- Flink Dynamic Table을 생성하는 일반적인 방법
	- 테이블 생성에 필요한 스키마 정보를 코드에 넣거나
	- 설정 파일에 기입 후 Flink Job에서 동적으로 불러와 사용
		- 테이블 스키마 정보를 코드나 설정 파일에 넣는 것은 보안상 좋지 않음?
		- 또한 스키마가 예기치 않게 변할 수 있기 때문에
			- 주기적으로 변할 가능성이 있는 정보를 파일로 관리하는 것은 운영상으로도 좋지 않음
- Iceberg 테이블도 필요한 스키마 정보를 코드나 파일에 기입한 후 Flink Job에서 생성 가능
- 단, Iceberg Table은 다른 방법으로도 생성 가능
	- Spark, Trino와 같은 별도의 엔진을 통해 생성 가능
		- 일반적으로 미리 테이블을 생성하는 방식을 많이 사용
	- 하지만 다른 엔진을 사용하여 테이블 생성하는 것은
		- Flink외에 다른 엔진에 대한 의존성을 만들고
		- 고려해야할 부분이 늘어나기 때문에 결과적으로 전체 연동 과정을 복잡하게 만듦
- 스키마 정보를 코드에 넣어 사용하는 예시
```scala
// 플링크 다이내믹 테이블 스키마 코드
val flinkDynamicTableSchema = DataTypes.ROW(  
  DataTypes.FIELD("id", DataTypes.BIGINT),  
  DataTypes.FIELD("first_name", DataTypes.STRING),  
  DataTypes.FIELD("last_name", DataTypes.STRING),  
  DataTypes.FIELD("email", DataTypes.STRING),  
  DataTypes.FIELD("phone_number", DataTypes.STRING),  
  DataTypes.FIELD("job_title", DataTypes.STRING),  
  DataTypes.FIELD("salary", DataTypes.STRING),  
  DataTypes.FIELD("department_id", DataTypes.INT),  
  DataTypes.FIELD("is_active", DataTypes.INT)
)


// 아이스버그 테이블 스키마 코드
val icebergTableSchema = new Schema(  
  Types.NestedField.required(1, "id", Types.LongType.get),  
  Types.NestedField.optional(2, "first_name", Types.StringType.get),  
  Types.NestedField.optional(3, "last_name", Types.StringType.get),  
  Types.NestedField.optional(4, "email", Types.StringType.get),  
  Types.NestedField.optional(5, "phone_number", Types.StringType.get),  
  Types.NestedField.optional(6, "job_title", Types.StringType.get),  
  Types.NestedField.optional(7, "salary", Types.StringType.get),  
  Types.NestedField.optional(8, "department_id", Types.IntegerType.get),  
  Types.NestedField.optional(9, "is_active", Types.IntegerType.get)
)
```
- 컬럼 타입에 대한 이슈도 존재함
	- Mysql Column Type, Flink의 Dynamic Table Column Type, Iceberg Column Type
		- 모두 개별 시스템에서 정의된 값
	- 컬럼 타입이 서로 호환되지 않거나
		- Flink Dynamic Table의 Column Type과 Iceberg Table의 Column Type이 호환되지 않을 수 있음
- 위 문제들을 해소하기 위해
	- Flink Job에서 `MYSQL`에 `DESCRIBE TABLE` 구문을 먼저 수행
		- 이후 수행 결과를 사용하여 동적으로 아래 정보 생성
			- Flink Dynamic Table
			- Iceberg Table 생성에 필요한 스키마
- 컬럼 타입의 목표 -> 지표 추출
	- 완전히 동일한 타입을 가져갈 필요도 없으며, 실제 가능하지도 않음

#### 컬럼 타입 매핑 예시
- 대신 라이브러리 코드 확인 및 테스트를 통해
	- 각 컬럼 타입들 간의 변환이 호환되고,
	- 지표 추출에도 문제가없는 선으로 매핑 룰을 정함
- 아래와 같은 `convert` 함수 사용
	- 해당 타입이 변환이 허용되는지를 파악함
```java
// 플링크 다이나믹 테이블의 칼럼 타입이 Int 타입인 경우
public Object convert(Object dbzObj, Schema schema) {  
    if (dbzObj instanceof Integer) {  
        return dbzObj;  
    } else {  
        return dbzObj instanceof Long ? ((Long)dbzObj).intValue() : Integer.parseInt(dbzObj.toString());  
    }  
}


// 플링크 다이나믹 테이블의 칼럼 타입이 Long 타입인 경우
public Object convert(Object dbzObj, Schema schema) {  
    if (dbzObj instanceof Integer) {  
        return ((Integer)dbzObj).longValue();  
    } else {  
        return dbzObj instanceof Long ? dbzObj : Long.parseLong(dbzObj.toString());  
    }  
```

#### 실제 사용하는 컬럼 타입 매핑 방법
- 정수형 타입, 일부 시간 관련 타입은 동일한 타입 지정
	- e.g. MySQL의 `datetime` 타입은 Flink Dynamic Table, iceberg Table에서 모두 지원되지 않음. `Timestamp` 타입 사용
- MySQL `bit` 타입은 먼저 자릿수 확인
	- 자릿수 1이면 True/False 데이터 취급. `String`으로 적재
	- 이외네는 `VARBINARY` 타입으로 읽어와 Iceberg의 `String` 타입으로 적재
- `tinyint`는 `int` 타입으로 적재
	- `0, 1`로 일반적으로 True/False 값이나, 종종 `2`이상의 값도 사용하기 때문
	- `String`으로 변환하게 되면, 2 이상의 값이 전부 `true`로 변환되어 지표 추출에 문제 발생
	- 이외의 타입들은 전부 `String` 타입으로 변환 적재
```scala
import org.apache.flink.table.types.DataType
import org.apache.iceberg.types.Types


// desc table 결과 기준, 플링크 다이나믹 테이블 스키마 타입 맵핑 룰
case "int" => DataTypes.INT()  
case "bigint" => DataTypes.BIGINT()  
case "tinyint" => DataTypes.INT() 
case "bit" if n > 1 => DataTypes.VARBINARY(n)  
case "date" => DataTypes.DATE()  
case "datetime" => DataTypes.TIMESTAMP(n)  
case "timestamp" => DataTypes.TIMESTAMP(n)  
case _ => DataTypes.STRING()


// desc table 결과 기준, 아이스버그 테이블 스키마 타입 맵핑 룰
case "int" => Types.IntegerType.get  
case "bigint" => Types.LongType.get  
case "tinyint" => Types.IntegerType.get  
case "datetime" => Types.TimestampType.withZone()  
case "timestamp" => Types.TimestampType.withZone()  
case "date" => Types.DateType.get  
case _ => Types.StringType.get
```


### 소스 연산자
- 위 테이블들이 동적 생성 이후, 소스 연산자가 Mysql에서 데이터를 읽기 시작
- 소스 연산자는 각 단계별 아래 데이터를 가져옴
	- 증분 스냅샷 단계: 테이블 데이터
	- 빈로그 스트림 단계: 바이너리 로그
		- flink의 dynamic table schema에 맞춰 컬럼 값 변환
		- 이후 RowData 타입의 메세지 생성

#### 소스 연산자의 동작 과정
- 단계
	- 설정된 접속 정보에 맞춰 DB 연결
	- MySQL 테이블의 데이터, 바이너리 로그를 읽음
	- Flink dynamic table의 컬럼 타입에 맞춰 읽어온 값들을 변환하고,
		- RowData 타입의 메세지 생성
	- 생성된 메세지를 하위 연산자로 전송
- 상세
	- 읽어온 메세지를 `RowData` 포맷 메세지로 만들려면
		- Flink CDC의 `RowDataDebeziumDeserializeSchema` 함수 사용
		- 만약 Json 타입의 Change event format으로 변환시
			- `JsonDebeziumDeserializationSchema`를 사용하면 됨
	- `RowDataDebeziumDeserializeSchema`를 사용하려면 타입 변환 필요
		- Flink Dynamic Table Schema(`DataType`) -> `TypeInformation[RowData]`
	- 해당 변환은 `DataType, logicalType, TypeInformation[RowData]` 순서로 진행되며
		- `staticfromDataTypeToLegacyInfo` 함수를 통해 한번에 변환은 가능하나,
			- 해당 함수는 곧 deprecate 예정
	- 타입 변환에는 Flink Table API에서 제공하는 `TypeConversions`와 `InternalTypeInfo`를 사용하면 됨

```scala
MySqlSource.builder[A]()  
  .hostname(...)  
  .port(...)  
  ...
  .deserializer(buildRowDataDebeziumDeserializeSchema(flinkDynamicTableSchema))
  .build()
  
private def buildRowDataDebeziumDeserializeSchema(flinkDynamicTableSchema: DataType): RowDataDebeziumDeserializeSchema = {  
  val logicalType = TypeConversions.fromDataToLogicalType(flinkDynamicTableSchema)  
  val typeInfo = InternalTypeInfo.of(logicalType).asInstanceOf[TypeInformation[RowData]]  
  
  RowDataDebeziumDeserializeSchema.newBuilder  
    .setPhysicalRowType(flinkDynamicTableSchema.getLogicalType.asInstanceOf[RowType])  
    .setChangelogMode(DebeziumChangelogMode.UPSERT)  
    .setResultTypeInfo(typeInfo)  
    .build()  
}
```

### 쓰기 연산자
- 상위 연산자에서 보낸 `RowData` 포맷의 메세지를 사용하여 iceberg 테이블 적재
- iceberg에서 제공하는 flink api를 사용할 수 있음
- `upsert` 모드를 사용하기 위해서는 동등 컬럼 설정 필요
```scala
FlinkSink.forRowData(rowDataFormatDataStmrea)
	.table(...)
	.tableLoader(...)
	.upsert(true)
	.equalityFieldColumns(pk)
	.append()
```

#### Write시 데이터와 메타데이터 활용
- 하기 코드상 메세지 타입에 따라 세부적인 동작이 다름
- `+I`(INSERT), `+U`(UPDATE_AFTER) 타입의 메세지는 동일한 로직 수행
- UPSERT 모드의 경우 메세지가 삭제 파일로도 들어감
	- 테이블 조회시 과거 데이터를 지우고 항상 최신 데이터만을 보여주기 위함
	- 테이블 조회시 데이터가 어떤 기준으로 삭제되어 보여지는지는 [[#스캔 플래닝]]파트에서 설명 예정
- `-D`(DELETE)는 삭제 파일로만 들어감

```java
public void write(RowData row) throws IOException {  
	RowDataDeltaWriter writer = route(row);  
	switch (row.getRowKind()) {  
		case INSERT:  
		case UPDATE_AFTER:  
			if (upsert) {  
				writer.deleteKey(keyProjection.wrap(row));  
			}  
			writer.write(row);  
			break;  
		case UPDATE_BEFORE:  
			if (upsert) {  
			  break;
			}  
			writer.delete(row);  
			break;  
		case DELETE:  
			if (upsert) {  
				writer.deleteKey(keyProjection.wrap(row));  
			} else {  
				writer.delete(row);  
			}  
			break;  
		default:  
			throw new UnsupportedOperationException("Unknown row kind: " + row.getRowKind());  
	}  
}
```

#### 삭제 파일
- 삭제파일은 하기 두 종류로 존재
	- 포지션 삭제 파일
	- 동등 삭제 파일
- 데이터 삭제시 관련 정보다 어떤 종류의 삭제 파일에 쓰이는가에 따라 다르며, 엔진별 구현 방식 상이
- UPSERT 모드인 경우 삭제 메세지가 삭제 파일로 들어가는 로직은 다음과 같으며, 흐름에 따라 삭제 파일이 결정됨
	- 삭제 메시지 인입
	- 동등 필드 컬럼값 확인
	- 쓰기 연산자에 위치 정보 존재 여부
		- 동등 컬럼 값이 동일한 메세지가 쓰기 연산자 메모리에 존재하는지 확인
		- 동일 스냅샷 시점 및 동일 쓰기 연산자에서 한 번 이상 들어왔다면 메모리에 존재
	- 정보 존재시,
		- 데이터 파일, 포지션 파일이 존재. 포지션 삭제 파일로 유입
	- 정보 미존재시,
		- 미존재시 동등 삭제 파일로 유입