---
date: 2024-05-11
datetime: 2024-05-11 16:38:53
book: 
page: 
tags: 
references:
  - https://airflow.apache.org/docs/apache-airflow/stable/authoring-and-scheduling/datasets.html
aliases:
---
`Dataset`을 활용하여, 데이터 기반으로 DAG를 스케줄링 한다
- Dataset이 업데이트 할 때를 기준으로 실행하도록
Airflow 2.4 부터 지원하는 형식

```python
from airflow.datasets import Dataset

with DAG(...):
    MyOperator(
        # this task updates example.csv
        outlets=[Dataset("s3://dataset-bucket/example.csv")],
        ...,
    )


with DAG(
    # this DAG should be run when example.csv is updated (by dag1)
    schedule=[Dataset("s3://dataset-bucket/example.csv")],
    ...,
):
    ...
```
- scheduling되는 부분에 `Schedule: Triggered by datasets` 관련 부분이 생김


## dataset 이란?
- 데이터의 논리적 그룹
- upstream producer는 dataset 업데이트가 가능함
- downstream consumer DAG scheduling 역할
- URI로 dataset을 정의함
```python
example_dataset = Dataset("s3://dataset-bucket/example.csv")
```
- URI가 나타내는 데이터의 내용, 위치에 대한 어떤 가정도 하지 않음
- URI를 문자열로 가정
	- regex나 file glob pattern을 지정해도 작동하지 x
- 올바른 URI를 사용하여 dataset을 만들어야 함
	- file(core), postgres, s3, third-party ...

## What is Valid URI?
- RFC 3986
- ASCII + `%, -, _, ., ~`
- URI로 식별 불가시 `%` 인코딩 사용
- case-sensitive
	- host의 경우도 case-sensitive
	- RFC 3986과 다른점
- `airflow` schema를 사용하지 x(airflow 내부용)
- schema에서 소문자 권장
```python
#invalid datasets
reserved = Dataset("airflow://example_dataset")
not_ascii = Dataset("èxample_datašet")
```
- 의미론적 제약을 포함하지 않는 데이터셋 정의시 `x-` prefix를 사용함
	- `x-`를 붙일 경우, 의미론적 검증(semantic constraints)을 건너 뜀
```python
# valid dataset, treated as a plain string
my_ds = Dataset("x-my-thing://foobarbaz")
```
- identifier는 절대적일 필요는 없으며
	- 체계가 없거나
	- 상대적인 URI이거나
	- 단순한 경로나
	- 문자열일 수 있음
```python
# valid datasets:
schemeless = Dataset("//example/dataset")
csv_file = Dataset("example_dataset")
```
- 절대적이지 않은 식별자는, 일반 문자열로 간주

## Extra information on dataset
- `dict`형태 포함
- dataset identity에 영향을 미치지 않음
```python
example_dataset = Dataset(
    "s3://dataset/example.csv",
    extra={"team": "trainees"},
)
```

```python
with DAG(
    dag_id="consumer",
    schedule=[Dataset("s3://dataset/example.csv", extra={"different": "extras"})],
):
    ...

with DAG(dag_id="producer", ...):
    MyOperator(
        # triggers "consumer" with the given extra!
        outlets=[Dataset("s3://dataset/example.csv", extra={"team": "trainees"})],
        ...,
    )
```
- 민감 정보를 Dataset URI로 지정하지 말 것
	- plaintext로 처리하기 때문

## How to use datasets in your DAGs
- dataset을 사용하여 DAG에서 data dependency를 정의할 수 있음
- producer DAG가 성공적으로 완료된 후, consuer DAG를 예약하는 방법
	- 작업이 success로 완료되었을 때만, dataset을 업데이트하도록 마킹
	- failed나 skip시에는 업데이트 발생하지 x
		- airflow가 consumer dag를 예약하지 x
```python
example_dataset = Dataset("s3://dataset/example.csv")

with DAG(dag_id="producer", ...):
    BashOperator(task_id="producer", outlets=[example_dataset], ...)

with DAG(dag_id="consumer", schedule=[example_dataset], ...):
    ...
```

