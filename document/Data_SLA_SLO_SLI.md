---
date: 2024-04-30
datetime: 2024-04-30 21:40:33
book: 
page: 
tags: 
references:
  - https://www.montecarlodata.com/blog-how-to-make-your-data-pipelines-more-reliable-with-slas/
  - https://www.atlassian.com/incident-management/kpis/sla-vs-slo-vs-sli
aliases:
---
# Why You Need to Set SLAs for Your Data Pipelines
- 다른 분야에서는 SRE, SLA, SLI, SLO 등의 방법을 통해
  - application downtime을 줄임
- 데이터 분야에서도 서비스 수준을 고려해야함
  - data downtime을 줄이는 방법
- Data Reliability의 우선순위를 정하고 표준화하며 측정
  - 데이터 팀이 이를 수행해야 함

## SLA, SLO, SLI
- Service Level Agreement
  - 고객, 사용자와의 협의
- Service Level Objective
  - SLA내에서 가동시간, 응답 시간과 같은 특정 메트릭에 대한 합의
- Service Level Indicator
  - SLO에 대한 준수 측정
  - e.g. SLA에서 99.5% 시간 동안의 사용가능한 시스템
    - SLO는 99.5%를 의미
    - SLI는 실제 가동시간의 측정값

## What is an SLA?
- 제공할 서비스 수준을 정의하고, 측정하는데 사용하는 방식
- SLA는 최종 고객 뿐만 아니라 내부 프로젝트나 사용자를 위해 개발됨

## Why do SLAs matter?
- SLA가 중요한 이유?
- SLA는 engineering, product, business 팀이 app에서
  - 실제 중요한 사항을 조정하고 들어오는 요청의 **우선순위**를 지정하는데 도움이 됨
- SLA를 통해
  - 서로 다른 팀과 이해관계자는 **동일한 언어**를 사용
  - **동일한 지표**에 관심을 가짐
  - 명확하게 문서화된 기대치에 대한 약속 공유

## Why data reliability SLAs matter
- Data LifeCycle 전반에 걸쳐 다음과 같은 요구사항 존재
  - 데이터 팀/소비자가 **Data Reliablity**를 정의, 추적
- Data Reliability SLA라는 지표가 존재한다면
  - 데이터 **중심** 의존성을 가짐
- 동일한 언어, 지표를 사용하게 되면서
  - 신속하게 우선순위를 정하고, 사고 대응시 시간 단축이 가능함

## How to create data reliability SLAs
- SLA를 정의하기 위해
  - SLI: 서비스 수준 지표
  - SLO: 서비스 수준 목표
    - 서비스 품질의 정량적 측정 및 합의
  - 각 지표가 충족해야하는 목표 값, 범위 필요
- e.g. 많은 engineering 팀이 현장 신뢰성을 나타내는 지표로 **가용성**을 측정하고
  - **가용성**을 99% 이상 유지하는 목표 설정

### Step 1: Defining data reliability with SLAs
- 신뢰할 수 있는 데이터가 조직에 어떤 의미를 갖는지 합의, 명확히 정의
- 기준선 정의
  - 데이터, 사용방법 및 사용자에 대한 Inventory 수행
  - **데이터 과거 성능을 평가하여 신뢰성에 대한 기준 지표 확보 가능**
- 데이터 소비자로부터 **신뢰성**이 어떤 것인지 피드백 받아야 함
  - data lineage를 잘 알고 있더라도, 데이터 엔지니어는 workflow/사용 사례에서 종종 제거될 수 있음
- 내부 팀과 신뢰성 계약시, 소비자가 어떻게 상호작용 하는지
- 어떤 데이터가 가장 중요한지
- 어떤 잠재적 문제에 가장 엄격하고 주의가 필요한지
- 이해관계자(reliability 기득권을 가진 모든 data leader or business consumer)가 개발중인 **신뢰성**의 정의를 고려
- SLA 정의
  - 어떤 데이터로 작업하고 있는지
  - 어떻게 사용하고 있는지
  - 누가 사용하고 있는지

### Step 2: Measuring data reliability with SLIs
- Data SLI는 `Step 1`에서 정의한 데이터와 상호 합의된 상태
- 데이터 사용/미사용 방법 경계 제공
- Data Downtime이 어떻게 보이는지 구체적으로 설명
  - 중복, 누락, 오래된 데이터 등의 시나리오가 포함됨
- **데이터 상태 정량화**에 대한 지표
  - The number of data incidents for a particular data asset(N)
    - 외부 데이터 소스에 의존 가능성이 높아, 사용자 통제를 벗어날 수 있으나,
    - 여전히 data downtime에 중요한 요인
  - TTD(Time-to-Detection)
    - 문제가 발생했을 때, 팀이 얼마나 빨리 경고를 받는가
    - 적절한 탐지, 경고 방법이 없을 경우 몇 주/몇 달 후에 측정 가능
    - 잘못된 데이터로 인해 발생하는 `Slient Error`는 비용이 많이 드는 결정을 내릴 수 있음
  - TTR(Time-to-Resolution)
    - 문제에 대한 경고 수신시, 얼마나 빨리 해결할 수 있었는가

### Step 3: Tracking data reliability with SLOs
- Data Reliability Dashboard 설정시 구체적인 정보를 얻어야 함
- `SLI`를 확인한 이후, data downtime을 허용할 수 있는 범위, 목표 정의 가능
- SLO는 **현실적**이어야 하며 **실제 상황**을 기반으로 해야 함
- 범위를 합의 하면
  - 사고를 **심각도**별로 평가하는 **일관된 프레임워크 개발 가능**
  - 문제 발생시 신속하게 의사소통 및 대응 가능
- 목표 설정 이후 `SLA`에 통합하면
  - 대시보드를 만들어 **진행 상황**을 추적 및 보고 가능

## Data Reliability in Practice
- 데이터 사고가 발생하면 downstream 소비자에게 큰 문제가 발생되기 전
  - 사고를 분류하고 관리하는 방법도 존재
- 사고 해결 프로세스를 쉽고 원활하게 수행하기 위해
  - SRE Handbook을 통해 효과적으로 전달 및 분류가 가능함
- 이모지를 통한 의사소통
  - ![image](https://github.com/Wshid/daily-poc/assets/10006290/450edd95-15b8-4145-81ff-f0b250e832d1)
- 사고의 심각성 판단
  - ![image](https://github.com/Wshid/daily-poc/assets/10006290/f4afd27c-04bf-4880-adf4-464d4f017f89)

## 정리
- SLA, SLI, SLO는 data downtime을 측정하는데 유용한 프레임워크 제공
- end-to-end data observation을 통한 data downtime 방지
