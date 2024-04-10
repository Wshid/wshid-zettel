---
tags:
  - trino
---

# Trino w/ Hive Connector
- https://trino.io/docs/current/connector/hive.html
- Trino는 Hive의 Data, MetaData만을 사용함
  - HiveQL, Hive의 실행 환경의 어떤 부분도 사용하지 x

## Multiple Hive Cluster
- 여러 Hive Cluster에 연결이 필요할 경우
  - `etc/catalog`에 `.properties`로 끝나는 이름으로 기록하면 됨
  - e.g. `sales.properties`로 입력하면, `sales`라는 카탈로그 커넥터가 생성됨

## HDFS Configuration
- Trino는 HDFS를 자동 구성, 구성 파일이 필요 x
- 몇 케이스에 대해 federated Hdfs | NameNode에 HDFS Client 옵션이 필요할 수 있음
  ```properties
  hive.config.resources=/etc/hadoop/conf/core-site.xml,/etc/hadoop/conf/hdfs-site.xml
  ```
- 구성 파일을 **최소화**하는 것이 좋음
  - 추가 속성으로 문제가 발생할 수 있기 때문
- Configuration File은 **모든 trino node**에 있어야 함
  - Hadoop config file을 참조하는 경우 해당 파일을 모두 복제해주어야 함

## HDFS username and permissions
- `CREATE TABLE`, `CREATE TABLE AS` 구문을 사용하기 전에
  - Trino가 `HDFS`에 접근이 가능한지
  - Hive warehouse 접근이 가능한지 확인이 필요함
- Hive warehouse 경로
  - `hive.metastore.warehouse.dir` in `hive-site.xml`
  - default: `/user/hive/warehouse`
- HDFS에 kerberos를 사용하지 않는 경우 `Trino Process`의 OS User를 기준으로 HDFS에 접근
  - 다음 옵션으로 user를 변경할 수 있음
  - `-DHADOOP_USER_NAME=hdfs_user`

## Hive general configuration properties
- hive.ignore-absent-partitions(default: false)
  - 파일 시스템 위치가 존재하지 않을때 쿼리 실패가 아닌 파티션을 무시함
- hive.storage-format(default: ORC)
- hive.immutable-partitions(default: false)
  - 이미 있는 파티션에 데이터를 insert하는 경우를 방어함
  - `hive.insert-existing-partitions-behavior=APPEND` 불가
- hive.insert-existing-partitions-behavior(default: APPEND)
  - 이미 존재하는 파티션에 대해 insert 작업을 수행할 경우
    - APPEND, OVERWRITE, ERROR
- hive.target-max-file-size(default: 1G)
  - 새로운 파일의 최대 크기
- hive.create-empty-bucket-files(default: false)
  - bucket의 데이터가 없을 경우에도 empty file을 생성할 것인지 여부
- hive.validate-bucketing(default: true)
  - 데이터가 올바른 bucket에서 읽어졌는지를 확인함
- hive.partition-statistics-sample-size(default: 100)
  - 데이터 계산시 분석할 파티션 수
- hive.max-partitions-per-writers(default: 100)
  - writer에서 사용할 최대 파티션 수
- hive.max-partitions-for-eager-load(default: 100,000)
  - 단일 테이블 검색에서 coordinator 로드당 최대 파티션 수
  - 특정 최적화는 빠른 로딩 없이 불가능
- hive.max-partitions-per-scan(default: 1,000,000)
  - 단일 테이블에서 스캔할 최대 파티션 수
- hive.non-managed-table-writes-enabled(default: false)
  - hive에서 external 테이블에 작성가능할지 여부
- hive.non-managed-table-creates-enabled(default: false)
  - hive에서 external 테이블을 생성 가능할지 여부
- hive.collect-column-statistics-on-write(default: True)
  - write시 automatic column level 통계를 활성화
- hive.file-status-cache-tables
  - 특정 테이블에 대해 캐시할 디렉터리 지정
  - `fruit.apple`, `fruit.orange`
    - `fruit` 스키마에 대해 apple, orange 캐싱
  - `fruit.*`
    - `fruit` 스키마내 모든 테이블 캐싱
  - `*`: 전체 캐싱
- hive.file-status-cache.max-retained-size(default: 1G)
  - 캐시 파일을 유지할 크기
- hive.file-status-cache-expire-time(default: 1m)
  - 캐시 디렉터리를 유지할 기간
- hive.per-transaction-file-status-cache.max-retained-size(default: 100M)
  - transaction file status 상태에 있는 모든 파일의 보존 크기
  - 현재 수행 중인 모든 쿼리에서 공유됨
- hive.rcfile.time-zone(default: JVM default)
  - 인코딩된 타임스탬프 값을 특정 시간대로 조정
  - `Hive 3.1+`에서는 `UTC`로 지정
- hive.timestamp-precision(default: MILLISECOND)
  - timestamp 컬럼의 precision
  - `MILLISECONDS|MICROSECONDS|NANOSECONDS`

## EXTERNAL TABLE using Hive
- external_location 옵션 사용
- https://www.starburst.io/community/forum/t/what-s-the-difference-between-location-and-external-location/105
- https://trino.io/docs/current/connector/hive.html#basic-usage-examples

## Performance tuning configuration properties

### Tuning Options
- https://www.starburst.io/community/forum/t/number-of-splits-using-the-hive-connector/363
- hive.max-outstanding-splits(default: 1000)
  - 스케줄러가 중단되기 전 쿼리내 각 테이블을 스캔에 대한 buffered split의 수
- hive.max-outstanding-splits-size(default: 256M)
  - query가 실패하기 전 쿼리내 각 테이블을 스캔에 대해 버퍼링된 분할 최대 크기
- hive.max-splits-per-second(default: X)
  - 테이블 스캔당 초당 생성되는 **최대 분할 수**
  - 이를 사용하여 스토리지 시스템의 부하를 줄일 수 있음
  - 기본값이 없기 때문에 **trino는 data access의 parallelize를 극대화함**
- hive.max-initial-splits(default: 200)
  - 테이블 스캔당 coordinator는 시작시 `max-initial-splits-size`에 맞게 file section을 할당
  - `max-initial-splits`가 할당된 후, 나머지는 `max-split-size`에 맞게 사용
  - `# 초기 파티션수와 같은 의미일까?`
- hive.max-initial-split-size(default: 32MB)
  - `max-initial-splits`이 할당될 때까지 worker에게 할당된 single file section의 크기
  - 분할이 작을 수록 **병렬성**이 높아지므로, 더 작은 쿼리가 boost됨
- hive.max-split-size(default: 64MB)
  - worker에게 할당된 single file section의 가장 큰 크기
  - 분할이 작으면 병렬성이 향상되므로, 지연시간을 줄일 수 있으나
    - overhead가 증가하고 시스템 부하가 증가할 수 있음

### Hive Tuning Example
- https://trino.io/docs/current/connector/hive.html#performance-tuning-configuration-properties

#### 105개의 파일 50MB -> 205개
- `max-initial-split-size`보다 기존 파일들의 크기가 큰 상태
- 32M으로 맞춰서 32 + 18로 분할
- 32MB 100개, 18MB 100개, 50MB 5개
- 200개는 `max-initial-splits`에 맞게 할당
- 이후 5개는 `max-split-size`(50MB)에 부합하므로 잔여 5개 할당
- 총 205개
