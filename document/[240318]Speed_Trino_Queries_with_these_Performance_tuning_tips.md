---
tags:
- trino
- performance
---

# Speed Trino Queries with These Performance-Tuning Tips
- https://thenewstack.io/speed-trino-queries-with-these-performance-tuning-tips/

## 1. Performance Bottlenecks for Trino Queries

### Compute Resources
- 데이터 처리를 위해 많은 CPU를 사용
- source, 중간 결과, 최종 출력을 저장하기 위해 memory 소비
- workload, H/W resource를 염두하고
  - 각 노드의 CPU 코어수, memory 균형을 잡는 것이 중요

### I/O Speed
- Storage-independent query enigne
  - local disk에 데이터 저장 x
- 대신 external storage engine에서 데이터를 가져옴
- storage system/network 속도에 크게 영향을 미침

### Table Scans
- Connector에서 데이터를 가져와 다른 작업자가 사용할 데이터를 생성하는 구조
  - 대규모 데이터 세트 사용시 **성능 병목** 발생
- 테이블 분할 방식, Parquet/ORC 등의 선택은 쿼리 속도에 큰 차이 발생

### Joins
- 가장 resource-intensive한 action
- join을 효율적으로 수행할 것

## 2. Process of Optimizing Trino
- ![image](https://github.com/Wshid/daily-poc/assets/10006290/d5fbed30-fba0-4938-bd24-5765feee800d)
- Trino Web UI를 통해 전체 클러스터 속도가 느려지는가
- Trio Web UI가 쿼리에 대기열에 있거나 block 되는가
- `EXPLAIN ANALYZE`를 통해 slow query의 병목현상 식별

### EXPLAIN ANALYZE
```sql
EXPLAIN ANALYZE select * from customers limit 5;
```
- 실행 계획 및 병목 지점 파악 가능

## Tuning Tips for Trino

### Tip 1: Optimize Resource Allocation
- 충분한 량의 memory가 있어야 정상 구동 가능
- maximum memory & number of concurrent queries 조정 가능
- 쿼리의 최대 메모리, 동시 실행 쿼리 수 균형 필요
- Step 2에서와 같이, 
  - 쿼리가 차단될 경우
    - 메모리를 늘리거나, 동시 쿼리의 최대 수를 줄이기
  - 대기열 쿼리가 많이 발견되면
    - CPU 리소스를 더 추가하거나, 동시 쿼리를 줄이기

#### 옵션
- query.max-memory-per-node
  - `JVM heap size / max_concurrent` 권장
  - 값이 높으면 큰 쿼리의 속도가 빨라짐
  - 값이 낮으면 memory 할당 deadlock 위험이 낮아짐
- query.max-memory
  - `query.max-memory-per-node`가 올바르게 설정되었다면
  - 해당 설정 하지 x
- query.low-memory-killer.policy
  - `total-reservation` 권장
  - memory allocation deadlock이 발생할 때
  - 가장 메모리를 많이 사용하는 쿼리를 제거할 것

### Tip 2: Improve I/O (Storage and Network)
- Trino는 Storage Independent Query Engine
- 각 쿼리에 대해 원격 저장소에서 데이터를 가져옴
- I/O throughput이 낮거나, network latency가 높다면
  - 쿼리가 queued되거나 blocked 될 수 있음
- queued and blocked query에 대해 I/O를 개선하기

#### I/O 개선 방법
- Use Faster storage
  - hotter storage tier of Amazon S3
- Reduce network latency
  - Trino와 Storage System간의 Netowrk latency 줄이기
- Caching
  - Caching Layer를 사용하기
  - Alluxio와 같이 Caching Layer를 구축하면
    - remote storage와 trino와 다른 컴퓨팅 엔진간의 데이터 공유간 쿼리 지연 시간 줄일 수 있음
  - Trino worker의 Data locality 확보

### Tip 3: Table Scan Optimization
- `EXPLAIN ANALYZE`
  - File Format, Compression, Partition, Bucketing, Sorting, ...

#### Columnar data file formats and compression
- footer of the files로부터 metadata에 먼저 접근
  - columnar data를 읽음
  - 파일의 구조와 data section의 위치를 지정
- trino는 columnar data를 효율적으로 처리하기 위해 많음 thread 사용
- columnar format은 아래 방법으로 성능 최적화 가능
  - skipping unnecessary columns
  - predicate push down
- ORC, PARQUET의 경우 predicate push down, efficient data compression 지원
- ORC가 종종 PARQUET보다 성능이 뛰어나나,
  - PARQUET의 성능을 향상시키기 위한 노력이 community에서 이루어지고 있음

#### Flat table column layout and dereference pushdown
- Trino 334 이후 **Dereference pushdown**을 도입함
  - query nested column의 비용 감소
- 만약 dereference pushdown의 이점이 없다면 flat table column layout을 사용할 것

#### Partitioning and bucketing
- Partition 기준으로 열 분할시, 쿼리 성능 향상
  - 불필요한 파티션에 데이터를 접근하지 않기 때문에
- 하지만 파티션이 매우 많은 경우
  - slow down planning
  - put pressure on storage
- Bucketing
  - a form of hash partitioning
  - table을 선택된 열을 기준으로 여러개의 hash bucket으로 나누는 방식
  - partition의 문제 관리시 도움이 됨

#### Partitioning and bucketing example
```sql
CREATE TABLE customers 
  customer_id bigint,
  created_day date
)
 
WITH (
  partitioned_by = ARRAY[created_day],
  bucketed_by = ARRAY['customer_id'],
  bucket_count = 100
)
```


### Tip 4: Join Optimization
- `EXPLAIN ANALYZE`를 사용하여 **slow join**도 식별 가능
- Trino의 CBO(Cost Based Optimizer)는 Table statistics를 기준으로 효과적인 조인 방법 결정
- Join Optimization을 위해
  - Consider join distribution types
  - join orders
  - dynamic filtering(가능한 경우)

#### Probe Table / Build Table
- Trino는 Hash Join Algorithm을 사용
- 한 테이블은 memory로 읽히고, 다른 하나는 table을 스캔하면서 재사용
- 일반적으로 메모리를 절약하고, 동시성을 높히기 위해
  - **작은 테이블을 build table로 선택**

#### Join Distributions Type(2)
- Partitioned Join
  - 각 노드는 build table의 데이터 중 일부에서 hash table 생성
  - 상대적으로 느림
  - 노드상 메모리 요구사항이 낮음
- Broadcast Join
  - 모든 데이터로부터 해시 테이블 구축
  - 빌드 테이블의 데이터를 각 노드에 복제
  - 상대적으로 빠름
  - 노드상 메모리 요구사항이 높음
    - Build Table이 메모리에 맞도록 해야 함
- trino는 자동으로 적절한 **join distribution strategy**를 사용
- `join_distribution_type` session property를 사용하여 변경 가능

#### Optimize Join Order
- 저장소에서 읽히는 데이터와, 작업자 간의 **데이터 전송 최소화**하기 위함
- Trino의 cost-based-join enumeration estimation cost는
  - 서로 다른 join order의 비용을 추정하고
  - 추정 비용이 제일 낮은 것을 선택
- **join order strategy**는 `join_reordering_strategy` session property를 사용하여 변경 가능


#### Dynamic Filtering 활성화
- ![image](https://github.com/Wshid/daily-poc/assets/10006290/db37309b-a415-47ca-90b1-7609b75511cc)
- probe table에서 읽는 불필요한 행의 수를 줄이는 방법
- Trino는 broadcast join동안 `build`측에서 생성한 **dynamic filter**를 적용하여
  - `probe table`데이터를 줄임
- 기본적으로 활성화되어 있는 옵션이며
  - `enable_dynamic_filtering` session property를 사용하여 변경 가능

## 4. Summary
- production에 trino를 사용할 때는 아래 사항에 유의할 것
  - Resource Allocation
  - I/O Improvement
  - Table Scan/Join Optimization
- Performance Tuning은 일회성 작업이 아님
  - 사용자의 특정 사용 사례, workload pattern에 따라
  - 정기적인 점검, 테스트 및 변경이 필요한 지속적인 프로세스
