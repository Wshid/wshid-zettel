# Improved Hive Bucketing
- https://trino.io/blog/2019/05/29/improved-hive-bucketing.html
- Flexible Bucketing
  - 0을 포함한 bucket당 임의의 수의 파일 허용
  - 전체 파티션 재작성 필요 x
  - 기존 파티션에 파일 추가 가능
  - 빈 버킷에 대한 파일 생성 필요 x
    - 쓰기 성능 향상

## Hive Bucketiong Overview
- Hive Bucketing = Hash Partitining의 간단한 형태
- 테이블은 고정된 개수의 hash bukcet으로
  - 하나 이상의 열에 bucketing 가능
- 예시
  ```sql
  CREATE TABLE page_views (
  user_id bigint,
  page_url varchar,
  dt date
  )
  WITH (
    partitioned_by = ARRAY['dt'],
    bucketed_by = ARRAY['user_id'],
    bucket_count = 50
  )
  ```
- bucketing은 **각 파티션 내에서** 발생함
  - 파티션된 테이블이 아닐 경우, 전체 테이블에 대해 진행
- 위 예시는,
  - 테이블은 **날짜별**로 파티션 되고
  - `user_id`를 사용하여 50개의 bucket을 가짐
  - 각 행에 대해 할당된 버킷은 `user_id`를 해싱하여 결정
  - **동일한 user_id**는 동일한 bucket에 들어가게 됨

## Original Hive Bucketing
- Hive에서는 **버킷:파일 = 1:1**
- 파일 이름
  - 버킷 번호가 **파일 이름의 사전 순서내**에서, 
  - 파일의 위치를 기반으로 implicit 되도록 함
- 예시, 각각 버킷의 번호가 `0 ~ 2`를 의미함
  ```bash
  # Example 1
  00000_0
  00001_0
  00002_0

  # Example 2
  file0
  file3
  file5

  # Example 3
  bucketA
  bucketB
  bucketD
  ```
- 파일 이름은 다른 파일 이름들과 비교할 때
  - **순서**를 제외하고는 의미가 없음

## Improved Hive bucketing 
- 새로운 버전의 Hive는
  - **버킷 번호**가 **파일 이름**에 포함됨
- 기존 Hive에서 사용했던 방식과 동일한 명명 방식
- 명명 규칙
  - **버킷 번호**를 파일 이름의 시작으로 가지며
  - 그 번호가 `0`으로 시작해야 함
- 예시: 버킷 번호가 4일 때
```bash
000000_0            # bucket 0
000000_0_copy_1     # bucket 0
000000_0_copy_2     # bucket 0
000001_0            # bucket 1
000001_0_copy_1     # bucket 1
000003_0            # bucket 3
```

### 과거 Prestro의 이슈
- Presto는 **사전 순서 지정 요구 사항**에 따라
  - 유효한 **다른 이름 지정 규칙**을 사용했지만
  - **명시적 번호 지정 규칙**은 사용하지 않음
- 예시: Presto가 작성한 파일 이름
  ```bash
  20180102_030405_00641_x1y2z_bucket-00234
  ```
  - `20180102_030405_00641_x1y2z`: Presto Query ID
  - `bucket-{padding}`: 버킷 번호

### 개선된 Presto
- 예시: 개선된 Presto가 작성한 파일 이름
```bash
000234_0_20180102_030405_00641_x1y2z
```
- 시작 부분에 버킷 번호가 존재
- 마지막에 query ID가 명시됨
- 버킷 테이블을 읽을 때 `Presto`는 아래 규칙들을 모두 지원
  - 새로운 Hive 규칙
  - 이전 Presto 규칙
- 또한 파일의 명명 규칙중, 어느것과 일치하지 않아도
  - 원래의 `Hive 체계`를 지원하여
  - **버킷당 하나의 파일이 있어야 한다는 요구사항**을 유지함

## Skipping empty buckets for faster writes
- Hive, Presto는
  - **empty bucket**에 대한 파일 생성을 필요로 하지 x
- 이전 버전과의 호환성을 위해 **기본적으로 생성**
- 향후 릴리즈에서는 생성하지 않도록 변경 예정
- 사용자 환경에 적합할 경우 각 설정에 맞게 다음과 같이 지정 가능
  - configuration property: `hive.create-empty-bucket-files` 
  - session property: `create_empty_bucket_files`
