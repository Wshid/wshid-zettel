---
date: 2024-08-17
datetime: 2024-08-17 12:48:56
book: 
page: 
tags: 
references:
  - https://aws.amazon.com/ko/what-is/apache-iceberg/
aliases:
---
- [[data_lake|데이터 레이크]]에 저장된 대규모 데이터 세트의 처리를 단순화
- Apache Spark, Flink, Hive, Presto 등과 간단히 통합 가능

## Transaction Data Lake
- 데이터 레이크: 모든 규모의 구조적, 비구조적 데이터를 저장할 수 있는 중앙 집중식 리포지토리
- 데이터 트랜잭션: 단일 작업으로 수행되는 일련의 데이터 교환
- [[Transaction DataLake]]

## Apache Iceberg의 이점
- SQL 활용 가능
- 데이터 일관성
	- 데이터를 읽고 쓰는 모든 사용자가 동일한 데이터를 바라봄
- 데이터 구조
	- 데이터 구조를 쉽게 변경할 수 있음
	- 스키마 진화
	- **사용자 기반 데이터를 건드리지 않고 데이터 테이블에서 열 추가, 변경, 제거 가능**
- 데이터 버전 관리
	- 시간 경과에 따른 데이터의 변경 내용 추적 가능
	- 사용자가 이전 버전의 데이터에 엑세스하여 쿼리하고
		- 업데이트 작업, 삭제 작업 간에 변경된 데이터를 분석하는 **시간 이동 기능**이 지원됨
- 크로스 플랫폼 지원
	- Spark, Hive, Presto 등의 다양한 스토리지 시스템, 쿼리 엔진 지원
	- 다양한 데이터 처리 환경에서 활용 가능
- 증분 처리
	- 사용자의 마지막 실행 이후 변경된 데이터만 처리할 수 있는 **증분 처리** 기능 제공
	- 이를 변경 데이터 캡처(CDC, Change Data Capture)라고 함

## Apache Iceberg의 일반적인 사용 사례
- [[data_lake|데이터 레이크]] 사용 사례에 적합
- 자주 삭제해야 하는 데이터 레이크의 데이터 테이블
- 레코드 수준 업데이트가 요구되는 데이터 레이크의 데이터 테이블
	- 데이터가 확정된 이후, 데이터 세트를 자주 업데이트 해야하는 경우
	- Iceberg는 전체 데이터 세트를 다시 게시할 필요 없이 개별 레코드를 업데이트 하는 기능 제공
- SCD(Slowly Changing Dimension) 테이블 처럼 **변경 내용을 예측할 수 없는** 데이터 레이크의 데이터 테이블
	- 시간 경과에 따라 알 수 없는 간격으로 변경될 수 있는 테이블들
- 데이터 레이크와 트랜잭션에서 데이터 유효성
	- 내구성, 신뢰성을 보장해야 하는 경우,
	- Apache Iceberg 테이블 형식을 배포하여 ACID Transaction 보장
- 추세 분석을 수행하기 위해 이전 버전의 데이터를 쿼리하거나,
	- 일정 기간 동안의 데이터 변화를 분석, 문제 해결을 위해 이전 버전으로 복원, 롤백 목적으로
	- 이전으로 시간 이동이 필요한 경우

## Iceberg의 사용자
- 데이터 엔지니어, 관리자, 분석가, 사이언티스트
- Iceberg를 사용하여 확장 가능한 데이터 스토리지 시스템 설계 및 구축 가능
- [[data_analyst|데이터 분석가]]와 [[data_scientist|데이터 과학자]]는 Iceberg를 사용하여 대규모 데이터 세트를 효율적으로 분석할 수 있음

## Iceberg를 선택해야 하는 이유
- 대규모 데이터 세트를 효율적으로 처리 가능
### 특징
- 오픈 소스
- 확장성
	- 여러 노드에 걸쳐 데이터를 분할 및 구성 가능
	- 워크로드 분산 및 데이터 처리속도 향상 가능
- 성능
	- 조건자 푸시다운 및 스키마 진화와 같은 열 기반 저장 및 압축 기술
	- 쿼리 성능 최적화하는 다양한 기능 제공
- 유연성
	- 쿼리 재작성이나 데이터 구조 재구축 없이도
		- 시간 경과에 따라 데이터가 진화할 수 있도록 데이터 구성 방식 변경 가능
	- 다양한 데이터 형식 및 데이터 소스 지원 -> 기존 시스템 쉬운 통합
- 신뢰성
	- 트랜잭션 지원으로 일관성과 신뢰성 보장
	- 시간 경과에 따른 데이터 변화 추적 및 이전 버전 롤백, 문제 해결 가능