## Multiple Datasets
- schedule 파라미터는 list type
- 모든 데이터 세트가 적어도 1번 업데이트 된 이후 DAG를 스케줄링
```python
with DAG(
    dag_id="multiple_datasets_example",
    schedule=[
        example_dataset_1,
        example_dataset_2,
        example_dataset_3,
    ],
    ...,
):
    ...
```

## Fetching information from a triggering dataset event
- `triggering_dataset_event` template이나 parameter를 통해
	- trigger된 데이터 세트의 정보를 가져올 수 있음
```python
example_snowflake_dataset = Dataset("snowflake://my_db.my_schema.my_table")

with DAG(dag_id="load_snowflake_data", schedule="@hourly", ...):
    SQLExecuteQueryOperator(
        task_id="load", conn_id="snowflake_default", outlets=[example_snowflake_dataset], ...
    )

with DAG(dag_id="query_snowflake_data", schedule=[example_snowflake_dataset], ...):
    SQLExecuteQueryOperator(
        task_id="query",
        conn_id="snowflake_default",
        sql="""
          SELECT *
          FROM my_db.my_schema.my_table
          WHERE "updated_at" >= '{{ (triggering_dataset_events.values() | first | first).source_dag_run.data_interval_start }}'
          AND "updated_at" < '{{ (triggering_dataset_events.values() | first | first).source_dag_run.data_interval_end }}';
        """,
    )

    @task
    def print_triggering_dataset_events(triggering_dataset_events=None):
        for dataset, dataset_list in triggering_dataset_events.items():
            print(dataset, dataset_list)
            print(dataset_list[0].source_dag_run.dag_id)

    print_triggering_dataset_events()
```
- `.values() | first | first` 등의 접근으로,
	- DAG에 제공된 하나의 데이터 세트 중 첫번째 데이터 세트
	- 해당 데이터 세트에 대한 하나의 데이터 세트 이벤트 중 첫번째 데이터 세트를 가져옴
- 여러 dataset이 있는 경우, 구현이 상당히 어려울 수 있음
	- **여러 dataset을 사용하는 것을 지양해야 할수도**

## Advanced dataset scheduling with conditional expressions
- `dataset`과 함께 조건식을 사용하는 고급 스케줄링 기능
- workflow trigger에 대한 제어를 위해 **논리 연산자**를 사용하여
	- dataset update를 기반으로 DAG 실행에 대한 복잡성 정의 가능
- `AND(&)`: 지정된 데이터셋이 모두 업데이트 된 경우 DAG 트리거
- `OR(|)`: 지정된 데이터셋 중 어떤 것이라도 업데이트 된 경우 DAG 트리거

## Example Use

### Scheduling based on multiple dataset updates
- 2개의 DAG가 모두 업데이트 된 경우 트리거
```python
dag1_dataset = Dataset("s3://dag1/output_1.txt")
dag2_dataset = Dataset("s3://dag2/output_1.txt")

with DAG(
    # Consume dataset 1 and 2 with dataset expressions
    schedule=(dag1_dataset & dag2_dataset), 
	# Consume dataset 1 or 2 with dataset expressions
	schedule=(dag1_dataset | dag2_dataset), # OR
    
    # Consume dataset 1 or both 2 and 3 with dataset expressions
	schedule=(dag1_dataset | (dag2_dataset & dag3_dataset)),
    ...,
):
    ...
```

## Combining dataset and time-based schedules

### DatasetTimetable Integration
- dataset 이벤트와 time-based schedule을 기반으로 DAG 스케줄링
- DAG가 데이터 업데이트에 의해 trigger되고, 고정된 시간에 주기적으로 실행될 때 사용
- [DatasetOrTimeSchedule](https://airflow.apache.org/docs/apache-airflow/stable/authoring-and-scheduling/timetable.html#dataset-timetable-section) 참고

#### DatasetOrTimeSchedule
- data 변경에 대한 응답성을 유지하고, 정기적인 검사 또는 업데이트를 지속적으로 수행
```python
from airflow.timetables import DatasetOrTimeSchedule
from airflow.timetables.trigger import CronTriggerTimetable


@dag(
    schedule=DatasetOrTimeSchedule(
        timetable=CronTriggerTimetable("0 1 * * 3", timezone="UTC"), datasets=(dag1_dataset & dag2_dataset)
    )
    # Additional arguments here, replace this comment with actual arguments
)
def example_dag():
    # DAG tasks go here
    pass1
```