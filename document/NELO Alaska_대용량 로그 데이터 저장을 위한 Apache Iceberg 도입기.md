---
date: 2025-03-15
datetime: 2025-03-15 14:32:53
book: 
page: 
tags: 
references:
  - https://d2.naver.com/helloworld/8998207
aliases:
---
- ElasticSearch 기반의 로그 모니터링 시스템
	- 비용 및 확장성 문제 발생
- Iceberg를 도입하여 모니터링 시스템 플랫폼에 적용

## Elasticsearch 기반 기존 로그 모니터링 시스템 한계
- Client -> Kafka -> ES 구조
	- ES Hot Tier: SSD 사용, 3일동안 저장
	- ES Warn Tier: HDD 사용, 최대 90일 저장
	- 데이터 효율적, 저비용 저장 가능
- 데이터 증가에 따라 Warm 계층에 저장된 데이터의 급증

## 새로운 타입의 데이터 저장 스토리지 필요성
- Warm 로그 급증 -> 장기간 로그 저장에 대한 요구 사항
- 서비스의 법적 요구 사항으로 90d -> 1y의 로그 데이터 저장 허용이 필요함
- Elasticsearch가 과연 적합한 구조였는가
	- 전체 검 쿼리중 95%가 당일 발생 쿼리, 99% 쿼리가 일주일 이내의 데이터
	- 0.5% 쿼리가 2주 이상 지난 데이터 요청
	- 검색 되지 않는 데이터를 저장하는 것은 효율적이지 x
- 또한, Elasticsearch의 마스터 노드가 관리할 수 있는 메타 데이터 규모의 한계 도달
- 위 문제를 해결하기 위해서는
	- Elasticsearch에는 자주 일어나는 단기간의 데이터 저장 허용
	- 장기간 데이터 저장을 위한 스토리지 필요
- **데이터 저장을 위한 스토리지 + 검색을 위한 컴퓨팅 분리**
- 저비용의 스토리지 검색이 가능한 데이터 포맷으로 저장할 수 있는 방법 확인
	- Iceberg, 기존 모니터링 시스템 대비 50% 이상 비용 절감

## Iceberg의 특징
- 새로운 타입의 스토리지 요구사항
	- 데이터 읽기/쓰기 동시에 필요
	- 데이터 읽기/쓰기 발생 상황에서 스키마 변경 가능해야 함
	- 단일 테이블로 페타바이트 규모 데이터 저장 필요
	- 수십만개의 테이블 운영 필요
	- 데이터 포맷으로 인한 쿼리 엔진 제한 x
	- 데이터 저장소와 쿼리 컴퓨팅 노드가 분리
	- 데이터 압축 효율 우수
- 위 요구사항을 구현할 수 있는 기술 -> 오픈 데이터 포맷을 사용하는
	- Iceberg, Delta Lake, Apache Hudi
	- Iceberg가 제일 활발히 업데이트 되고 있었음
	- Databricks의 주요 기술. 오픈 테이블 포맷 기술의 주도권 보유

