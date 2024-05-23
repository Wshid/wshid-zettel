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

#### 2.4.2. 문서 청킹
- 텍스트 임베딩 엔진이 설정되면
	- 큰 문서를 임베딩하는 어려움 고려
- 연구 논문과 같은 긴 문서를 다룰 때
	- 전체 문서를 단일 벡터로 임베딩하는 것은
	- 실용적이지 x
- [[ 문서 청킹]](document chunking)을 사용하기

##### 최대 토큰 범위 분할
- 문서 청킹의 한 방법
	- 최대 토큰 범위 분할(Max Token Window Chunking)
	- 구현하기 쉬운 방법
	- 주어진 최대 크기의 청크로 문서를 나누는 것을 포함
	- e.g. 토큰 범위 500 설정시 각 청크가 500 token보다 약간 작을 것으로 예상함
	- 비슷한 크기의 청크를 생성하는 것은 시스템을 일관성 있게 만드는데에 도움
- 우려 사항
	- 중요한 텍스트 일부를 나눠진 청크 사이에서 잘라낼 수 있음 => 문맥이 분리됨
	- 이 문제 보완을 위해, 토큰이 청크 사이에 공유되도록
		- 지정한 양의 토큰으로 겹치는 범위 설정
	- 다소 중복되는 느낌이 있으나, 더 높은 정확도와 대기시간을 얻을 수 있음
- 전체 교과서 가져오기
```python
# PDF를 가져오기 위해 PyPDF 사용
import PyPDF2

# PDF read by br mode
with open('../data/pds2.pdf', 'rb') as file:
	reader = PyPDF2.PdfReader(file)
	principles_of_ds =''
	for page in tqdm(reader.pages):
		text = page.extract_text()
		# 추출할 텍스트의 시작점 찾기, ']'에서 시작하는 텍스트 추출
		principles_of_ds += '\n\n' + text[text.find(' ]')+2:]

# 결과 문자열에서 앞, 뒤 공백 제거
principles_of_ds = principles_of_ds.strip()
```

- 이 문서를 특정 최대 토큰 범위로 분할
	- 중첩을 포함하는 또는 포함하지 않는 교과서 분할
```python
def overlapping_chunks(textr, max_tokens = 500, overlapping_factor = 5):
	'''
	max_tokens: 각 조각에 들어갈 최대 토큰 수
	overlapping_factir: 각 조각이 시작할 때 이전 청크와 중첩되는 문장의 숫자
	'''
	# 구두점을 사용하여 텍스트 분할
	sentences = re.split(r'[.?!]', text)

	# 각 문장의 토큰 수 얻기
	n_tokens = [len(tokenizer.encode(" " + sentence)) for sentence in sentences]
	chunks, tokens_so_far, chunk = [], 0, []

	# 튜플로 결합된 문장과 토큰을 반복하여 처리
	for sentence, token in zip(sentences, n_tokens):

		# if 지금까지의 토큰 수 + 현재 문장의 토큰 수 > 최대 토큰수:
		# 분할 조각을 청크 목록에 추가, 지금까지의 청크 및 토큰 리셋
		if tokens_so_far + token > max_tokens:
			chunks.append(". ".join(chunk) + ".")
			if overlapping_factor > 0:
				chunk = chunk[-overlapping_factor:]
				tokens_so_far = sum([len(tokenizer.encode(c)) fcor c in chunk])
			else:
				chunk = []
				tokens_so_far = 0
		# if 지금 문장의 토큰 수 > 최대 토큰 수
		# 다음 문장으로
		if token > max_tokens:
			continue

		# 그렇지 않다면, 문장을 조각에 추가하고 토큰 수를 총합에 더하기
		chunk.append(sentense)
		tokens_so_far += token + 1
	return chunks

split = overlapping_chunks(principles_of_ds, overlapping_factor=0)
avg_length = sum([len(tokenizer.encode(t)) for t in split]) / len(split)

# 비중첩 청킹의 문서수와 평균 토큰 길이
print(f'non-overlapping chunking approach has {len(split)} documents with average length {avg_length:.1f} tokens')

# 각 조각에 5개의 중첩 문장 포함
split = overlapping_chunks(principles_of_ds, overlapping_factor=5)
avg_length = sum([len(tokenizer.encode(t)) for t in split]) / len(split)

# 중첩 청킹의 문서수와 평균 토큰 길이
print(f'overlapping chunking approach has {len(split)} documents with average length {avg_length:.1f} tokens')
```
- 중첩을 사용하면 청크의 수가 증가(대체적으로 같은 크기의 청크)
- 중첩 비율이 높을 수록 시스템에 더 많은 중복성이 생김
- 최대 토큰 범위 방법은
	- 문서의 자연스러운 구조를 고려하지 않아
	- 정보가 청크 사이에 나누어 질 수 있거나
	- 중복된 정보가 있는 청크가 생길 수 있음
- 이러한 현상은 **검색 시스템**을 혼란스럽게 할 수 있음

##### 맞춤형 구분 기호 찾기
- 청킹 방법을 돕기 위해
	- 페이지 분리나 단락 사이의 새로운 줄과 같은
	- 맞춤형 자연 구분 기호 생성 가능
- 주어진 문서에 대해, 텍스트 내의 자연스러운 공백 식별 및
	- 임베딩되는 청크에 들어갈 의미있는 텍스트 단위 생성
