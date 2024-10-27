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
