---
tags:
  - data-quality
  - data-observability
---

# Data Observability
- https://www.montecarlodata.com/blog-what-is-data-observability/
- 데이터 및 데이터 시스템의 상태를 완벽하게 파악한다
- 아래에 대한 지표 제공
  - 데이터가 언제 잘못되었는지
  - 무엇이 잘못되었는지
  - 어떻게 수정해야하는지

### 5가지 특징
- Freshness
  - 데이터 테이블의 최신 상태
  - 테이블 업데이트 주기
  - stale data == wasted time and money
- Quality
  - data pipeline에서 '올바른' 데이터를 처리하는가
  - e.g. data의 null 비율, unique 비율, 가용한 범위 내에 있는가
  - 데이터에서 기대할 수 있는 것을, 테이블을 기반으로 신뢰할 수 있는지 통찰력 제공
- Volume
  - 데이터 테이블 완전성
  - 데이터 소스 상태에 대한 통찰력 제공
  - e.g. 200M rows -> 5M rows, 문제 발생 감지
- Schema
  - 데이터 구조의 변경 감지
- Lineage
  - 데이터가 깨질때 어떤 upstream/downstream ingestor가 영향을 받았는지
  - 어떤 팀이 데이터를 생성하고 액세스 하고 있는지
  - 좋은 lineage는 특정 데이터 테이블에 관련된 governance, business 및 메타 데이터에 대한 정보를 수집 -> 모든 소비자에게 신뢰할 수 있는 단일 source 역할

## Data observability is as essential to DataOps as observability is to DevOps
- DevOps
  - 시스템 상태에 따라 지속적인 pulse 유지. CI/CD 접근 방식 배포
- Observability of Data
  - Metric: 시간의 지남에 따라 측정된 데이터의 숫자 표현
  - Log
    - 주어진 타임스탬프에서 발생한 이벤트의 기록
    - 특정 이벤트가 발생한 시점의 context 제공
  - Trace
    - 인과 관계가 있는 이벤트

## Why is data observability important?
- DQ를 측정하는 방법
  ```bash
  DDT = N * (TTD + TTR)
  ```
  - DDT: Data downtime
  - N: Number of incidents
  - TTD: Tiem to Detection
  - TTR: Time to Resolution
  - N
    - 매년 발생하는 예상 사건 수(현재 해당 사건을 포착하는지 여부)를 간단히 계산하려면
    - 현재 table 수를 `15`로 나누면 됨
  - TTD, TTR
    - metric으로 수집하고 있지 않다면,
    - 각각 약 4시간, 9시간으로 산정하여 계산
- DDT는
  - Data Engineer/Developer; 시간과 자원의 낭비를 의미
  - Data Consumer; 의사 결정에 대한 신뢰 저하

## The key features of data observability tools
- It connects to your existing stack quickly and seamlessly
  - 데이터 파이프라인을 수정하거나 새 코드 작성 등이 필요 없음
  - 큰 투자 없이도, 가치를 평가하는데 걸리는 시간 단축, 테스트 범위 최대화 가능
- It monitors your data at-rest
- It requires minimal configuration
  - 임계값 설정은 거의 필요하지 x
  - data observabtility tool은 ML을 사용하여 자체 판단
- It requires no prior mapping
  - 어떤 방식으로 모니터링해야하는지 매핑이 필요 없음
- It provides rich context
  - 문제 해결을 가능하게 하는 컨텍스트
- It prevents issues from happening in the first place
  - 데이터 자산에 대한 풍부한 정보 노출
  - 능동적으로 변경/수정 가능

## Data observability vs. data testing
- ![image](https://github.com/Wshid/daily-poc/assets/10006290/708d341c-258a-479b-bef7-65d27a59ad0d)
- data testing은 reliability stack의 일부
- Data Engineer의 엄격한 테스트에도 DQ에 문제가 발생하는 사례가 많음
  - DQ는 예측 할 수 있는 문제/없는 문제로 나뉘기 때문
- DO에서 다루는 unknown example
  - 업데이트 되지 않는 대시보드/보고서
    - 오래된 데이터는 감지되지 않음
  - 조직의 코드베이스가 변경되었을 때, API가 Tableau 대시보드에 데이터 수집 진행 x
  - JSON 스키마의 변경 실수
  - ETL/ELT/reverse ETL에서 의도치 않은 변경 발생으로 일부 테스트가 진행되지 x
  - 최근 n년 동안 파이프라인의 일부였으나, 현재 Business Logic을 반영하지 못한 경우
- data observability is different and more effective than testing 
  - because it provides end-to-end coverage, is scalable, and has lineage that helps with impact analysis. 

## Data observability vs. Data Monitoring
- ![image](https://github.com/Wshid/daily-poc/assets/10006290/337021b8-ad3b-4b20-a601-51184f7a9979)
- data pipeline monitoring -> data pipeline이 일반적으로 동작하는 방식 이해 이후, 경고 발생
- 모니터링은 관찰의 일부
- 데이터 값의 정상범위, 데이터 자체 모니터링을 통해 데이터 관찰이 가능해짐
  - 데이터 자체, BI 계층에서 소비에 이르기 까지 **전체 파이프라인을 end-to-end로 모니터링 해야함**

## Data Observability vs Data Quality
- DO는 DQ를 활성화 하거나 향상시킴
- Data Quality
  - Completeness
  - Consistency
  - Timeliness
  - Validity
  - Uniqueness
- DQ는 보통 binary metric으로 평가됨
  - Your CFO doesn’t come up to you and say, “the data was accurate but out of date so I’m considering it to be of average quality.”
  - 데이터가 정확하나 out of date 되기 때문에 DQ 평균 값을 가짐
- DQ는
  - 데이터 전문가가 데이터가 작동하지 않는 부분과, 어디에 집중하는지 파악하는데 도움이 되나,
  - 데이터 소비자에게는 DQ가 좋거나 좋지 않다고 판단됨
- **이에 따라 data downtime 지표를 만듦**

## Data quality vs data reliability
- ![image](https://github.com/Wshid/daily-poc/assets/10006290/c58fa076-8cd9-42c4-b6eb-25253adc8d5f)
- 시간에 따라 Quality가 어떻게 변화하는가?
  - Data Reliability
- Data Reliability(신뢰성)을 해결하려면
  - 단순히 현 시점과 공간에 대한 평가가 아닌
  - **예상되는 품질 + 서비스 수준**을 설정하고
    - **데이터 사고를 신속하게 진단하고 해결하 수 있는 toolkit 확보 필요**

## Data observability vs Data Reliability Engineering
- SRE(Site Reliability Engineering)와 유사

## The future of Data Observability
- Google Trend data상 우상향 중
- 최근 data stack의 핵심 기술
- whether it’s enabling more self-service analytics and data team collaboration, adoption, or working alongside **dbt unit tests** and **Airflow circuit breakers** to prevent bad data from entering the data warehouse (or data lake) in the first place
- It is also a core component of important and emerging **data quality** best practices such as `data mesh, data SLAs, and data contracts`.


## 추가 파악 가능한 단어
- data mesh
- data SLA
- data contract
