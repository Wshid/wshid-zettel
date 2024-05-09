### 2.1. 들어가는 글

의미 기반 검색(Semantic Search) 분야
LLM을 기반으로 한 [[텍스트 임베딩]]을 생성하는 기능

텍스트 -> 벡터로의 대응은 일종의 **hash**로 생각할 수 있음
- 벡터 -> 텍스트는 불가능하나
- 인코딩된 상태에서 점수 비교가 가능한 **추가적인 이점** 확보가 가능
언어와 문맥의 이해를 통해 문장이나 단어 내포 의미 포착
- 언어 작업에 있어서 정확한 모델 개발 가능


### 2.2. 작업
- 검색엔진에 입력하는 단어는, 항상 예상하는 검색 결과에 사용된 단어와 정확하게 일치하지 않을 수 있음
- 동일한 단어가 검색한 것과 다른 의미를 가질 수도 있음

#### 2.2.1. 비대칭적 의미 기반 검색
- [[의미 기반 검색 시스템]]
- 비대칭(asymmetric)이란 불균형이 있는 것
	- 입력 쿼리의 의미 정보(기본적으로 크기)
	- 검색 시스템이 검색해야하는 문서/정보
- 기본적으로 둘중 하나는 짧음
	- e.g. 네 단어 정도의 쿼리는, 결과로 나오는 문단보다 짧음
-  쿼리에 정확한 단어를 사용하지 않더라도, 정확한 검색 결과를 얻는 것이 목표

### 2.3. 솔루션 개요

#### 단계
- 1단계, 문서 저장
	- 임베딩을 위한 문서 저장(항목의 단락 설명)
	- 의미 정보 인코딩을 위한 **텍스트 임베딩** 생성
	- 나중에 쿼리가 주어졌을 시, 검색 할 수 있도록 임베딩을 **데이터베이스**에 저장
	- ![[LLM_2_3.excalidraw]]
- 2단계, 문서 검색
	- 사용자에게 전처리되고 정리할 수 있는 쿼리 존재(사용자가 항목 검색시)
	- 임베딩 유사도를 통해 후보 문서 검색
	- 필요한 경우 후보 문서의 순위를 재순위화(re-ranking)
	- 최종 검색 결과를 사용자에게 반환
	- ![[LLM_2_3_2.excalidraw]]
	- 문서에 사용한 것과 동일한 임베딩 체계를 활용
	- 쿼리를 임베딩하고, 이전에 저장된 문서화 비교
	- 가장 가까운(적합한)문서 반환

### 2.4. 구성 요소

#### 2.4.1. 텍스트 임베더
- Text Embedder
	- 텍스트 문서나 단어 또는 구문을 받아 벡터 변환
	- 벡터는 입력된 텍스트 마다 고유, 구문의 맥락적 의미를 포착
- **텍스트 임베더의 선택**은 텍스트를 벡터로 표현하는 품질을 결정하기 때문에 중요
- LLM으로 벡터화 하는 방법은 아래 방식 모두 존재
	- open source
	- closed source: 예시에서는 openAI의 embedding 제품 사용
- OpenAI의 Embedding
	- 고품질의 벡터를 빠르게 제공할 수 있는 강력한 도구
	- but, closed인 만큼 잠재적인 편향에 대한 제어가 제한적
	- 기반 알고리즘에 접근 불가. 발생하는 문제를 해결하는데 어려울 수 있음

##### 무엇이 텍스트를 유사하게 만드는가
- 텍스트를 벡터로 변환
- 텍스트 조각끼리 얼마나 **유사**한가를 파악 => 수학적으로
- [[코사인 유사도]]
- [[내적]]이나 유클리드 거리와 같은 다른 유사도 지표를 사용할 수 있음
- Open AI의 임베딩 엔진은 특별한 속성 존재
	- 벡터의 크기(길이)는 기본적으로 길이 1로 정규화 => 수학적으로 이점
- 수학적 이점
	- [[코사인 유사도]] = [[내적]]
	- [[코사인 유사도]]와 유클리드 거리는 동일한 순위의 결과 반환
- 즉, [[코사인 유사도]]를 통해서 **의미적으로 두 구문이 얼마나 유사한지**를 알 수 있음


##### OpenAI의 임베딩 엔진
- 의미적으로 유사한 항목을 서로 가까이 배치하는 [[임베딩]]매커니즘에 의존
	- 항목이 실제로 유사할 때 [[코사인 유사도]]가 큼
