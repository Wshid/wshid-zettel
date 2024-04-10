# Data Governance
- https://www.databricks.com/kr/glossary/data-governance

## Data Governance란?
- 데이터가 가치를 창출하는지 확인하는 행위
- 비즈니스 전략 지원
- 단순한 도구/프로세스가 X
- 비즈니스 목표, 목적을 지원하는 문화에 초점
  - 프레임워크를 통해 비즈니스 전략에 데이터 관련 요구 사항 일치

## Data Governance는 Business에 어떤 장점을 제공하는가?
- Data Complexity가 높아지면서, DG를 찾는 조직이 늘어남
- 장점
  - Analytics, ML을 위한 일관적이고 우수한 품질의 데이터 제공
  - Insight 확보 시간 단축
  - HIPPA, FedRAMP, GDPR, CCPA와 같은 산업 규정에 대한 위험 관리, 규정 준수 지원
  - 조직내 모든 사람이 **데이터 중심적 결정**을 내릴 수 있도록 데이터 민주화
  - 사용자가 큰 클러스터를 시작하지 못하게 차단
  - 비싼 GPU Instance를 사용하는 것에 대한 가드레일을 만들어 비용 최적화

## Good DG Solutions
- 데이터 중심 기업 -> lakehouse에서 분석하기 위한 Data Architecture를 구축
- Data Lakehouse
  - Data Lake에 저장된 **방대한 데이터**에 직접 접근하여
  - DE, ML, Data Warehousing, Business Intelligence를 적용하는 아키텍처
- Data LakeHouse의 DG는 다양한 핵심 기능을 제공

### Unified Catalog
- 모든 데이터, ML Model, Analytic Artifacts, Data 객체의 메타 데이터 저장
- 기존 Hive Metastore와 같은 **다른 카탈로그에 있는 데이터**를 결합함
  - 마치 trino?

### 통합 데이터 액세스 제어
- 모든 데이터 자산 및 클라우드에 대해 **하나의 통합 권한 모델**을 제공
- 여기에는 개인 식별 정보에 대한 **속성 기반 액세스 제어**(ABAC)가 포함됨

### 데이터 감사
- Data Access는 **알림 및 모니터링**기능을 통해 한곳에서 감사하여 **책임감**을 부여

### 데이터 품질 관리
- 품질 관리, 테스트, 모니터링 및 정책 적용 기능으로 **데이터 품질을 안정적으로 관리**
- DownStream BI, Analytics, ML Workload에 **정확하고 유용한 데이터**를 제공할 수 있도록 함

### 데이터 리니지
- Data Source - Use
- LakeHouse의 데이터 흐름을 전체적으로 표현

### 데이터 발견
- DS, DA, DE가 손쉽게 데이터를 발견하고
- 관련 데이터를 빠르게 찾아내 참조
- 가치 창출 시간 단축

### 데이터 공유
- 데이터는 모든 클라우드와 플랫폼에서 공유됨

## 데이터 관리와 데이터 거버넌스 사이의 차이

### 데이터 관리
- 데이터 거버넌스 정책, 원칙 및 표준 준수
- 이를 통해 **신뢰 할 수 있는 데이터 제공**이 목적
- 프로젝트 중심적이고 단기적인 활동

### 데이터 거버넌스
- 장기적 이익 실현을 위한 프로그램
- 중앙 집중형 거버넌스 도구, 거버넌스 구현시 중요한 역할

### DataBricks Reference
- Databricks [Unity Catalog](https://www.databricks.com/kr/product/unity-catalog)
- Databricks [Delta Sharing](https://www.databricks.com/kr/product/delta-sharing)