### [[Apache Iceberg]]의 구조
- 데이터, 메타데이터, 카탈로그의 세 부분으로 나누어 저장
	- 데이터
		- Parquet으로 관리되며, [zstd](https://datatracker.ietf.org/doc/html/rfc8478) 형식으로 압축
		- 데이터는 오브젝트 스토리지에 저장
	- 메타데이터
		- 하나의 테이블을 구성하기 위한 데이터 파일의 집합 관계, 스키마 정보
		- JSON, Avro 같은 형태로 저장
		- 메타데이터는 오브젝트 스토리지에 저장
	- 카탈로그
		- 메타데이터의 메타
		- 가장 최신의 메타 데이터 위치 정보 같은 최소한의 정보만 카탈로그에서 관리
		- 카탈로그 데이터는 데이터베이스에 저장
- 테이블로 구성한 파일에 대한 메타데이터까지 함께 관리
	- Iceberg는 단순한 데이터 포맷이 아닌 **테이블 포맷**이라고 부름
- Iceberg는 ACID Transaction을 지원하며 schema evolution, hidden partitioning 등 유용한 기능 제공

## 신규 로그 모니터링 시스템의 구조
- Iceberg를 기반으로 개발한 새로운 로그 모니터링 시스템
	- Elasticsearch 기반의 모니터링 시스템 대체 x
	- Elasticserach -> 실시간 모니터링이 필요한 짧은 기간의 로그
	- Iceberg -> 장기간 보관이 필요한 데이터 
- ES 기반 모니터링 시스템: Kafka -> Elasticsearch에 Indexing
- Iceberg 기반 시스템: Kafka -> Iceberg Table format으로 저장
	- Elasticsearch Warm 계층에서 데이터를 읽어 저장하지 않는 이유?
		- 실시간 검색/모니터링이 필요하지 않는 데이터, ES가 아닌 Iceberg로 직접 저장 가능
		- ES와 Iceberg 중복 데이터 저장시, Iceberg 기반 시스템 비용이 매우 저렴
		- ES의 Warm 계층으로 데이터를 읽으면 HDD 기반의 클러스터에 큰 부하 발생

### 신규 모니터링 시스템 아키텍처
- 하기 두 부분으로 구분됨
	- 데이터 적재(Data ingestion & Optimization)
	- 데이터 쿼리(Data Query)
- ![Image](https://github.com/user-attachments/assets/41ce4806-e95a-46dd-aaf4-2959fd222b3f)
- 데이터 적재
	- Orca: Kafka 데이터를 Iceberg Table format으로 변환하여 오브젝트 스토리지에 저장하는 컴포넌트
	- Polarbear: Iceberg 테이블 데이터를 최적화하고 데이터 라이프사이클을 관리하는 컴포넌트
	- Puffin: Iceberg 카탈로그 컴포넌트
- 데이터 쿼리
	- Trino: Iceberg 테이블 조회를 위한 쿼리 컴퓨팅 엔진
	- API Server: Alaska 데이터 조회를 위한 Nelo Open API 제공
	- Frontend: Alaska 쿼리 UI 제공

### Kappa Architecture
- 신규 모니터링 시스템의 데이터 프로세싱 구조
- **실시간으로 저장되고 있는 로그 데이터 테이블에 사용자가 접근해 데이터를 조회할 수 있는 구조**
- [[Lambda Architecture]]는 로그 저장 목적으로 사용하기에는 너무 복잡하고 효율적이지 않음
- Iceberg의 Open Table Format은 ACID Transaction을 지원
	- 실시간 쓰기가 발생하는 테이블을 동시에 사용자가 읽어도 데이터 정합성 보장 및 서비스
- 사용자는 짧은 지연 시간(데이터 동기화 주기 5분)안에 데이터 조회 가능
- 데이터 저장을 위해 사내 오브젝트 스토리지 서비스인 Nubes
	- MiniIO라는 S3 게이트웨이 활용
	- S3 인터페이스를 기반으로 Iceberg와 연동되어 있음

### 오픈소스 PoC
- Orca, Polarbear, Puffin 모두 Iceberg Java SDK를 기반으로 직접 개발한 컴포넌트
- Poc를 진행하였으나, 하기와 같은 한계로 사용 불가

#### 데이터 적재
- kafka-connect
	- 기능적 요구사항은 충족하였으나, 지원하는 동기화 대상 테이블의 수가 적음
	- 동기화 대상 테이블은 수십만이었으나,
		- kafka-connect는 테이블 수가 수백개 수준만 도달해도 OOM 발생
- flink
	- Kafka의 데이터를 Iceberg로 저장하는 기능은 제공하나, 단일 테이블에 대해서 동작
	- 테이블 [[fan-out]] 기능이 존재하지 않음
	- 동기화해야 하는 테이블의 수만큼 flink app을 실행해야하는 경우 존재
		- 현실적으로 운영이 어려움

#### 데이터 최적화
- Trino, Spark, Hive 등 Iceberg 테이블을 사용하는 쿼리 엔진
	- 데이터 최적화 및 라이프사이클을 관리하는 것이 기능적으로 가능
	- 단, 요구하는 테이블의 규모를 지원하려면 비용 부담이 큼
	- 또한 세부적인 스케줄링, 스로틀링 설정이 어렵기 때문에 오브젝트 스토리지에 과한 부담 발생

#### 카탈로그
- Hive metastore, Nessie, Polaris, Unity 등 Iceberg 테이블을 지원하는 카탈로그
	- 초기 설계에서는 Hive metastore를 사용
		- [Hive Lock 버그](https://github.com/apache/iceberg/issues/10429)로 인해 경합이 심할 때는 데드락에 빠지는 이슈 발생
	- **장기적으로 Iceberg REST Catalog를 표준**으로 만들고, 다른 카탈로그를 직접적으로 사용하는 것을 중단할 계획이 있다고 함
		- by Iceberg Community
	- REST Catalog는 표준 스펙만 존재하며 공식적인 구현체가 존재하지 않음
	- Snowflake에서 최근에 Polaris라는 REST Catalog 스펙에 준한 카탈로그를 공개하였으나 특정 카탈로그에 제한될 우려 존재
	- 또한 카탈로그를 사용자에게 공개해 데이터 연동(data federation) 제공 계획이 있어
		- 컴포넌트를 직접 개발하는 것이 효율적이라 판단

## 데이터 적재
- Orca가 Kafka -> Iceberg 테이블 포맷으로 변환해 저장하는 과정
- ![Image](https://github.com/user-attachments/assets/87e53623-fdc2-42d3-a595-04a052358523)
- Kafka 데이터 수신
	- Kafka topic으로 부터 데이터를 읽음
	- 다중 컨슈머 구성을 통해 I/O 병목 문제 해결, 처리량 극대화
- 로그 데이터 관리 및 전달
	- 데이터를 수신한 후 데이터를 내부 메모리 큐에 적재
	- 메모리 큐에 적재된 데이터는 record repository를 통해 각 Iceberg 테이블에 해당하는 Writer로 분배
- 데이터 포맷 변환 및 저장
	- 각 Writer는 데이터를 Parquet 형식 변환, Writer 내부 메모리 버퍼에 저장
	- Flush Manager가 특정 주기로 오브젝트 스토리지에 데이터 저장. Iceberg 테이블 커밋

### 여러가지 상황을 고려한 설계
#### Table fan-out 기능
- Kafka 토픽 저장 로그, tenant별로 분리되어 Iceberg 테이블에 저장
- 단일 데이터 스트림에서 다수의 테이블로 데이터를 전송하는 fan-out이 필요
- 테이블 데이터가 처음 인입되는 시점에 동적으로 Writer 생성, flush가 실행되는 시점에 메모리 해제

#### 효율적 메모리 관리
- 초당 수십만 건의 로그 데이터, 실시간 처리에는 메모리 사용량 최적화 필요
- 실시간 유입 데이터를 변환해 메모리에 적재. 5분 단위로 flush로 메모리 확보
- 특정 테이블에 데이터 다량 유입시, 해당 테이블에 해당하는 데이터를 파일로 먼저 내보내는 roll-over 동작 수행
- 메모리 사용량 급증시 전체 Writer에서 강제 flush를 진행하여 OOM 예방

#### Kafka Offset 관리
- Kafka로 부터 읽은 데이터의 Iceberg 테이블 커밋이 완료된 이후 Kafka Offset commit이 가능함
- Kafka로부터 읽어온 batch 단위로 Iceberg table commit은 비효율적. 성능 저하
- Kafka로부터 읽은 데이터가 충분히 메모리에 쌓였을 때 커밋을 수행해야하는데,
	- 이렬 경우 Kafka에서 제공하는 자동 오프셋 커밋 기능 사용 불가. 수동으로 오프셋 관리 필요
- 내부 메모리에 오프셋을 저장하고 실제로 Iceberg 테이블에 커밋이 성공한 위치까지의 Offset만 Kafka에 commit
- at-least-once 구조
- Iceberg의 equality delete를 사용하면 중복 데이터 제거가 가능하나, Iceberg 테이블 운용 비용이 비싸짐
- 로그 유실은 문제가 될 수 있으나 중복 발생은 대부분 크게 문제가 되지 않음
- 추가적으로 모든 로그에는 unique id가 있기 대문에
	- 필요시 사용자가 쿼리로 중복 데이터 제거하도록 가이드

#### 데이터 변환
- 기본적으로 신규 필드 유입시 시스템에서 해당 필드를 String으로 취급하여 스키마 자동 업데이트
	- 사용자가 UI, API를 통해 신규 필드를 원하는 타입으로 생성 가능
- 신규 필드가 유입되면 해당 테이블에 대해 메모리에 쌓여 있던 데이터를 강제 flush
	- 스키마 업데이트 이후 다시 메모리에 데이터 적재
- 특정 필드에 대해 변환 불가능한 경우, 에러 필드에 원본 데이터와 이유를 함께 저장
- **Iceberg 테이블은 컬럼 이름의 대소문자 구분을 지원**하나
	- 쿼리 엔진이 대소문자 구분을 지원하지 않기 때문에
	- 대소문자를 구분하지 않게(case-insensitive)하게 설정해야 함
- 대소문자만 다른 이름을 가진 중복되는 필드가 유입되면 에러 필드에 저장
- 또한 String이 아닌 다른 타입으로 지정된 필드에 대해 지원되지 않는 값으로 데이터 유입시 에러 필드에 저장
- 사용자는 에러 필드를 조회해 누락된 데이터 값과 누락 사유를 알 수 있음
- 알 수 없는 이유로 데이터 변환 실패시 DLQ에 전송해 후처리 진행
- 트래픽이 증가해 데이터 적재 컴포넌트를 많은 수로 확장(sacle-out)하면
	- Iceberg 테이블에 대해 동시에 여러 노드가 Write 수행
	- 이때 다음과 같은 문제 발생
		- Iceberg 테이블에 동시에 발생한 커밋의 충돌로 인한 실패 가능성 증가
		- 여러 노드에 데이터가 분산되어 작은 파일로 쪼개어 write
		- 처리량 저하 및 object storage에서도 작은 파일로 인해 부담 발생
		- 추후 데이터 최적화를 위한 Rewrite 과정에서도 문제 발생
	- 위와 같은 문제 때문에 데이터 적재 컴포넌트가 단일 노드에서 최대한의 성능을 낼 수 있도록 최적화
	- 추후 Kafka Topic Custom Partitioner를 통한 계획도 존재