- 임베딩을 위해 OpenAI외에 다양한 임베딩 엔진 사용 가능
```python
import openai from openai.embeddings_utils import get_embeddings, get_embedding

openai.api_key = os.environ.get('OPENAI_API_KEY')

# 텍스트 임베딩에 사용될 엔진 설정
ENGINE = 'text-embedding-ada002'

# 지정된 엔진을 사용, 주어진 텍스트의 벡터 표현 생성
embedded_text = get_embedding('I love to be vectorized', engine=ENGINE)

# 예상 크기(1536)과 일치 여부 확인
len(embedded_text) == 1536
```
- 텍스트 임베딩을 위해 여러 임베딩 엔진 옵션 제공
	- 여러 수준의 정확도
	- 다양한 유형의 텍스트 데이터에 최적화 가능
- 한 번에 여러 텍스트를 `get_embeddings`에 전달 가능
	- 한번의 API 호출로 모두에 대한 임베딩 생성 가능
	- 각각으로 호출하는 것보다 효율적
##### 오픈소스 대안
- 텍스트 임베딩을 위해 여러 오픈소스를 사용할 수도 있음
- BERT를 이용한 Bi-encoder
	- 다양한 자연어 처리작업에 입증됨
	- 딥러닝 기반 알고리즘
- 많은 오픈소스 저장소에서, 사전 훈련된 Bi-encoder를 찾을 수 있음
- Sentense Transformers 라이브러리는 다양한 자연어 처리 작업을 위해 사용할 수 있는
	- 사전 훈련된 모델을 제공
- Bi-encoder는 두 개의 BERT 모델을 훈련시키는 과정 포함
	- 각각 입력 텍스트를 인코딩, 출력 텍스트를 인코딩
	- 이 두 모델은 입력, 출력 텍스트 쌍이 최대한 유사하도록 큰 텍스트 데이터 말뭉치에서 동시 훈련
	- 결과적으로 얻어진 임베딩 들은 입력, 출력 텍스트 사이의 의미적 관계 포착
	```mermaid
	graph LR;
		문장A[문장 A] --> BA[BERT] --> PA[pooling] --> u --> 코사인유사도;
		문장B[문장 B] --> BB[BERT] --> PB[pooling] --> v --> 코사인유사도;
		코사인유사도 --> Bi-encoder
	```
	- pooling: 벡터의 크기를 줄이는 연산

- 사전 훈련된 `sentence_transformer` 패키지의 Bi-encoder를 사용하여 텍스트를 임베딩

```python
from sentence_transformers import SentenseTransformer

# multi-qa-mpnet-base-cos-v1 사전 훈련 모델로 SentenceTransformer 모델 초기화
model = SentenceTransformer(
							'sentence-transformers/multi-qa-mpnet-base-cos-v1'
)

# 이베딩을 생성할 문서 리스트 정의
docs = [
		"Around 9 million people live in London",
		"London is known for its financial district"
]

# 문서에 대한 벡터 임베딩 생성
doc_emb = model.encode(
					   docs, # 문서들, 반복 가능한(iterable) 문자열
					   batch_size=32, # 해당 크기로 임베딩 일괄 처리
					   show_progress_Bar=True # 진행 막대 표시
)

# 임베딩의 형태 (2,768)
# 길이 768, 임베딩 2개
doc_emb.shape # == (2,768)
```
- multi-qa-mpnet-base-cos-v1
	- 다중 작업 학습, 특히 질문-답변과 같은 텍스트 분류와 같은 작업을 위해 설계
	- 비대칭 데이터를 사용하여 사전 훈련
	- 짧은 문서, 긴 문서 잘 처리 및 비교 가능
- 문서에 대한 벡터 임베딩 생성
	- `SentenceTransformer::encode`함수 사용
	- 결과적으로 얻어진 임베딩은 `doc_emb`변수에 저장
- 알고리즘마다 특정 유형의 텍스트 데이터에서 더 나은 성능 발휘
	- 벡터 그기도 다를 수 있음
	- 알고리즘의 선택 => 임베딩 품질에 상당한 영향을 미침
- open source는 closed 보다
	- 더 많은 맞춤화와 파인 튜닝 필요
	- 임베딩 과정에 대한 큰 유연성과 제어 제공
- 이 책의 예시 코드 링크
	- https://github.com/sinanuozdemir/quick-start-guide-to-llms