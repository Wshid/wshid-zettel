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

## 데이터 최적화
- 데이터 최적화 컴포넌트의 역할
	- Iceberg 테이블 데이터 최적화 및 라이프사이클 관리
	- Iceberg 테이블 관련 API 제공
- 데이터 최적화를 진행하지 않으면?
	- Iceberg 테이블에 쌓이는 파일의 수가 너무 많아짐 -> 성능 저하 유발
- 데이터 최적화, 라이프사이클 관리를 위한 task를 지속 수행 -> 테이블 상태를 최적으로
- API 서버로부터 Iceberg 테이블에 관련된 메타 데이터 정보, 스키마 업데이트 등을 요청받아 처리하는 역할도 수행
- 데이터 최적화 컴포넌트는 **임베디드 분산 캐시**를 내장하고 있음. 캐시에 따라 노드 리더 선출
- 리더로 선출된 노드는 수행해야할 테스크를 주기적으로 스케줄링, 내부 시스템 테이블로 생성
	- 시스템 테이블도 Iceberg 테이블 기반 생성
- 팔로워 노드는 시스템 테이블이 업데이트 될 때 자신에게 할당된 테스크를 읽어 실행
- ![Image](https://github.com/user-attachments/assets/52091fbe-71bb-493b-9a43-2bcb7163a7af)
- ![Image](https://github.com/user-attachments/assets/0e0ff1b4-b611-41ee-9805-d8c63a40e31f)

### Iceberg 최적화, 라이프 사이클 관리를 위한 Batch Job

#### Rewriting data
- 같은 시간 파티션 안에 있는 파일 병합
- 데이터 적재시 5분 주기로 데이터 flush 실행
	- 실제 테이블에 쓰인 데이터는 작은 파일로 나누어짐
- 오랜 시간 데이터 적재 -> 메타 데이터 거대화 -> 커밋 성능 저하
	- 작은 파일에 대한 I/O 증가, 쿼리 성능 저하
	- 지속적인 파일 쓰기 -> 오브젝트 스토리지에 부담
- **파일을 주기적으로 병합하는 작업 필요**
- 테이블은 시간 파티션으로 분할됨
- 시간 순서 유입에 따른 테이블
	- 같은 시간 파티션 안에 존재하는 파일 병합 작업을 **매시간** 실행
		- 목표 파일 병합 크기 = `128MiB`
		- 트래픽이 많은 일부 테이블을 제외하고, 대부분 1시간 로그 데이터는 1개의 파일로 병합
- 시간 순서 유입이 아닌 과거 시간의 로그와 뒤섞인 데이터(disorder data) 인입 테이블
	- 긴 시간 범위에 대해 병합 진행
	- 최대 3일 전의 로그까지 수집하는 것을 정책상 허용
	- 이때 3일 이내의 시간 파티션에 계속 작은 파일이 생성되는 문제 발생
		- 해당 테이블에 대해서는 3일 내의 모든 파티션을 병합하는 작업 실행
- 데이터 적재 지연이 발생하거나 제대로 된 정보 수신이 되지 않을 경우, 작업 스케줄링 중단
	- 데이터 지연을 고려하지 않으면 이미 병합이 종료된 파티션에 다시 작은 파일이 생성 -> 문제 유발 가능성 존재

#### Expire snapshot
- 주기적으로 snapshot 제거
- Iceberg 테이블은 매 커밋마다 snapshot 정보를 남김
- snapshot 미관리시 무한대로 snapshot이 생성됨 -> 메타 데이터 파일이 매우 커짐 -> 작은 파일이 쌓이는 문제 발생
- 주기적으로 테이블마다 최대 10개만 남기고 제거

#### Optimize table
- [[#Rewriting data]]와 [[#Expire snapshot]]을 하나의 Job으로 구성
- 파일 병합이 종료된 이후 snaphost 제거

#### Retention
- 보존 기간이 지난 로그 삭제
- 각 테이블마다 로그 보존 기간이 있음
- 보존 기간이 지난 로그를 하루에 한번씩 삭제

#### Delete Table
- 삭제 요청이 있는 Iceberg 테이블을 물리적으로 삭제
- 삭제 요청이 있은 시점으로부터 3일(데이터 복구 가능 기간)이 지난 후에 실행
- Iceberg SDK가 제공하는 삭제 API 실행 + 실제 스토리지 데이터 제거 작업 필요(garbage data 잔여 가능성)
	- S3 API를 통해 해당 테이블 디렉터리 하위에 존재하는 모든 파일에 대한 삭제 진행

#### Delete orphan files
- Garbage data를 삭제하는 작업
- Iceberg 테이블에 데이터 커밋시 충돌 발생 -> 메타 데이터, 데이터 영역 모두 garbage data 생성 가능
- 메타파일과 실제 오브젝트 스토리지에 존재하는 파일을 주기적으로 대조 및 삭제
- 작업 실행중 신규 파일이 커밋되면, 신규 파일 삭제 위험성이 있기 때문에
	- 생성된지 7일 이상 지난 파일에 대해서만 garbage 데이터 분류 실행

### 리더노드의 Batch Job 수행
- 리더 노드는 주기적으로 Batch Job 수행
- task는 각 테이블의 평균 사이즈를 기준으로 [[bin packing]] 방식으로 모든 노드에 할당
- 할당된 결과는 Iceberg 시스템 테이블에 저장, 각 노드는 해당 system 테이블을 주기적으로 읽어 자신에게 할당된 task 실행
- 실행이 완료된 task는 시스템 테이블에서 삭제
- task scheduling 상태가 iceberg 테이블로 저장되어 있기 때문에
	- 노드가 재시작되더라도 작업을 이어서 수행 가능

## 카탈로그와 데이터 연동
- Iceberg REST catalog를 사용
- REST catalog의 handler 등의 구현체는 Iceberg SDK에 포함
- SDK를 기반으로 Spring boot로 래핑하여 서버로 작동하게 만든 것 -> Puffin
- REST Catalog를 사용하려면, 실제 메타 데이터 저장소가 필요 -> MYSQL을 backend catalog로 지정하여 사용
- Alaska의 초기 설계부터 카탈로그를 사용자에게 제공하여 데이터 연동 지원하려고 했었음
	- 로그에 포함되어 있는 데이터를 분석하려는 사용자가 다수
		- 기존 로그 모니터링 시스템 환경에서는 Open API를 통한 로그 다운로드하여 사용
	- 그렇기 때문에 카탈로그를 사용자에게 제공하면 사용자는 데이터 다운로드 없이
		- 자신의 쿼리 엔진과 직접 연동해 바로 SQL 쿼리 실행 및 데이터 분석 가능
- 데이터 연동을 위한 카탈로그에 다음과 같은 기능 구현
	- 기존 로그 모니터링 시스템에서 발급받은 access key, 인증 시스템과 연동(authentication)
	- 인증된 정보를 기반으로 권한이 있는 테이블에만 접근할 수 있도록 제(authorization)
	- 데이터 연동시 read-only API에만 접근 허용, 테이블에 커밋 및 삭제 실행 불가하도록
- 인증 기능은 Iceberg REST catalog 표준 스펙에 맞춰 구현,
	- `iceberg.rest-catalog.oauth2.token`의 access key 값을 통해 사용자가 권한 있는 테이블에 읽기 전용 접근하도록 구성
	- ![[Pasted image 20250412140330.png]]

## 데이터 쿼리
- 신규 로그 모니터링 시스템의 쿼리 엔진 -> Trino
- Trino에 의존성을 가지지 않도록 내부 구조를 설계했기 때문에
	- Spark와 다른 쿼리 엔진으로 변경 가능
- 사용자는 Web UI, Open API 등으로 쿼리 수행 가능
- 신규 모니터링 시스템에서는 기본적으로 다음과 같이 쿼리를 Main Query, Sub Query로 구분

### Main Query와 Sub Query
- Main Query
	- Query for the data source
	- Work asynchronously
		- User requests a query
		- API Server validates syntax and expected resource consumption
		- The requested query is saved in MySQL
		- Leader node schedules the submitted query
		- Assigned node executes the query
- Sub query for query results
	- Subsequent query for the query result
	- Work synchronously(User interactive)
		- User requests a query
		- API Server executes the query via Trino
- Main Query는 원본 로그 데이터 테이블을 대상으로 실행하는 쿼리
	- 기본적으로 비동기로 실행하며, CTAS로 쿼리 실행. 쿼리 결과는 또 다른 테이블로 저장
- Sub Query는 메인 쿼리에 의해서 생성된 쿼리 결과 테이블을 대상으로 실행
	- 동기 방식으로 실행
- Main Query의 비동기 방식 이유
	- 대용량의 데이터를 검색할 때 실행시간이 매우 길어질 수 있기 때문
	- 일반적으로 인덱스가 없기 때문에 Elasticsearch 보다 응답 속도가 느림
	- 장기간 검색을 허용하기 때문에 검색하는 데이터 양, 쿼리 형태에 따라 결과를 얻는데 수시간이 소요될 수 있음
	- 이런 상황에서 동기 방식으로 쿼리를 실행하면 사용자는 무한정 대기
	- Main Query를 통해 최대한 관심있는 데이터 영역만 필터링해 쿼리 결과를 만들면
		- 그 이후부터는 관심 있는 데이터 영역 탐색 가능
	- [[#새로운 타입의 데이터 저장 스토리지 필요성]]의 내용 처럼 장기보관 데이터에 대해서는 쿼리 발생 비율이 낮음
		- 그렇기 때문에 Trino 클러스터를 적은 리소스로 제공
		- 다만, 신규 SQL 쿼리 기능 도입으로 이전에 없던 쿼리 패턴이 등장해 쿼리 리소스가 과도하게 소모될 가능성 존재

#### 쿼리 제약 사항
- 테이블마다 Main Query는 동시에 최대 1개만 수행
- Sub query의 실행 속도를 사용자마다 15 queries/min으로 제한
- 쿼리를 [ANTLR 4](https://github.com/antlr/antlr4)기반으로 분석해 SQL 문법을 제한함
	- SELECT 쿼리만 허용
	- WITH, JOIN, UNION, INTERSECT, EXCEPT 연산자 사용 불가
	- 중첩 쿼리(nested query) 사용 불가
	- FROM 절에는 반드시 한 개의 대상 테이블만 명시
- SQL 쿼리 실행시 사용되는 리소스 제한
	- 사용자 쿼리 요청시 바로 실행하지 않고, Query Planning을 통해 리소스 예측
	- 예측된 리소스가 제한 값을 초과하면 사용자에게 에러 반환
- 쿼리 제약이 없다면 무거운 데이터 분석 쿼리로 쿼리 엔진 비용의 급속 증가 가능성 존재
- 제약 사항을 넘어서는 쿼리 수행이 필요한 경우
	- 카탈로그 데이터 연동을 통해 사용자의 쿼리 엔진 리소스를 사용하도록 안내

## 신규 로그 모니터링 시스템 적용 결과
- Iceberg의 신규 로그 모니터링 시스템 오픈 후
	- 기존 Elasticsearch 기반의 로그 데이터는 최대 14일간만 보관
	- 2,000대의 ES 노드 감소
	- 데이터 용량 감소: 수 PB -> 수백 TB
- 비용 절감 사유?
	- 상대적으로 비용이 저렴한 오브젝트 스토리지에 데이터 저장
	- Parquet 데이터 포맷에 zstd 압축을 적용해 데이터 압축률이 높음
		- 전체 평균 원본 데이터 대비 6% 수준으로 압축
	- 쿼리 엔진 리소스를 분리해 최소한의 규모로 운영
		- 기존 ES 기반의 모니터링 시스템의 Warn 계층의 데이터 노드와 비교해 Trino 클러스터 규모가 더 작음

## 마치며
- 데이터 쿼리 패턴 분석시, 대규모 데이터의 70%는 검색이 거의 이루어지지 않았던 cold data
- 이런 데이터를 고비용, 고성능 저장소인 ES에 저장해야할지 검토
- 새로운 로그 모니터링 시스템, Iceberg라는 오픈 테이블 포맷 기반 구성
- 오브젝트 스토리지에 저장하는 기술 개발, Trino Query engine에 기반해 로그 시스템 구축
- 새로운 저비용의 로그 모니터링 시스템, 인프라 비용 절감
	- 새로운 비용의 SQL 로그 검색/분석 기능 제공
- Iceberg의 오픈 테이블 포맷을 사용한 데이터 저장은 데이터 분석 플랫폼에서 흔하게 쓰이는 방식
	- 하지만 로그 모니터링(observability) 측면에서 기존 로그 모니터링 시스템의 요구사항 만족이 어려웠음
	- 데이터 적재 및 최적화를 위한 오픈소스 활용이 어려워 Iceberg SDK를 사용해 직접 컴포넌트 개발
	- Iceberg SDK의 레퍼런스 부족, 데이터를 시간 단위 나누어 저장하고 다시 병합하며
		- 메타 데이터 관리에서는 많은 어려움이 있었음
	- 또한 신규 시스템 트래픽이 사내 오브젝트 스토리지 시스템의 부하 발생 유발
	- 하지만 컴포넌트 자체 개발로 특정 엔진에 제한되지 않고, 최신 Iceberg 버전을 적용할 수 있다는 점은 매우 큰 장점
