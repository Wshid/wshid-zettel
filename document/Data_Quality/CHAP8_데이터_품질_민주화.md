---
date: 2025-02-12
datetime: 2025-02-12T21:46:00
book: 데이터_품질의_비밀
page:
  - "262"
tags: 
references: 
aliases:
---
- 조직 전반에 대해 데이터 품질 관리의 중요성
- 구성원 모두가 데이터 품질 관리에 쉽게 접근할 수 있도록 민주화 하는과정
	- 문화적, 조직적 장벽 논의
- 데이터를 프로덕트처럼 취급하는 시각
- 데이터 품질에 대한 회사의 RACI 매트릭스를 이해하는 원칙
- 비즈니스에 미치는 영향을 극대화할 수 있는 데이터 조직 구성 방식
- 하기 질문을 받아본 적이 있다면, 이미 데이터 신뢰성 책임자
	- 이 데이터의 상태가 괜찮은가
	- 대시보드를 믿어도 되는가
	- 이 데이터셋 담당자는 누구인가
- 데이터 신뢰성을 확보하는 과정에서 데이터 품질 테스트의 수행 횟수는 중요하지 않음
	- 회사 전체가 이를 중요하게 생각할 때 비로소 데이터 신뢰성 확보 가능
	- 데이터 품질의 이니셔티브를 추적하고, 수행하고 이를 측정하는 역할은 대개 데이터 조직이 하기 때문
- 데이터 품질을 확보하는 일
	- 안정적인 데이터 파이프라인을 구축하거나
	- 데이터 신선도를 위해 SLA를 마련하는 작업 그 이상
	- 이는 교육과 커뮤니케이션에 관련된 일이기도 함
- [[data_quality|데이터 품질]]은 기술적인 프로세스뿐 아니라 문화적인 측면도 포함함
	- 완벽하게 정확한 데이터를 갖추는 일이라기보다
	- 어느 정도로 신뢰할 수 있는지 제대로 이해하는 일에 가까움
- ThoughtSpot의 CDO이자 가트너의 부사장이었던 Cindi Howson
	- 사람들은 고품질 데이터가 애초에 완벽하게 깨끗할 수 없다는 사실을 이해하면
		- 오히려 데이터의 출처를 신뢰하게 됨
	- 데이터 분석 리드 중 한명이 아래와 같은 질문을 하다면 좋음
		- 데이터가 방향성 차원에서 정확한가?
		- 우리가 이 데이터로 의사 결정을 내릴 수 있을 만큼 정확한가?
	- 물론 어떤 일들은 매우 정확해야 하나
		- 캠페인 분석이나 고객 경험 트렌드 등은 어느 정도 정확하기만 하면 됨
- 모두가 데이터 품질에 쉽게 접근하고 이를 가깝게 느끼도록 만들기 위해서는
	- 소프트웨어 코드 작업을 할 때와 마찬가지로
	- 최대한 쉽고 반복적으로 데이터 품질을 개선하고 보완할 수 있도록 작업하면 좋음
- 데브옵스와 소프트웨어 조직은 애플리케이션 구축에 '애자일'한 방식 채택
- 데이터 품질을 위해 관련 절차와 시스템을 참고할 수 있는데,
	- 이는 데이터를 프로덕트처럼 다루는 것

# 8.1. 데이터를 프로덕트로 다루는 시각
- 데이터는 더 이상 [[이급 객체]](second-class citizen)이 아님
- 관련된 기술 발달, 역할 다양화, 데이터 잠재력에 대한 이해도가 높아지면서
	- 데이터 생태계를 보유한 기술 스택 중 하나의 온전한 부분으로 인식
- 데이터를 프로덕트처럼 취급하는 새로운 패러다임
- 데이터 프로덕트가 정확히 무엇인지 정의하는 것은 어려움
	- e.g. 루커 대시보드, 태블로 리포트, A/B 테스트 플랫폼 ~ 멀티레이어드 데이터 플랫폼에서의 정말 다양한 것들을 데이터 프로덕트로 간주 가능
- 구체적으로 시각화하거나 처리한 데이터 또는 작업에 사용하는 데이터가 무엇인지와는 별개로,
	- 데이터 프로덕트라면 공통으로 다음의 결과물을 제공할 수 있어야 함
		- 데이터 접근성의 향상(필요한 곳에, 필요한 때에 제공)
		- 데이터 민주화의 발전(누구나 데이터 쉽게 이용 가능)
		- 더욱 빠른 이익 실현(신속한 인사이트 도출)
		- 데이터 조직과 사용자의 시간 절약
		- 더욱 정확한 인사이트(e.g. 실험 플랫폼)

### 데이터 프로덕트가 갖추어야 할 중요한 특징 또는 품질 기준

#### 데이터 신뢰성과 옵저버빌리티
- SaaS 프로덕트에서 허용 가능한 데이터 손실로 인한 가동 중지는
	- `99.9%`, `99.999%`와 같이 '소수점 뒷 자리수'를 셀 만큼 정교한 수준
- 소프트웨어 엔지니어들이 datadog 또는 New relic과 같은 프로덕트를 통해 SaaS 프로덕트의 성능을 측정하듯
	- 데이터 프로덕트 매니저들에게는 거의 실시간 수준으로 데이터 프로덕트의 성능 이슈 발견 및 해결 솔루션 필요

#### 확장성(Scalability)
- 데이터 프로덕트는 조직 성장 및 수요 증가에 따라 유연하게 변할 수 있어야 함

#### 범용성(Extensibility)
- 데이터 프로덕트는 주로 다양한 솔루션을 통합하여 구축했을 가능성이 높음
	- 그럼에도 API 쉽게 통합 가능 필요
- 최종 사용자가 원하는 각기 다른 수많은 방식으로 데이터 사용 가능 필요

#### 사용성(Usability)
- 훌륭한 SaaS 프로덕트는 더 나은 고객 경험을 제공하는데 집중
- 좋은 프로덕트는 사용자가 배우기 쉽고, 사용하는 과정은 재밌으며, 빠르게 작업 완수

#### 보안과 컴플라이언스
- 데이터 유출의 대가는 과태료만큼이나 값비싸고 고통스러움

#### 프로덕트 출시 규율과 로드맵
- SaaS 프로덕트는 지속적으로 성장하고 개선됨
- 업데이트를 위한 강력한 품질 보증 절차와 함께
	- 최소 1년에 한번은 로드맵 구축

# 8.2. 데이터를 프로덕트로 다루는 사례
- 2000년대 초, 링크드인, 넷플릭스, 우버 같은 회사들이 겪은 문제
	- 지금까지 업무에 계속 데이터를 활용하고 대규모 데이터도 자주 사용
	- e.g. 프로덕트 로드맵의 개선, 경영진의 의사 결정, [[paid marketing]]
- 데이터 사용 방식과 주체에 대한 [[compliance]]와 지침, 제한 사항도 마련함
- 그러나 데이터 분석의 운영과 확장 가능성, 접근성을 위한 솔루션 개발을 담당하는 이는 아무도 없었음
- 하지만 최근의 데이터 엔지니어, 기술 아키텍트, 관리자 ,임원은
	- 소프트웨어 엔지니어링과 사이트 신뢰성의 모범적 사례들을 데이터 시스템에 적용해왔고
	- 그 과정에서 몇 가지 교훈을 얻음
- 데이터 품질을 최우선순위로 여긴다는 점
	- 데이터를 프로덕트처럼 취급하기

## 8.2.1. 서비스 또는 산출물로서의 데이터
- 최근까지도 각 기능 조직과 관련 분석가들은
	- 사일로 형태의 각기 다른 운영 및 분석 데이터의 품질과 가용성, 성능 관리 담당
- 그러나 데이터 패브릭, 데이터 매시와 같은 분산 아키텍처와 데이터 공유 부각으로
	- 데이터를 다양한 분야를 넘나들며 진화하는
	- 하나의 독립적인 객체로 취급하는 절차, 워크플로가 업계의 표준이 될 것
- [[Convoy]]는 서비스 또는 산출물 이라는 두가지 접근 방식을 통해 데이터를 일종의 프로덕트 취급

### 데이터를 프로덕트로 취급하는 것에는 두 가지 접근법
- 데이터 플랫폼의 프로덕트 책임자, Chad Sanderson
- 데이터를 생성하는 내외부 프로덕트나 서비스에서 **데이터 파이프라인을 포함한 데이터 일체를 프로덕트로 보는 견해**
	- 데이터 애플리케이션 코드와 동일하게 엄격한 수준 준수 필요
	- 머신러닝 모델 배포, 데이터 웨어하우스 쿼리, 어떤 일이든 결국 서비스 이기 때문
- 사용자에게 가치를 제공하는 **코드베이스의 산출물 일체**를 프로덕트로 보는 견해
	- 보고서, 대시보드, 플랫폼과 같은 데이터의 산출물을 프로덕트로 취급
	- [[data_warehouse|데이터 웨어하우스]]는 SQL로 구성된 코드베이스로서 분석가나 데이터 과학자 또는 데이터를 이용해 비즈니스 의사 결정을 내리는 프로덕트 매니저와 같이 내부 고객에게 서비스 제공
	- **회사에서 접근 할 수 있는 프로덕션 데이터 환경에 푸시되는 것들이라면 무엇이든 프로덕트가 될 수 있음**
	- 나아가 모드나 Metabase와 같은 대시보드 도구를 사용하고, SQL을 작성하며
		- 해당 대시보드를 다른 사람이 접근할 수 있는 공용 환경으로 푸시하는 경우에도 프로덕트로 간주