- 중복을 포함하는 최대 토큰 범위: 청크간 중복 텍스트 내역 존재
- 중복이 없는 자연 공백 청킹: 청크1 = 페이지1, 청크2 = 페이지2, ...
- 자연 공백으로 교과서를 청크로 나누기
```python
from collections import Counter
import re

# principles_of_ds에서 하나 이상의 공백 있는 곳 찾기
matches = re.findall(r'[\s]{1,}', principles_of_ds)

# 문서에서 가장 빈번하게 발생하는 5개의 공백
most_common_spaces = Counter(matches).most_common(5)

# 가장 흔한 공백과 그들의 빈도수 출력
print(most_common_spaces)
```
- 가장 흔한 공백은, 연속된 두 개의 개행 문자(newlines)
	- 실제로 페이지를 구분하는 방법
- 청크를 구분하는 방식에 있어, 좀 더 창의적으로 접근하기 위해 ML을 활용할 수도 있음


##### 클러스터링을 사용하여 의미 기반 문서 생성하기
- 의미 기반으로 문서를 생성하기 위해 **클러스터링**을 사용
	- 의미적으로 유사한 작은 정보 청크를 결합하여 새로운 문서를 생성하는 것
- 청크에 대한 어떤 수정이든, 결과로 벡터를 변경할 수 있으므로
	- 약간의 창의성이 필요함
- scikit-learn의 병합 클러스터링 인스턴스
	- 유사한 문장이나 단락을 함께 그룹화 하여 새로운 문서를 형성
- 중복이 없는 자연 공백을 사용하는 청킹: 청크1 = 페이지1, 청크2 = 페이지2
- 중복을 포함하는 최대 토큰 범위 방법: 청크1=페이지1+페이지3, 청크2=페이지2, 청크3=페이지4, ...
- 의미적 유사성에 따라 문서 페이지 클러스터링
```python
from sklearn.cluster import AgglomerativeClustering
from sklearn.metrics.pairwise import cosine_similarity
import numpy as np

# embeddings라고 부르는 텍스트 임베딩 리스트 가정
# 모든 임베딩 쌍의 코사인 유사도 행렬 계산
cosine_sim_matrix = cosine_similarity(embeddings)

# AgglomerativeClustering 모델의 인스턴스 생성
agg_clustering = AgglomerativeClustering(
										 n_clusters=None, # 데이터 기반으로 최적의 클러스터 수 결정
										 distance_threshold = 0.1 # 클러스터간 모든 다른 임베딩 쌍과의 거리가 0.1보다 클때까지 클러스터 형성
										 affinity='precomputed', # 입력으로 미리 계산된 거리 행렬(1-유사도 행렬)을 제공
										 linkage='complete' # 컴포넌트 간의 최대 거리 기반, 가장 작은 클러스터를 반복적으로 병합하여 클러스터 형성
)

# 모델과 코사인 거리 행렬(1- 유사도 행렬)에 맞춤
agg_clustering.fit( 1 - cosine_sim_matrix)

# 각 임베딩에 대한 클러스터 레이블을 얻음
cluster_labels = agg_clustering.labels

# 각 클러스터에 있는 임베딩의 수 출력
unique_labels, counts = np.unique(cluster_labels, return_counts=True)
for label, count in zip(unique_labels, counts):
	print(f'Cluster {label}: {count} embeddings')
```
- 이 접근 방식은 일반적으로 의미적으로 연관성 있는 청크를 생성하나
	- 내용의 일부가 주변 텍스트와 맥락에 벗어날 수 있음
- 따라서 청크들이 서로 관련이 없을 때
	- **청크들이 서로 독립적일 때 잘 작동**

##### 청크를 나누지 않고 전체 문서 사용하기
- 전반적으로 가장 쉬운 옵션
- 단, 문서가 너무 길어서 텍스트 임베딩시 문맥 윈도우 한계에 도달할 수 있음
- 문서가 불필요한 문맥 포인트로 채워졌을 때
	- 결과 임베딩들이 너무 많은 것을 인코딩 하려고 하여, 전체적인 품질이 저하될 수 있음
	- 여러 페이지의 큰 문서에서 복합적으로 나타남
- 결과적으로 문서 임베딩을 위한 접근 방식 선택시
	- 청크 사용과 전체 문서 사용의 장단점 고려할 것


| 청킹 유형                 | 설명                               | 장점                             | 단점                                           |
| --------------------- | -------------------------------- | ------------------------------ | -------------------------------------------- |
| 최대 토큰 범위 청킹(중복 x)     | 문서는 고정된 크기의 범위로 나눔, 각 범위는 별도의 청크 | 단순하고 구현하기 쉬움                   | 청크 사이의 문맥이 잘리면서 정보 손실 발생                     |
| 최대 토큰 범위 청킹(중복 o)     | 문서는 고정된 크기의 중복 범위로 나눔            | 단순하고 구현하기 쉬움                   | 다른 청크 간의 중복된 정보                              |
| 자연 구분자를 기준으로 한 청킹     | 문서의 자연 공백이 청크 경계를 결정함            | 자연스럽게 끝나는 의미 있는 청크             | 적절한 구분기호 찾는 비용                               |
| 의미 기반 문서 생성을 위한 클러스터링 | 비슷한 청크 결합. 더 큰 의미 기반 문서 형성       | 문서의 전반적인 의미 포착하는 더 의미 있는 문서 생성 | 더 많은 컴퓨팅 자원, 구현 복잡도 높음                       |
| 전체 문서 사용(청킹 x)        | 전체 문서가 하나의 청킹                    | 단순하고 구현하기 쉬움                   | 임베딩을 위한 문맥 윈도우로 인해, 문맥이 과도하게 들어가, 임베딩 품질에 영향 |
|                       |                                  |                                |                                              |

문서를 어떻게 청킹할지 결정하면,
- 생성한 임베딩을 저장할 곳이 필요
로컬에서는 빠른 검색을 위해 **행렬 연산**에 의존할 수 있음
여기서는 클라우드 환경에 구축하는데 필요한 데이터베이스 옵션 확인