- 두 가지 접근법에서 모든 데이터는 사일로화한 단독 객체가 아닌 하나의 마이크로 서비스에 가까움
	- 각각의 비즈니스 부문에서 동일한 데이터를 다양한 사례 활용
	- 데이터 조직 외에도 다양한 사용자가 데이터에 접근하기 때문
- 어떤 경우든 모두 우수한 데이터 테스트와 명확한 [[Data_SLA_SLO_SLI]], 다방면의 문서화 모니터링 필요
	- 데이터는 신뢰할 수 있어야 하며 그러지 못한 경우가 발생하였을 때,
	- 데이터 조직과 이해관계자들은 이를 해결할 수 있는 방법을 갖추고 있어야 함

## 8.2.2. 데이터 프로덕트 매니저의 등장
- Ubercab으로 출발한 우버
	- 우위의 수단 및 사용자에게 더욱 신뢰할 수 있는 맞춤형 경험 제공을 위한 수단 => 데이터를 중요시
- 우버에서도 실시간 가격 책정 모델을 구축하는 데이터 과학자부터
	- 운전자 수요 예측을 위한 운영 분석가에까지 수천명의 인원이 데이터 분석
- 대규모 데이터를 제대로 운영하기 위해
	- 사내 여러 팀에서 활용할 수 있는 프로덕션 소프트웨어로 취급해야 했음
- 소프트웨어 회사에서는
	- 전통적으로 프로덕트 매니저가 소프트웨어 솔루션의 아이디어 구상부터 결실을 얻기까지 전 단계 관리
- 데이터 역시 신뢰성, 접근성을 갖추면서 여러 사용 사례 동시 만족이 필요했음
- 우버는 데이터에도 프로덕트 매너지를 두기로 함
- [[데이터 프로덕트 매니저]] 
	- [[데이터 프로덕트 매니저#우버의 데이터 프로덕트 매니저, 다음 질문에 답하는 것에 전념|우버의 데이터 프로덕트 매니저, 다음 질문에 답하는 것에 전념]]
	- 위 질문의 답으로 직원들을 위한 내부 도구 및 플랫폼을 구축

## 8.2.3. 데이터를 프로덕트로 보는 접근 방식
- 프로덕트 접근법을 조직에 적용하는 5가지 방법

### 이해관계자들과 초기부터 자주 이해도 조정하기
- 데이터 = 프로덕트, 내부 고객인 구성원 = 이해관계자
	- 데이터 프로덕트 로드맵과 SLA를 구상할 때 주요 데이터 사용자들과 먼저 협력 필요
- [[데이터 프로덕트 매니저]]도 내부 고객의 요구 사항, 걱정, 데이터를 사용하는 이유에 대해 완전히 이해하는 것
	- 누가 어떤 목적으로 어떻게 데이터를 사용하는가
	- 사용자들의 요구사항을 충족하기 위해 어떤 데이터 프로덕트를 구축해야 하는지 파악하는데 도움
- [[데이터 스토리텔링]]을 적용하는데 도움이 됨
	- SW와 프로덕트, UX 조직은 데이터 스토리텔링을 통해 자신들의 업무 맥락을 다양한 관점으로 설명
	- 이를 통해 이해관계자들은 이들의 가치를 더욱 잘 이해
- [[데이터 스토리텔링]]은 데이터 인프라에 투자하도록 사람들을 설득하는 데 유용하게 사용됨
	- [[데이터 이니셔티브]]가 비즈니스 목표, 즉 회사의 수익을 높이는 데 도움이 된다는 사실을 분명히 전달한다면
	- [[data_quality|데이터 품질]]에 할당된 리소스를 포함한 각종 예산과 인력 등을 정당화하기 용이해짐
- **필요는 발명의 어머니**
- 데이터 수요가 증가할 수록 데이터 관련 지출을 정당화하는 방안 역시 발전
	- "데이터 품질은 비즈니스에 도움이 됩니다"라기 보단
	- 데이터 신뢰성이 어떻게 수익 예측 머신러닝 모델의 정확도를 높일 수 있는지에 대해 스토리텔링 한다면
		- 더욱 더 신뢰성을 높일 수 있음

### 프로덕트 관리 마인드셋 적용하기
- 데이터 = 프로덕트, 데이터 구축, 모니터링, 측정시 **프로덕트 관리 마인드셋**을 적용하는 것
	- 데이터 파이프라인, 시스템 구축 -> 일반적인 소프트웨어 구축 절차 적용
	- 과업 범위 문서를 만들고, 프로젝트를 스프린트 단위로 나누어 설계하는 절차 포함
- 애자일 기반의 워크플로
	- 데이터를 프로덕트처럼 취급
	- 데이터 조직에도 프로덕트 관리 원칙 적용
	- 데이터가 필요한 대규모 전략 프로젝트 -> 프로젝트 매니저가 이해관계자들과 함께 스펙 문서 작성하는 것처럼
		- 데이터 과업 범위 문서 작성
	- 엔지니어 및 프로덕트 매니저들과 함께 반복적으로 개선 -> 다양한 팀이 참여
		- 이해 관계자들 간의 기대화 이해를 합일 시켜 나감
- 데이터 조직은 파이프라인 구축 시, 확장성과 향후 사용 사례 고려 필요
	- 데이터 조직들의 과거 업무 방식과의 차이?
	- 데이터 모델이 지저분해지는 가장 큰 이유
		- 대게 서비스를 빠르게 구축하는 데에만 집중하고 데이터 자체를 뒷전으로 취급하기 때문
	- 데이터 = 프로덕트 -> 위 업무 방식을 바꿔 나가기 위한 일련의 변화
- 데이터 조직의 성장 그리고 확장성
	- 데이터 조직에 새로 합류한 사람들의 온보딩 및 데이터를 더 쉽게 찾고 접근할 수 있는 방법 탐색
	- '슬랙에 가서 물어보세요'는 확장성에 도움이 되지 않음
	- 데이터 프로덕트 구축시 **무엇이든 문서화**하고 그 과정에서 불필요한 중복, 다른 문제가 없는지 확인
		- 문서를 매우 명확히 관리 필요
- 데이터 프로덕트 구축 전 **비즈니스 목표에 부합하는 KPI 설정** 필요
	- 스토리텔링은 데이터 품질에 대한 투자의 잠재적인 가치를 설명하는데 도움이 될 수 있으나
	- 대부분의 조직은 이러한 [[데이터 이니셔티브]]를 통해 얻을 수 있는 재정적 가치를 측정하기 때문
- 다수의 데이터 조직에서는 [[data_quality|데이터 품질]]과 관련된 KPI를 채택함
	- 혁신이나 새로운 데이터 프로덕트 구축보다
	- 데이터의 불완전함, 오류, 누락 또는 부정확한 일이 발생한 [[data_downtime|데이터 다운타임]]에 따른 비용 계산
	- 데이터 조직의 구성원들이 문제 해결에 소요한 시간을 측정
- [[데이터 이니셔티브]]의 파급 효과를 정량화하여 측정하기 위한 방안 -> **메트릭 설정**
	- 중앙 데이터 플랫폼이 있는 경우라면 메트릭 사용 사례 전반에 일관되게 적용되었는지도 확인 필요

### 셀프 서비스 도구에 투자하기
- 데이터를 사일로에서 꺼내 그 자체로 가치 있는 프로덕트 취급을 위해
	- 비즈니스 사용자들이 스스로 업무 수행 및 데이터에 대한 요구사항 충족 필요
- 기술 팀이 아닌 이들도 데이터에 접근할 수 있도록 **셀프 서비스 도구** 사용하기
	- 데이터 조직은 애드혹 업무 대신 가치를 생산할 수 있는 프로젝트에 집중
- 셀프 서비스 도구는 분산 데이터 아키텍처에 대한 새로운 접근 방식인 [[data_mesh|데이터 메시]] 개념의 주요 원칙 중 하나
- 사용자들이 특정 셀프 서비스 도구를 사용하여 데이터 검색, 접근 등 작업을 완료하는 데
	- 걸리는 시간을 줄였는지 평가할 수 있는 메트릭 적용
- 중앙 데이터 조직은 데이터 생산자와 사용자 모두 필요한 작업을 더욱 쉽게 수행할 수 있게끔
	- 올바른 셀프 서비스 인프라와 도구를 사용할 수 있도록 해야 함
- 올바른 도구를 갖춰 주고 구성원들이 알아서 업무에 사용하게 만드는 것

### 데이터 품질과 신뢰성 우선시 하기
- 데이터를 프로덕트로 취급하기 위한 또 다른 방법 -> 데이터 수집 ~ 사용자에게 전달되는 결과물까지
	- 데이터 환경 전반에 엄격한 기준을 적용하는 것
- 데이터 라이프사이클 전체에서 데이터 품질과 신뢰성을 우선시 한다는 뜻
	- [데이터 신뢰성과 성숙도 곡선](https://oreil.ly/CsX3x)에 진척 상황을 표시하여 데이터 품질의 현황 평가 가능
- 데이터 신뢰성에는 하기 네가지 단계 존재

#### 사후 대응
- 조직은 비상 상황에 대응하는 훈련, 문제 파악에 대부분의 시간 사용 -> 중요한 [[데이터 이니셔티브]]를 더디게 활용
- 따라서 프로덕트, 머신러닝 알고리즘, 비즈니스 의사결정에 데이터를 효과적으로 사용하기 위해 고군분투

#### 사전 준비
- [[data_engineer|데이터 엔지니어]], [[data_analyst|데이터 분석가]], [[data_scientist|데이터 과학자]] 들의 적극적인 협업을 통해 자동 검사, 커스텀 QA 쿼리 개발 및 검증
	- e.g. 데이터의 신선도를 보장하기 위해 파이프라인의 주요 단계에서 행의 수 확인, timestamp 확인
- 문제가 발생할 경우 슬랙 메세지, 이메일로 전달되지만
	- 상당 수의 문제는 이러한 사전 준비 과정에서 모두 포찰

#### 자동화
- 조직은 파이프라인의 더 넓은 범위에 검증 쿼리르 예약 -> 데이터의 신뢰성, 정확성 우선시
- 데이터 상태 대시보드를 활용해 문제를 확인하고 해결한 후 구성원에게 상황 공유
	- e.g. 추이와 변경 사항을 관찰하기 위해 차원과 측정값 관련된 메트릭을 저장하여 추적
	- e.g. 데이터 수집 단계에서부터 스키마를 모니터링하고 적용

#### 확장성
- [[dev_ops]] 개념을 기반으로 스테이징 환경, 유효성 검사를 위해 재사용할 구성 요소, 데이터 오류 경고 등을 적용
- 광범위하고 필수적인 데이터에 관해 적절히 조치 -> 사용자에게 영향을 미치기 전에 대부분 문제 파악
- e.g. 모든 주요 메트릭에 대한 이상 탐지 수행, 모든 테이블 및 작업을 모니터링하여 품질을 추적

#### 네 가지 단계 정리
- 사후 대응: 비상 상황 대응 모드
- 사전 준비: 수동 추적 및 탐지
- 자동화: 프로그램화된 조치
- 확장성: 넓은 범위에 임베디드된 조치

### 적절한 데이터 조직 구조 찾기
- 조직 구조 = 데이터 상호 작용하는 방식에 큰 영향
	- e.g. 중앙 집중형 조직이 데이터 관리 및 app 전반을 담당하는가
		- 분석가들이 여러 사업부에 속해, 특정 요구사항에 대응하고 도메인 전문 지식을 쌓는 한편
		- 거버넌스 부족, 사일로로 고통받고 있지는 않은가
- 여러 데이터 리더들은 [[hub and spoke]] 구조에서 가장 좋은 결과를 만들어냄
- [[hub and spoke|허브 앤 스포크]]구조에서 중앙 집중형 데이터 플랫폼 조직이 인프라 및 데이터 품질을 담당하는 한편
	- 각 사업부에 분산되어 속한 분석가들과 엔지니어들이
	- 각 시멘틱 레이어를 담당하며 데이터를 비즈니스에 사용
- 위 구조는 조직이 빠르게 성장하고, 빠르게 움직이는 경우에 효과적
	- 단, 중앙 집중형 데이터 조직과 긴밀한 협업이 이루어지지 않을 경우
	- 각 사업부에 속한 분석가들이 불필요한 중복 업무 처리 상황 발생
- 중앙 집중형 -> 분산형 -> 허브 앤 스포크 구조 -> 프로덕트 관리원칙인 애자일을 강조함

> 결국 데이터 조직의 핵심은 모든 사람이 비즈니스에 최대한의 가치를 더할 수 있게 만드는 일
> 변화에 열려 있고, 다양한 시도, 조직의 인원 규모에 따라 알맞은 방식이 다름

- 분산형 구조의 장점
	- 분석가와 엔지니어들이 데이터 요청 너머에 숨은 진짜 비즈니스 수요를 이해할 수 있음
	- e.g. 꼭 이렇게 복잡한 클러스터링 기법을 고수해야 하는가? 정말 필요한 건 무엇인가?
		- 만약 그를 알면 빠르게 필요한 데이터 서빙이 가능할 것
- [[hub and spoke|허브 앤 스포크]]구조를 통해 데이터 품질과 거버넌스의 책임을 포기하지 않고도 비즈니스 요구사항을 빠르게 충족 가능
- 데이터를 프로덕트로 취급하는 관점은 유행이 아님
	- 이는 스스로 데이터를 활용할 수 있는 능력, [[데이터 민주화]] 증진
	- 더욱 정확하고 확신 있는 의사 결정을 위해 [[data_quality|데이터 품질]] 개선
	- 조직내 데이터 영향력 증진, 유의미한 결과물을 만들어 내기 위해 필요한 사고방식의 전환

# 8.3. 데이터 플랫폼을 향한 신뢰 축적
- 데이터를 프로덕트로 취급하기, 실제로 어떻게 구현할까?
- 조직에서 데이터 플랫폼을 실제로 사용하고, 이해관계자들이 그 결과를 신뢰할 수 있도록 토대를 마련하는 방안?
	- 데이터 플랫폼을 프로덕트처럼 다루게 만드는 방안은?
- 하기 사례를 통해 알아보기

## 8.3.1. 프로덕트 목표와 비즈니스 목표의 일치
- 데이터 플랫폼을 구축하거나 확장할 때
	- 데이터가 회사의 목표에 어떻게 부합하는지 가장 먼저 확인해야 함
- 이를 위해, 데이터 플랫폼을 총괄하는 프로덕트 매니저의 관점으로 사고해야 함
- 단, 다른 프로덕트 매니저들과 다르게 데이터 플랫폼의 프로덕트 매니저는
	- 특정 영역별 목표 너머 비즈니스의 큰 그림을 이해해야 함
- 데이터가 마케팅, 채용에서부터 사업 개발 및 영업에 이르기까지
	- 다른 모든 기능 팀의 요구사항에 반영되기 때문

### 예시: 비즈니스 목표가 수익 증대라면
- 위 목표를 달성하는데 데이터가 주는 도움?
	- 수익 성장을 주도하는 서비스 및 프로덕트는 무엇인가
	- 해당 서비스 또는 프로덕트는 어떤 데이터를 수집하는가
	- 데이터를 사용하기 전에 해야 할 일은 무엇인가
	- 이 데이터가 필요한 조직은 어디인가, 그들은 해당 데이터를 어디에 사용하는가
	- 이 데이터 또는 데이터의 분석 결과에 접근할 수 있는 사람은 누구인가
	- 사용자들은 데이터에 얼마나 빨리 접근할 수 있어야 하는가
	- 데이터 플랫폼에서 해결해야 할 컴플라이언스 준수 또는 거버넌스 확인이 있다면 무엇인가
- 위 질문들에 답을 하다보면
	- 프로덕트 로드맵의 우선순위를 지정하는 방법뿐 아니라
	- 누굴 위해 데이터 플랫폼을 구축하고 디자인하는지 더 명확하게 이해할 수 있음
- 전체적인 관점에서 KPI를 설정하고 실행 전략을 세움으로서
	- 조직 전체에 보다 더 큰 영향력을 발휘하는 플랫폼 만들기 가능

## 8.3.2. 올바른 이해관계자들에게서 피드백과 승인 획득
- 데이터 플랫폼을 구축하는 과정에서 항상 미리 승인을 받고
	- 중간 과정에서도 반복적으로 피드백을 받는 것은 중요함
	- 그러나 구체적으로 누구의 승인과 피드백을 받아야 할 것인지 이해하는 이들은 드물다
- 당연히 CTO와 데이터 부사장의 최종 승인이 필요함
	- 그러나 이들의 의사 결정도 대게는 다른 엔지니어, 프로그램 매니저, 데이터 실무자의 조언 참고
	- 회사의 새로운 데이터 카탈로그 시스템을 개발하는 과정에서
		- 엔지니어링 부사장을 설득하기 위해 3개월이라는 시간을 쏟았으나
		- 한 수석 보좌관의 이메일 한통으로 프로젝트가 중단되는 상황도 있었음
- 따라서 회사의 DNA를 바탕으로 다양한 방법 고려가 필요, 다음 세 가지 방식을 동시에 진행하기
	- 데이터 플랫폼의 비전을 갖도록 리더들을 설득하기
	- 데이터 플랫폼의 실제 사용자들에게 사례와 핵심 내용을 강조하기
	- 누구와 대화하든 항상 고객 중심으로 접근하기
		- 데이터 조직과 데이터 사용자를 포함해
		- 다양한 환경에 놓인 다양한 유형의 구성원들에게 도움을 줄 수 있는 서비스로 포지셔닝
- 뛰어난 데이터 플랫폼을 사용하면
	- 기술을 사용하는 이들은 더욱 쉽고 효율적으로 업무 수행 가능
	- 기술과 다소 거리가 있는 사람들이더라도 풍부한 인사이트를 얻거나, 엔지니어와 분석가의 도움 없이도 데이터 기반 시각화 가능
- 데이터 플랫폼을 구축하려면, [[data_engineer|데이터 엔지니어]], [[data_scientist|데이터 과학자]], [[데이터 프로덕트 매니저]], 비즈니스 관련 사용자, 일반 관리자 등
	- 데이터와 관련된 다양한 사람들을 고려해야 함
- **결국, 이 과정을 통해 함께 만들고, 나누고, 배울 수 있는 데이터 애호가들의 커뮤니티 구축이 필요**
	- 회사의 데이터 플랫폼에는 회사 전체에 양질의 서비스를 제공할 수 있는 잠재력이 있음
	- 따라서 중간 과정에서 약간의 타협이 있더라도
		- 관련된 모든 이들이 데이터 플랫폼의 성공에 참여하고 기여했다는 느낌을 받을 수 있게끔 해야함

## 8.3.3. 단기적인 이익보다 장기적인 성장과 지속 가능성 우선
- 데이터 플랫폼은 단순히 남들보다 '최초로 출시'했다고 해서 성공 x
- 데이터 플랫폼 = 내부 사용 도구
	- 특정 기능 단위의 성공이 아닌 지속 가능성을 염두
- 데이터 조직의 고객은 회사, 회사의 성공은 데이터 조직의 성공
- 프로덕트 로드맵은 수차례 변경되겠지만
	- 이런 변화가 있을 때마다 항상 프로덕트의 성장, 성숙 고려 필요
		- 우버의 빅데이터 플랫폼, 5년이 걸림. 비즈니스 요구사항에 따라 지속적으로 진화
		- 핀터레스트, 핵심 데이터 분석 프로덕트에 대해 수차례 반복적으로 수정, 개선 작업
		- 링크드인, 2008년부터 데이터 플랫폼을 구축하여 지속적 보완
- 시간에 따라 변화하는 조직의 맥락에 맞게 의미 있는 솔루션을 선택하고
	- 조직의 기대치와 데드라인에 맞춰 계획을 세울 것을 권장
- 단기적인 사용성을 위한 솔루션은 시작하기 쉬우나
	- 시간이 갈수록 결국 지속 가능성을 고려해 만든 플랫폼보다 더 큰 비용이 발생함
- 장기적인 프로덕트 개발 전략의 일환으로,
	- 일부 즉각적인 성과를 만들어 내는 것이 내부 구성원들의 승인을 얻는데 더 유용
	- 단, 데이터 플랫폼도 한 번에 뚝딱 만들어지는 것은 아님

## 8.3.4. 데이터 측정 기준 메트릭 설정
- 데이터를 신뢰할 수 없다면 데이터 플랫폼이 훌륭한지는 의미가 없음
- 그러나 각 이해관계자마다 [[data_quality|데이터 품질]]이 의미하는 바는 다름
- 그래서 그 기준과 정의를 이해 관계자들의 기대, 이해에 맞추지 못한다면
	- 데이터 플랫폼은 결코 성공할 수 없음
- 위 문제를 해결하기 위해 데이터 신뢰성,
	- 전체 데이터 라이프사이클에서 제공하는 데이터 가용성과 상태에 대해 기본적인 기대치 설정 중요
- 소프트웨어 어플리케이션 신뢰성에 대해 명확한 [[Data_SLA_SLO_SLI|SLO, SLI]]를 설정하는 것은 쉬운 일은 아님
- 데이터 조직은 [[data_pipeline|데이터 파이프라인]]에 대해서도 목표, 지표 지정 핑료
- 그러나 각기 다른 이해관계자들이 '좋은 데이터'에 관해 모두 동일한 비전을 가져야 한다는 뜻은 아님
	- 그렇지 못할 가능성이 다분하나 그럼에도 괜찮음
- 서로 완전히 다른 것을 기대하기보다는 데이터 신뢰성 면에서 기본이 되는 메트릭을 설정하고
	- 최소한의 공통 분모를 찾아 합의하는 것이 중요
- [[data_downtime|데이터 다운타임]] 시간 또는 주당 데이터 품질 이슈의 개수와 같은 SLA를 선택하여
	- 회사 내의 데이터 실무자들이 기본 품질 메트릭에 대해 동일한 기대, 이해를 갖도록 만들기

## 8.3.5. 직접 구축 또는 구매할 시기 파악
- 데이터 플랫폼에 관해 가장 먼저 내려야 할 의사 결정
	- 플랫폼을 처음부터 새로 구축할지, 외부 솔류션을 구매할지
- 우버, 링크드인, 메타 => 오픈 소수 솔루션으로 자체 데이터 플랫폼 구축
	- 이 방법이 꼭 맞는 것은 아님
- 데이어 플랫폼을 직접 구축할지, 구매해야 할지에 대한 공식은 없으나, 다음 경우가 되기 전 까지는 외부 솔루션 구매 권장
	- 개인의 재정 기록, 건강 상태에 대한 정보처럼
		- 규제상 외부 업체와 공유할 수 없는 민감 정보 or 기밀정보 이용 운영시
	- 내부의 다른 도구 및 시스템과 원활하게 작동하려면 커스텀이 필요하나
		- 공급 업체에서 이를 지원해줄만큼의 규모가 되지 않는 경우
	- 비즈니스 차원의 경쟁 우위나 인재 채용관점의 이점 등
		- 구매보다는 직접 구축하는 방안이 전략적으로 유리한 경우
- [[data_warehouse|데이터 웨어하우스]], [[data_lake|데이터 레이크]], 데이터 시각화 도구와 같이 기술적으로 더 크고 보편적인 경우라면, 대개는 구매하는 편이 더 나음
- 그러나 고속도로 GPS 데이터 집계와 같이 틈새시장이지만 비즈니스 차원에서는 매우 중요한 문제를 해결하기 위해
	- 직접 구축하는 것이 더 나을 수도 있음
- 리버스 ETL, 데이터 과학 워크북, 행동 분석, 머신러닝 피처 저장소와 같이
	- 이전에는 틈새 기술로 여겨졌던 것들이 널리 채택되고 있음
- 데이터 플랫폼을 프로덕트로 직접 구축한다면
	- 데이터 우선순위에 대해 더 많은 합의를 이끌어 낼 수 있으며
	- 데이터 품질과 다른 KPI를 표준화하고 협업 강화가 가능하며
	- 결과적으로 회사에 더 많은 가치 제공 가능
- 데이터 관리, 데이터 신뢰성, 데이터 민주화를 위한 효율적인 수단이라는 점 외에도, 데이터 플랫폼을 프로덕트로 구축하는데의 이점
	- 영업 활동에 가이드 제공(잠재 고객의 반응을 기반으로 역량을 집중시킬 곳에 대한 인사이트 제공)
	- 프로덕트 로드맵을 주도적으로 작성
	- 고객 경험의 개선
		- 서비스의 문제점이 무엇인지
		- 제대로 작동하는 부분과 그렇지 못한 부분이 무엇인지
	- 전사 차원의 [[데이터 거버넌스]] 및 컴플라이언스 조치 표준화
		- [[GDPR]]
		- [[CCPA]]
- 데이터 플랫폼 직접 구축은 부담스러울 수 있음
	- 하지만 [[data_quality|데이터 품질]]을 보장하고 확장해 나갈 수 있도록 올바른 접근법을 취한다면
	- 이는 조직 전체의 역량을 배가할 수 있는 잠재력 지니게 됨
- 지금까지 데이터 품질 우선 문화를 구축하기 위해 해야할 일을 중점적으로 살펴봄
- 이어 이를 담당할 주체 살펴보기

# 8.4. 데이터 품질 책임 할당
- 현대의 데이터 조직에는 [[data_quality|데이터 품질]]의 책임에 대한 다양한 질문, 답변이 존재
- 대다수의 전문가가 [[data_quality|데이터 품질]]이나 [[data_downtime|데이터 다운타임]]이 발생하면 책임을 돌리기 바쁨
- 문제 발생에 대한 영향 범위 => 폭발 반경
	- 데이터가 중단될 시 이해관계자가 경험하는 다운타임의 정도
- 조직에는 CDO 부터 데이터 엔지니어까지
	- 데이터 중단 상황과 관련된 여러 이해관계자들이 존재
- [[data_downtime|데이터 다운타임]]은 회사의 데이터와 분석에 의존하는 모든 사람에게 영향을 미치며
	- 이는 데이터 파이프라인의 하단으로 내려갈 수록 커짐

### [[data_downtime|데이터 다운타임]]의 폭발 반경

|  **누가**  |          데이터 엔지니어           | 분석가, 데이터 과학자 | 앱 사용자 |   조직 구성원   |      일반 대중      |
| :------: | :-------------------------: | :----------: | :---: | :--------: | :-------------: |
| **프로덕트** |             플랫폼             |     인사이트     | 추천 엔진 |   의사 결정    |       인식        |
|  **도구**  | 데이터 웨어하우스<br>데이터 레이크<br>ETL | BI<br>데이터 모델 |  개인화  |  신뢰<br>경험  |   언론<br>월스트리트   |
|  **영향**  |            통제 가능            |    좌절과 불만    | 매출 손실 | 기업 및 경력 제한 | 법적 위험 등 실질적인 문제 |

- 가상의 데이터 조직 사례를 통해 데이터 신뢰성이 구현되고, 데이터 품질을 책임지는 직책 사례를 살펴보기
- 각 직무와 직무별 희망 사항을 비롯하여
	- 애로사항이 무엇인지, 그리고 회사의 데이터 신뢰성 보장 방안 공유

## 8.4.1. 최고 데이터 책임자
- CDO
- 데이터 파이프라인이나 루커 대시보드를 직접 관리하지는 아니나
	- 조직이 제공하는 데이터의 안정성, 정확성, 연관성, 해석 가능성, 신뢰성 책임자
- 매일 아침마다 다른 부서에서 그들이 필요한 데이터를 제대로 받고 있는지
	- 데이터 관련 리스크를 제대로 관리하는지 확인
- CDO로서 데이터 환경이 제대로 작동되고 있다는 것을 확인한 후에 마음이 놓일 것
- 만약 대표나 대중, 다른 데이터 사용자들에게 잘못된 데이터 전달시 위험

## 8.4.2. 비즈니스 인텔리전스 분석가
- 비즈니스 인텔리전스 책임자 & 데이터 분석가
- 비즈니스 상황에 대한 다양한 질문에 답하기 위해
	- 마케팅, 영업, 운영 부서의 이해관계자들과 공유할 수 있는 효과적이고 통찰력있는 대시보드 필요
- 실무자 단에서 문제가 발생할 경우 가장 먼저 호출됨
- 데이터 신뢰성을 보장하기 위해 다음과 같은 상황 확인
	- 데이터를 통해 비즈니스에 의미 있는 메트릭, 인사이트 전달이 가능한가
	- 데이터 신뢰도에 자신이 있는가
	- 다른 사람들도 해당 인사이트에 쉽게 접근하여 이해 되는가
- 메타에게 `NULL`과 중복 값은 천적
- [[data_downtime|데이터 다운타임]]으로 마음고생하는 일을 피할 수만 있다면 무엇이든 하고자 함
- 그녀는 보고서에 나타난 이상한 값들을 확인해달라는 수많은 요청때문에 지친 상태
- 데이터의 출처를 따라 제대로 된 데이터가 맞는지 확인하려면 시간이 오래 걸림

## 8.4.3. 분석 엔지니어
- 비즈니스 조직, 데이터 분석 조직, 데이터 엔지니어링 조직 사이에서
	- 각 이해관계자들이 필요한 데이터에 접근해 사용할 수 있는지 확인
- 데이터 빌드 도구인 dbt 활용에 능하며, 거의 모든 일은 모델링을 통해 해결할 수 있다고 자신함
- 다만, 이는 잘못된 스키마 변경이 없을 때의 이야기
- [[data_downtime|데이터 다운타임]]이 발생하면 데이터가 손상된 이유와 과정을 설명해야 하며
	- 이릉 위해 데이터 엔지니어링 및 데이터 플랫폼 조직과 협력하여 근본 원인 파악
- [[data_observability|데이터 옵저버빌리티]]는 소중함

## 8.4.4. 데이터 과학자
- [[data_scientist|데이터 과학자]]
- python, 데이터 시각화 사이
- 데이터의 출처와 신뢰도가 매우 중요함
- 데이터를 신뢰할 수 없다면 A/B 테스트가 소용이 없어지며
	- 분석가, 매니저, 임원, 고객 등을 비롯한 모든 데이터 사용자들이 영향을 받음
- 업무시간의 8할을 데이터 스크러빙, 정제, 데이터 맥락 이해에 사용함
	- 이런 과정을 도와줄 도구와 솔루션 필요

## 8.4.5. 데이터 거버넌스 리드
- [[GDPR]]과 [[CCPA]]가 등장할 당시 [[데이터 컴플라이언스]] 준수에 주력
- 조직의 규모가 커질 수록 중요함
- 데이터 신뢰성과 관련하여 전사적인 차원의 데이터, 메트릭 정의의 통합과
	- 데이터의 접근 및 조회권한을 이해하는데 주력
- 잘못된 데이터는 값비싼 벌금, 고객의 신뢰 약화, 소송을 의미
- 종종 자신의 일이 회계와 비슷하다는 농담

## 8.4.6. 데이터 엔지니어
- 데이터 프로덕트를 구축하는 것을 넘어 팀의 비즈니스 의사 결정에 필요한 데이터 소스 통합하는 역할
- [[Snowflake]], Power BI를 비롯한 여러 도구 사용
- 회사의 데이터 환경을 묶는 접착제 역할
- 데이터나 신뢰성을 모니터링하는 기술을 구현하고
	- 문제가 발생하면 새벽 중에라도 분석 조직에서 호출을 받음
	- [[#8.4.2. 비즈니스 인텔리전스 분석가|비즈니스 인텔리전스 분석가]]와 마찬가지로 밤을 새는 경우가 많음
- 성공적인 업무 수행을 위해 다음 업무 수행
	- 확장 가능한 데이터 플랫폼 솔루션 설계하기
	- 데이터 수집의 신뢰성 보장하기
	- 다른 조직이 플랫폼에 액세스 할 수 있도록 만들기
	- 데이터 다운타임이 발생했을 때 신속하게 해결하기
	- 무엇보다도 데이터 조직 전반의 분석을 지속 가능하게 만들기

## 8.4.7. 데이터 프로덕트 매니저
- 데이터 엔지니어링 및 데이터 분석 솔루션에 대해서도 늘 최신 정보 파악
	- 조직이 투자해야 할 프로덕트에 관한 의사 결정을 내리는 경우가 많음
- 접근성과 규모 확장에 용이한 데이터 프로덕트를 제공하는 관점에서
	- 실전 경험을 통해 자동화 도구와 셀프 서비스 도구의 차이가 무엇인지 알고 있음
- 무수히 많은 소스로부터 데이터 수집, 통합, 접근할 수 있도록 만들어
	- 비즈니스 전반의 사용자에게 제공할 수 있는 데이터 플랫폼을 일의 책임자
- 분석가부터, 소셜 미디어 관리자에 이르기까지 모든 데이터 이해관계자들이 그에게 의존
- [[GDPR]], [[CCPA]] 및 [[데이터 컴플라이언스]] 준수도 빼놓을 수 없음
- 역할 자체가 매우 도전적이기 때문에 모든 이를 만족시키기는 어려움
	- e.g. 그의 플랫폼은 비즈니스 인텔리전스 측면에서 '실제로' 바라는 바와 다른 경우가 많음

## 8.4.8. 데이터 신뢰성을 책임지는 사람은?
- 답은 결코 간단하지 않음
- CDO부터 데이터 엔지니어까지, 데이터 신뢰성 보장의 책임은 궁극적으로 모두에게 있음
- 모든 부서가 데이터에 의존하나, 모든 데이터 조직이 동일한 구조도 다르며, 산업도 다름
- [[RACI]] 매트릭스 가이드라인
- [넷플릭스](http://oreil.ly/7kuDl) 또는 [우버](http://youtube.com/watch?v=YxlmgwHJaqw&feature=youtu.be) 같은 테라바이트 단위의 데이터를 수집하고 변환하는 회사에서는
	- 데이터 엔지니어, 데이터 프로덕트 매니저가 데이터 신뢰성 문제를 모니터링하고 경고하는 책임을 짐
- 그러나 대기업을 제외하면 대부분 데이터 엔지니어와 일반 프로덕트 매니저에 책임 존재
	- 이들은 데이터에 대한 조직의 요구사항, 신뢰할 수 있는 데이터 사이 균형 잡기 필요
- 잘못된 선택의 대부분은 비즈니스 인텔리전스 분석가가 책임을 지는데,
	- 이들은 대시보드에 잘못된 정보가 포함되거나
	- 전달되지 않은 변경사항으로 인해 중단되는 경우가 발생하기 때문
- 따라서 초기의 데이터 조직에는 이러한 역할들을
	- 만능 데이터 담당자 또는 프로덕트 매니저에게 함께 맡기기도 함

# 8.5. 데이터 품질 보장을 위한 책임감 조성
- [[data_engineer|데이터 엔지니어]]는 [[데이터 카탈로그]]가 아님
- [[data_analyst|데이터 분석가]]는 종종 파이프라인에 들어오는 데이터가 믿을만한지 물어보는 질문에 답을 해야함
- 이러한 역할의 혼선은 데이터 조직이 더 빠르게 움직이고
	- [[data_mesh|데이터 메시]]를 가로질러 셀프 서비스 데이터 플랫폼을 갖추며 생겨남
- 처음에는 요약 자료로 만든 문서가 나중에는 다음과 같이 바뀜
	- 수많은 사람들이 어느새 임시 장표를 만들고 공유
	- 장표를 수정하다보니 원래의 장표와 다를바 없음
	- '최종_최최종'과 같은 사본이 만들어짐
- 위와 같은 일이 데이터 조직에도 일어남
	- 이해관계자들을 돕거나, 인사이트 제공, 데이터 추출등의 노력을 하지만
	- 템포가 상당히 빠름
- 결국 살짝만 바꾼 모델 몇 개가 사실상 똑같은 기능을 하고 있고
	- 그중에서 무엇이 최신본인지, 어떤 필드를 참조해야 하는지 아는 사람은 업섹 됨
- 위와 같은 상황의 운영상 문제점
	- 불필요한 트래픽 관리, 비효율적인 주기
	- 저품질 데이터
	- 부적절하거나 잘못된 데이터를 사용해 발생한 문제를 해결하는 데 드는 시간 증가
	- 데이터에 대한 조직 내 신뢰 저하
	- 데이터 다운타임 증가
- 비즈니스 환경 전반의 불확실성이 높은 경우, 치명적인 결과를 초래함

# 8.6. 데이터 접근성과 신뢰간 균형
- [[데이터 검색]]은 데이터 자산의 상태를 실시간으로 확인할 수 있는 새롭고 중요한 방식
	- 최근 데이터 스택의 주요 요소
- [[데이터 검색]]은 데이터의 수집 및 저장, 통합 방식과 특정 사용자의 사용 방식 등을 기반으로
	- 데이터에 대해 도메인별로 다양하고 풍부한 이해 제공
- [[데이터 검색]]은 각 도메인의 데이터에 대해 실시간으로 인사이트를 제공하면서 [[데이터 카탈로그]] 대체 가능
	- 동시에 [[데이터 거버넌스]]의 기준 준수
- ![[Drawing 2025-06-24 21.23.49.excalidraw]]
- [[데이터 카탈로그]]와 마찬가지로 거버넌스 기준과 도구들이 도메인 너머 통합
	- 접근성과 상호운용성을 높임
	- [[데이터 검색]]은 카탈로그에 기록된 상태 대신 **데이터의 현재 상태**를 실시간으로 드러냄
- 특히, [[데이터 검색]]은 프로덕트처럼
	- 각기 다른 데이터 오너들이 각자의 데이터를 책임지는 분산된 접근 방식을 취할때 유용함
	- 이를 통해 비즈니스 내 데이터에 능숙한 사용자들이 데이터를 직접 사용할 수 있기 때문
- 이렇게 데이터 접근성이 좋아지는 가운데,
	- 도메인별 데이터 조직에서 어떤 데이터가 사용 및 변환, 승인 되었는지 일일히 파악할 수 있는 방안은?
- 조직 전반에서 공통된 데이터 품질 기준, 오너십, 커뮤니케이션 절차가 유지되고 있는지 개별 도메인에서 확인하는 방법?
	- **데이터 인증에 있음**


# 8.7. 데이터 인증
- 데이터 인증은 데이터 품질, 옵저버빌리티, 소비권 및 귀책 사항, 문제 해결 절차, 커뮤니케이션 등과 같이
	- 상호 합의 하에 맺은 [[Data_SLA_SLO_SLI]]를 충족한 데이터를 바탕으로
	- 조직 내 데이터 사용의 승인을 관리하는 절차
- 데이터 인증이란 데이터 품질, 유효성 검사 및 **데이터 검증**과 유사한 개념으로
	- 사람, 프레임워크, 기술을 비즈니스의 핵심 정책에 부합시키기 위한 중요한 절차
- 데이터 인증 절차의 요구사항은 비즈니스 수요, 데이터 엔지니어링 팀의 역량, 데이터 가용성 등에 따라 다르나, 아래 내용을 포함
	- 데이터 신선도, 볼륨, 스키마, 분포 등의 품질 확인 절차의 자동화
	- 정의된 업타임 내로 SLA 충족
	- 경보 발생시 대응을 책임지는 데이터 오너
	- 슬랙 또는 이메일로 경보 발송
	- 서비스 중단 등의 비상 상황 발생 시 대응 가능한 커뮤니케이션 절차 마련
# 8.8. 데이터 인증 프로그램 실행 7단계
- 데이터 인증 프로그램은 여러 도메인에 걸쳐 적용된 일관적인 접근 방식을 통해 확장성을 높힘
	- 또한 커뮤니케이션 라인을 명확히 하여
	- 도메인 간의 정보 교환을 더 신뢰할 수 있도록 만들어 효율성을 높힘
- 데이터 인증 프로그램의 적용 단계

## 1단계: 데이터 옵저버빌리티 능력 갖추기
- [[data_observability|데이터 옵저버빌리티]] 능력: 조직의 시스템 내 데이터의 상태를 이해할 수 있는 능력
	- 이 능력을 갖추는 것이 인증 프로세스의 첫 번째 단계
	- 기준선을 마련하려면 당연히 현재 성능에 대한 이해를 갖추어야 함
- 선제적으로 문제를 발견하고, 알리고, 분류하기 위해서는
	- 시작부터 끝까지 체계적인 접근 방식이 필요함
- 이러한 옵저버빌리티 역량을 바탕으로
	- 데이터 사고 대시보드는 자동으로 이상 데이터, 스키마 변경, 테이블 삭제, 규칙 위반을 표기함
- 데이터 활용 체계에 대한 상세한 이해 외에도
	- 데이터 파이프라인 내에서 이슈가 발생하는 경우
	- 이를 바로 인지할 수 있는 체계는 문제가 생긴 지점을 짚어내
	- 이슈 탐색과 해결에 필요한 시간을 단축함
- 데이터 사고 대시보드
	- 지난 7일간의 데이터 사고 내역
	- 이상 데이터 수, 스키마 변경 수, 규칙 위반 수
- 문제를 일으키는 시스템이나 데이터 셋이 무엇인지를 파악하면
	- 데이터 SLA 작성 절차에 필요한 정보를 제공하는데 도움이 됨
	- 또한 주요 테이블 또는 보고서의 종속성을 이해함으로써, 가장 주의해야하는 데이터가 무엇인지를 이해하는데도 도움이 됨
- **데이터 인증**을 위해서는 테이블이나 데이터셋의 이상치를 면밀히 모니터링 해야 함
	- 나아가 머신러닝을 통해 꾸준히 이 과정을 학습하고 개선할 수 있다면 가장 이상적

## 2단계: 데이터 오너 결정하기
- 인증된 각 데이터 자산에는
	- 데이터 수집부터 분석에 이르는 데이터 라이프 사이클 전반을 책임질 당사자, 데이터 오너가 있어야 함
- 최신 메타데이터 관리 도구를 사용하면 데이터 오너를 각 테이블에 지정 가능
	- 이를 통해 주요한 데이터셋의 신뢰성을 확인할 수 있음
- 책임자를 지정할 때 [[RACI]] 모델을 활용하거나, 커뮤니커에시녀 절차나 문제 해결 시간과 같은 기준을 포함해 [[Data_SLA_SLO_SLI|SLA]]에 직접 포함할 수도 있음

## 3단계: '좋은' 데이터의 기준 이해하기
- 비즈니스 이해관계자들에게 "누가, 무엇을, 언제, 어디에서, 왜"를 질문함으로서
	- 데이터 품질이 그들에게 어떤 의미인지, 어떤 데이터가 실제로 가장 중요한지 이해할 수 있음
	- 이 과정을 통해 다음과 같은 핵심 성과 지표 개발이 가능함
- [[데이터 신선도]]
- [[데이터 분포]]
- 데이터 볼륨: 테이블 X의 크기는 감소하지 않음
- 데이터 스키마: 이 테이블의 필드는 삭제하지 않음
- 데이터 계보: 테이블 X 내의 모든 데이터는 출처와 사용자의 표기, 관련 메타데이터 정보를 포함
- [[data_downtime|데이터 다운타임]] 또는 데이터 가용성
	- 발생한 사건의 수 X(사건 탐지에 소요된 시간 + 사건 해결에 소요된 시간)
	- 테이블 X의 데이터 다운타임은 연중 Y시간 이하여야 한다
		- 데이터 다운타임의 SLA 예시
	- 데이터 다운타임의 각 요소에 대한 SLA는 실행 가능해야 함
		- 데이터 사건을 X% 감소시킨다
		- 탐지에 소요되는  시간을 X% 감소시킨다
		- 해결에 소요되는 시간을 X% 감소시킨다
- 쿼리 속도
	- 데이터 조직에 대한 포스팅을 다루는 블로그 locally optimistic
		- 평균 쿼리 속도를 기준으로 시작하는 것도 좋지만
		- X%의 쿼리를 Y초내에 수행한다와 같이 자세한 기준을 세워야 함
- 데이터 수집: 외부 파트너의 책임을 관리하는데 효과적
	- 데이터는 파트너 Y에게서 매일 오전 5시에 수신
- 위와 같은 절차를 통해 비즈니스 중요도에 맞춰 세분화된 경고 규칙 구성 가능

## 4단계: 주요 데이터셋에 SLA, SLO, SLI 설정
- 데이터 파이프라인 SLA의 설정은
	- 데이터 신뢰성 향상을 위한 중요한 단계이자 데이터 인증 프로그램에도 핵심적인 요소
- SLA는 구체적이어야 하며 SLO, SLI를 통해 측정 가능해야 하고, 달성 가능해야 함
- SLA는 합의한 기준치뿐 아니라 합의 당사자 간의 관계에 대해서도 정의함
	- SLA는 평소 상황과 이슈 발생시에 누가 무엇에 책임을 지는지 설명됨
- **효과적은 SLA는 현실적이어야 함**
	- 데이터는 항상 신뢰할 수 있어야 한다 => 모호함
	- 좋은 SLA는 구체적이고 자세해야 함
		- e.g. 해당 데이터셋이 비즈니스에 중요한 이유는 무엇이고, 기대하는 바는 무엇임
		- 그런 기대는 언제 어떻게 충족해야 하는지
		- 데이터 위치와 관련된 이들은 누구인지 등을 모두 묘사
	- **특정 목표에 집중한 SLA를 세우기**
- SLA가 충족되지 않을 경우 어떻게 대응해야 하는가
	- e.g. 테이블 X의 데이터는 매일 오전 8시에 업데이트 되어야 한다 x
	- 팀 Z는 테이블 X의 데이터가 매일 오전 8시에 업데이트 되는지를 확인한다
	- 이상 경고 발생 2시간 내로, 팀은 문제를 확인하고 관련된 당사자들에게 이를 알린 뒤, 이슈의 근본 원인 분석 시작
	- 영업일 하루 이내로 업무를 생성하고, 이슈 해결의 진행 상황에 대해 전사적으로 공유
- 이 정도 수준의 명료함, 조직 구성을 갖추려면 **좋은 데이터의 기준**이 무엇인지 이해하기 위해
	- 팀이 초기부터 자주 이해관계자들과 이해를 합일 시켜야 함
	- 여기에는 데이터 팀 뿐만 아니라 비즈니스를 담당하는 조직도 포함되어야 함
	- 좋은 SLA에는 실제 서비스의 운영과 데이터 사용자들의 사용 방식에 대한 파악이 필수이기 때문
- 브랜든의 다른식 접근, '테이블 X의 데이터는 매일 오전 8시에 업데이트 된다'라는 식의 SLA와
	- 'SLA를 99% 수준으로 충족시킨다'라는 SLO를 구별하여 사용함
	- 그러나 접근법이 무엇이든 그는 실현 가능성이 중요하다고 강조함
	- 실제로 그의 고객중 대부분은 데이터 인증 프로그램을 우선 실행하는데 초점을 맞추고, 그 뒤에 오래된 것들을 정리해 나가는 식으로 보완
- 대부분 뛰어난 데이터 팀들은 **가장 주요한 테이블과 데이터셋을 인증하는 데에서 출발**
	- 비즈니스에 가장 많은 가치를 가져오고, 가장 많은 쿼리가 발생하고, 가장 사용자가 많거나 의존성이 높은 테이블이나 데이터셋
	- 브론즈, 실버, 골드와 같은 등급을 구분해 서비스 및 지원 수준을 구별하는 등급 제도를 시행하는 조직도 있음

## 5단계: 커뮤니케이션 및 사고 관리 절차 구축하기
- 이슈가 발생할 경우, 이에 대한 경고는 어디에서 어떻게 전송되는가?
- 팀 내외부적으로 이슈에 대한 대응책, 진행 상황은 어떻게 전달되는가
- 이 질문에 대한 대답은 기본적이고 당연할 수 있으나
	- 분명하고 투명한 커뮤니케이션이야 말로 책임감 있는 데이터 문화를 구축하는데 필수적
- 대부분의 조직이 슬랙, 페이저듀티, 마이크로소프트 팀즈 서비스를 이용해
	- 경고 안내를 전달받고 이에 대한 확인, 대응에 대한 논의 진행
- [[CHAP6_대규모_데이터_품질_문제_해결]]에서와 같이
	- 이를 통해 빠른 대응을 하는 동시에 전사적으로 상황에 대해 투명하게 공유함으로써 건강한 사고 관리 업무 체계를 구성함
- 서비스 중단이 발생할 때, 이를 전사적으로 공유하는 방안에 대해서도 고민해야 함
	- 예를 들어, 대규모 생산 중단 경고 발생시, 온콜 엔지니어는 이 소식을 전사에 어떤 식으로 전달해야하는가
	- 어디에 이 소식을 알리고, 이후 소식은 얼마나 자주 공유해야 하는가

## 6단계: 데이터 인증 절차 설계하기
- 마침내 측정 가능한 목표, 투명한 책임 구조, 명확한 커뮤니케이션 절차, 문제 해결 및 대응에 관한 기준이 담긴 SLA를 갖추게 됨
- 팀의 성공을 위한 각종 도구, 사전 대책이 마련된 것
- 마지막 남은 단계는 이해관계자를 위해 데이터 자산에 최종적으로 인증하고 이를 알리는 것
- **데이터 인증 절차를 분산할 것을 추천**
	- 결국 인증 절차는 **팀들이 더 빠르고 유연하게 확장**할 수 있도록 돕기 위해 설계된 것
	- 각 도메인 수준에서 중심이 되는 규정을 만든다면, 관료주의와 같은 불필요 요소 덜어낼 수 있음
- 그리고 이러한 인증 절차 과정은
	- 데이터 검색 솔루션, 자체 개발 솔루션 또는 다른 형태의 데이터 카탈로그 등을 통해
	- 데이터 조직이 테이블에 태그를 지정하고, 데이터를 검색하고, 사용하는 과정으로 이루어짐

## 7단계: 데이터 조직과 데이터 사용자 교육하기
- 테이블에 인증 태그를 붙인 것만으로 분석가들의 업무가 깔끔히 정리되는 것은 x
	- 적절한 절차에 따른 교육도 필요하며, 이는 상황에 따라 의무 교육으로 지정될 필요도 있음
	- 이슈에 대한 경고 및 커뮤니케이션 수준을 세부적으로 조정하는 일도 필요
- 때로는 별다른 조치가 필요하지 않는 경고를 받는 것도 바람직
	- e.g. 팀에서 새 데이터 소스를 추가하면 갑자기 특정 테이블의 크기 증가
	- 오류가 생기거나 조치가 필요한 사항은 아니나, 팀이 이 상황을 인지하는 것이 좋음
	- 누군가에게는 이슈가 되지 않는 일이 팀의 다른 누군가, 다른 도메인의 누군가에게는 중요할 수 있기 때문
- 하지만 너무 많은 경고는 피로감을 불러일으키기도 함
	- 만약 팀이 경고를 무시하기 시작하면 모니터링을 조정하거나
	- 커뮤니케이션 채널을 분리하는 등의 방식을 통해 중요한 정보 위주로 잘 드러날 수 있도록
	- 경고를 최적화해야 할 시점이라는 신호일 수 있음
- 그럼에도 데이터 조직은 데이터 사용자들 앞에서 위축될 필요 x
	- 지금까지 데이터 사용자들의 요구에 맞춰 데이터 품질을 보장하기 위한 강력한 시스템 도입
	- 이제 데이터 사용자들이 여러분의 업무를 더욱 객관적으로 이해할 수 있도록 돕고, 그들에게 솔루션 사용에 필요한 정보를 제공하면 됨
- 데이터 인증 프로그램이 실제로 작동하는 모습은 아름다움
	- 데이터 엔지니어가 데이터셋 책임자와 함께 인증 태그를 테이블에 지정
	- 분석가가 이를 대시보드로 가져와 사용할 수 있도록 [[data_warehouse|데이터 웨어하우스]]에 반영
	- 이 과정을 통해 데이터 다운타임을 방지하며, 결과적으로는 최소한 빈도라도 줄어들 것
- 그러나 적절한 절차와 문화가 갖춰지지 않는 이상, 데이터의 신뢰성을 인증하고 조직의 신뢰를 구축하는 일이 사실상 매우 어려움
	- 기술은 도움이 될 수 있으나, 기술만으로는 우수한 데이터 상태를 온전히 구현하기 어려움
- 따라서 명확한 SLA와 함께 데이터 인증 프로그램을 실행했다면,
	- 데이터 품질의 문화 및 조직적 차원의 장애물을 헤쳐 나갈 수 있는 방안으로
	- 비즈니스의 강점과 요구사항을 충족하는 조직 구조를 만들어 나가야 함

# 8.9. 적합한 데이터 조직을 찾는 토스트의 여정
- 어느 스타트업의 데이터 책임자
	- 조직 문제: 중앙에 집중하느냐, 분산시키느냐
	- 이는 실제로 중요한 문제
	- 데이터 리더들이 애자일 방법론을 통해 회사의 성장에 따라 확장되는 데이터 조직을 구축하는 이야기
- 데이터 리더들은 조직을 빠르게 확장해야 한다는 임무를 받음
	- 채용 계획, SLA 등 각 단계마다 회사에 통찰력 있는 정보 제공 필요
- 가장 큰 과제중 하나: 데이터 조직의 적절한 보고 체계 구축
	- 데이터 수요 증가 => 중앙 집중화된 데이터 팀, 병목현상 증가
	- 분산된 팀 => 업무 중복 및 복잡도 증가
- 만약, 데이터 엔지니어링 조직 중앙 집중화, 데이터 분석가를 분산 구조식으로 배치
	- 그리고 분석 엔지니어들이 간극을 매우거나
	- 소수 데이터 분석가들이 CTO 아래에서 데이터 엔지니어와 함께 COO에게 직접 보고
	- 위와 같이 구성하면 회사의 우선순위, 전략도 완전히 바뀜

## 8.9.1. 소규모 조직으로 데이터 수요를 충족하기 위한 고군분투
- 2016년 토스트 합류, 200명의 직원. 데이터 전담 직원은 x
	- 회사는 항상 데이터를 통한 의사결정 우선시
- 도구, 절차, 기본적인 데이터 프로그램 구축
	- 데이터 조직 3명
	- 특히 리더들이 큰 의사 결정을 하기 전에 데이터를 참고하는 분위기
- 전사규모 2배 증가
	- 중앙 집중형 데이터 조직으로는 빠르게 성장하는 데이터 기반 조직의 넘치는 수요 감당이 어려움
	- 데이터에 대한 수요가 능력을 벗어나기 시작
	- 수요에 부응할 방법을 찾기
## 8.9.2. 조직의 초고속 성장을 지원하기 위한 분산형 구조
- 각 부서의 자체적 데이터 수요 충족 -> 분산 구조 전환
- 세일즈, 고객 성공 파트에서 자체적 데이터 분석 진행
	- 소규모 데이터 팀에서 대응하기 어려웠기 때문
- 10명의 데이터 전문가 분산형 팀, 각 사업부내에서 업무
	- 2배로 성장한 회사, 데이터 수요 충족 가능
	- 데이터 조직에서는 e2e에서 s3, airflow, snowflake 등 최신 분산형 스택으로 마이그레이션, 데이터 기술 스택 재구축
	- 각 사업부 데이터 분석가, 데이터 중앙 조직과 긴밀한 관계 유지
		- 완전 중앙 집중형 데이터 조직 및 분산형 데이터 조직의 하이브리드 구조
		- 하지만 데이터 조직의 지속 확장으로 이 구조도 문제가 발생
- **데이터 일관성 문제**
	- 부정확한 데이터가 해로울 수 있음
	- 전사 차원의 데이터 옵저버빌리티, 신뢰를 구축하기 위해서는 분석가, 기술리더, 이해관계자 간의 커뮤니케이션이 중요
	- 비즈니스가 더욱 커지고 복잡해지면서 전체를 보려면 결국 분석가가 필요
		- 분산형 모델에서도 성능, 운용성에 대한 표준 설정시
		- 분석가가 다른 분석가 및 기술 리더화 협업 필요

## 8.9.3. 데이터 신뢰를 위한 조직 재구성 및 중앙집중화
- 고객 성공팀, 사업 개발팀에서 각각 일하던 분석가를 중앙 분석 팀으로 배치
	- 이 당시, 중앙 집중형 구조, 분산형 구조, 하이브리드 3가지 선택지가 존재
	- 단, **분석 중심의 팀 규모와 업무 범위**에서는 **하이브리드 구조**가 효과적
- 중앙 집중형 구조는 장기적인 해결책이 아닐 수 있음
- 결국 데이터 조직의 핵심은 **모든 사람이 비즈니스에 최대한의 가치를 더할 수 있도록 만드는 것**
- 200명, 500명, 1000명 등 조직의 규모에 따라 알맞은 방시이 모두 다를 수 있음
- 팀의 구조가 어떻든 중요한 것은
	- 비즈니스 요구사항을 충족하는 동시에
	- 기술 리더들이 분석의 병목현상이 아닌 원동력이 되도록 하는 것

### 중앙 집중형 구조
- 장점
	- 분석에 대한 기준과 기대치 일치
	- 팀내 협력을 통해 더 많은 조언과 빠른 성장 가능
	- 필요시 상대적으로 자원 이동이 용이
	- 전체 비즈니스의 파악 용이
	- 업무 중복 감소
- 단점
	- 일반적으로 속도가 느림
	- 현업의 관점 부재로 데이터의 해석이 무의미
	- 인력 할당에 관한 다툼이 상당할 수 있음
	- OKR의 일치 복잡
- 특징
	- 분석 팀이 별도 존재
	- 분석가나 분석 팀이 각기 다른 부서나 사업무의 업무에 할당되거나 간접적인 보고 체계를 가질 수 있으나
		- 모든 보고는 중앙 허브로 이루어짐

### 하이브리드 구조
- 장점
	- 각 구조의 장점만 취함
	- 부서별 팀이 기본 업무를 진행하는 동안 중앙 팀은 더 큰 과업 진행 가능
- 단점
	- 중앙 팀과 각 분산 팀 간의 책임 소재 혼선
	- 거버넌스 이슈: 각 부서마다 서로 답이 다를 수 있음
- 특징
	- 분석 팀이 별도로 존재하나, 일부 부서나 사업부에 포함된 분석가 또는 분석 팀의 보조가 함께 분석 기능 담당
	- 중앙의 분석 팀으로 간접 보고가 이루어지기도 함
	- 대부분의 기업이 이런 형태를 취하며, 형태는 매우 다양

### 분산형 구조
- 장점
	- 분석가들이 현업의 관점을 갖출 수 있음
	- 분석 속도가 빠르고 비즈니스에 따라 맞춰갈 수 있음
	- 각 팀이 분석 자원 쉽게 조정 가능
	- OKR에 직접 일치
- 단점
	- 분석에 대한 기준과 기대치 상이
	- 기술적 성장 어려움
	- 분석가들이 전체 비즈니스를 이해하기 어려움
	- 불필요한 업무 발생
	- 보고 구조에 따라, 선입견에 따라 분석이 진행될 수 있음
- 특징
	- 부서 및 사업부에서 자체적으로 분석 기능 담당
	- 분석가 또는 분석 팀은 각 부서 또는 사업부의 리더에게 보고


## 8.9.4. 데이터 팀을 확장할 때 고려할 사항들
- 최종적으로 일부 분산형 구조의 요소를 포함한 중앙 집중형 조직 구조
	- 데이터 프로덕트에 대한 책임, 거버넌스 증진
	- 확장성을 갖춘 모듈식 데이터 스택 확보
- 초고속으로 성장하는 기업에서 데이터 리더들이 참고할 수 있는 내용
	- 모든 방안은 회사의 비즈니스 요구를 가장 잘 충족하는 접근법이어야 하며
	- 이 요구는 시간이 지남에 따라 변경될 수 있다는 점을 생각할 것
- 리더는 늘 민첩해야 하며, 데이터 조직은 비즈니스 요구에 잘 적응해야 함

### 한 가지 상황만 제외하고 전문가 대신 제너럴리스트 채용
- 가장 우선적으로 채용해야할 전문가, 데이터 엔지니어
- ETL 파이프라인을 구축 및 유지 관리하고, 회사의 분석 수요에 맞춰 데이터 인프라 확장 => 필요한 기술 자원은 늘 부족
- 그렇기 때문에, 이것저것 조금씩 다 할 수 있는 분들을 찾아야 하나
	- 데이터 엔지니어는 예외임
- 분석가를 몇 명 채용했다면, 그 다음엔 바로 데이터 엔지니어를 알아봐야 함

### 조직 구축 초기부터 다양성 중시
- 장기적으로 성공하는 조직 구축 => 구축 초기부터 다양한 경험, 배경을 가진 사람들에게 투자하기
- 동질성은 혁신을 저해하며,
	- 데이터 분석가들과 엔지니어들이 데이터 사용자의 다양한 요구 사항과 관점을 이해하는 데도 장애물이 됨
- 그러나 채용 및 성장의 KPI에 이를 반영하지 않는 이상, 조직이 빠르게 성장하는 시기에는 이를 놓치기 쉬움
- **초기에는 다양성을 염두해 두어야 함**
	- 작은 규모의 데이터 팀에서는 신경 쓰지 않으면, 어느새 비슷한 경험 및 생각을 가진 사람들만 모이게 됨
	- 하지만 데이터 팀에 필요한 건, 비슷한 사람이 아닌 다양한 관점과 배경
- 단, '다양성을 갖춘 팀을 구성해야 한다'는 말과 이를 실제로 수행하는 일은 전혀 다른 차원의 일
- 데이터 리더들을 위한 팁
	- 경영진, 인사 팀과 협력하여 다양한 경험과 배경을 포함하는 직무 설명서 작성
	- 데이터 조직 소속이 아니더라도 채용 과정에 다양한 이들을 포함할 것
	- 전통적인 직무나 역할에서 벗어난 후보자들을 모집할 수 있도록 그물을 넓게 펼치기
		- 직무나 역할 정의는 꾸준히 변하고 있음
	- 성별과 인종 등의 요소는 걷어내고, 자격 요건과 경험에만 집중하는 블라인드 채용
	- 다양한 배경을 가진 사람들은 다양한 배경을 가진 이들이 모인 조직에 합류하고 싶어 함
		- 시간이 지날수록 그들을 채용하기는 어려워짐
		- 지금 시작하지 않으면 갈수록 어려워짐

### 조직을 바꿀 때는 오버커뮤니케이션을 하기
- 많은 조직이 재택으로 일하며 이메일, 슬랙 등으로 커뮤니케이션 하는 원격 근무 시대 => 커뮤니케이션 매우 중요
- 기업이 핵심 가치 제안과 같은 메세지를 고객에게 꾸준히 반복하여 전달해야 하며
	- 이는 데이터 리더들이 이해관계자, 업무나 조직 구조 변경에 대해 소통할 때도 마찬가지
- e.g. 분산형 구조 방식으로 3개월 정도 고객 성공 조직에서 일하며 그 리더에게 보고를 하던 분석가
	- 앞으로 중앙 분석 조직의 리더에게 보고를 하는 것으로 체계 변경
	- 그러면 이를 알리는 것은 물론, 해당 변화로 조직의 목표가 달라지지 않는다는 사실도 상기시켜야 함
	- 그러면 데이터 조직이 더 이상 분산형 구조로 일하지 안헥 되더라도,
		- 이해 관계자들은 핵심 비즈니스 목표를 달성하는 데 필요한
		- 시의 적절하면서도 정확한 데이터 분석을 문제 없이 받아볼 수 있다는 사실 이해할 것
- 조직의 구조적 변화는 필연적으로 각 기능 조직을 비롯한 이해관계자 및 서비스를 제공하는 데이터 조직 관계에 영향
	- 하지만 그렇더라도 KPI에는 변화가 없다는 사실을 명시
	- 이렇게 반복하여 소통하면 조직 사이의 관계 개선 및 변화 극복에 도움이 됨
- **분석가가 비즈니스 부문의 리더들에게 보고하는 경우에는 데이터를 근거로 충분히 다른 의견을 개진할 수 있도록 해야함**
	- 그렇지 않으면 입증되지도 않은 가설을 뒷받침하도록 데이터를 가져다주는 이상한 협업 구조가 됨
	- 분석가가 하나의 팀에 모두 모이면 서로 배울 점은 많아지나, 다른 부서에 영향을 주는 일은 어려워짐
	- 토스트의 데이터 조직은 1년 반동안 중앙 집중형 구조로 비즈니스 요구사항을 충족해 왔음

### '[[단일 진실 공급원]]'을 과대평가 하지 말기
- 단일 진실 공급원 혹은 황금 데이터라고 불리는 개념이 유명한 이유?
	- 메트릭을 일관되게 정리하고 깨끗한 데이터를 유지하려는 노력을 통해 회사는 올바른 방향을 가고 있다고 믿게됨
	- 하지만 초고속으로 성장하는 스타트업에서 데이터 신뢰성을 직접 챙기는 동안에는 단일 진실 공급원은 그렇게 시급하지 않음
		- e.g. 다양한 실험과 프로젝트 참여, 테이블 최신 여부, 데이터 셋의 담당자 여부, 50 -> 500 행의 증가 이유 등
- **항상 사람들에게 단일 진실 공급원을 과대평가하지 말길 권장함**
	- 100% 정확성을 추구해야 할 때가 있는 반면, 굳이 그럴 필요가 없는 경우도 많음
	- 대개는 방향만 정확하면 괜찮음
- **완벽함을 추구하다 보면 시간을 낭비하는 경우가 생김**
	- [[80대 20법칙]]
- 데이터는 늘 지저분하기 마련이고, 완벽하기란 어려움
	- 그러므로 데이터의 상태와 정확성을 세분화해 관리하기 보다는 전체적인 조망을 우선시하여 일을 처리할 것
- 커뮤니케이션 능력이 뛰어난 좋은 사람을 고용하면 나머지는 쉬워짐
	- 좋은 사람은 다른 좋은 사람을 불러오기 때문에, 이렇게 하면 뛰어난 사람 고용 가능
	- 하지만 아무리 똑똑한 사람을 데려다 놔도, 그들이 기술을 잘 알지 못하는 사람들과 분석 내용과 관련한 소통을 원활하게 하지 못한다면 조직 성공이 어려움

