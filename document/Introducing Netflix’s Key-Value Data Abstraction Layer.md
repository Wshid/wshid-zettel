---
date: 2024-10-06
datetime: 2024-10-06 14:14:51
book: 
page: 
tags: 
references:
  - https://netflixtechblog.com/introducing-netflixs-key-value-data-abstraction-layer-1ea8a0a11b30
aliases:
---

# Introduction
- NoSQL database인 [[Apache Cassandra]]를 사용
- Key-Value DB 도입 이후, 데이터 저장소 오용으로 인한 문제 발생
	- 개발자는 여러 저장소에 걸친 복잡한 글로벌 배포
		- 일관성, 내구성, 성능 추론에서 문제
	- 개발자는 새로운 데이터 모델링 관행, 일반적이지만 중요한 Data Access Pattern을 지속적으로 학습
		- tail latency, idempotency(멱등성)
		- managing wide partitions with many rows
		- handling single large fat columns
		- slow response pagination
	- 여러 Native database API(수시로 진화, 하위 호환성을 만족하지 않는 api 포함)와의 긴밀한 결합
		- Micro-service의 data access를 유지하고 최적화하기 위한 조직 전체의 엔지니어링 노력 필요
- 위 과제를 해결하기 위해 Data Gateway Platform을 기반으로 하는 전체론적 접근 방식 채택
- 여러 가지 추상화 서비스를 만들기
	- Key-Value(KV) Data Abstraction Layer(DAL)
	- 데이터 액세스 간소화, 인프라 안정성 향상, Netflix가 요구하는 광범위한 사용 사례를 적은 비용으로

# Key-Value Service
- 분산 데이터베이스, 데이터 액세스 패턴과 관련하여 과제 해결 위한 도입
- 간단한 해시맵부터 복잡한 데이터 구조까지
	- 다양한 사용 사례 처리
	- 다재다능 및 효율적인 데이터 저장 솔루션
	- 높은 가용성, 조정 가능한 일관성, 낮은 지연 시간

## Data Model
- KV Abstraction, 2단계 맵 아키텍처를 중심으로 구축
	- Hash된 String ID
	- `SortedMap<Bytes, Bytes>`
	- 단순 복잡한 데이터 모델을 모두 지원함
> `HashMap<String, SortedMap<Bytes, Bytes>>`

- 구조화(`Records`) 되거나, 시간 순서가 지정된 복잡한 데이터 모델(`Events`)도
	- 계층적 구조를 효과적으로 처리하여 데이터 검색 가능
	- 간단한 사용사례(`flatMap<Key, Value>`)에서도 처리 가능
- KV Data의 높은 수준 시각화 예시
- ![image](https://github.com/user-attachments/assets/33eae8fe-ba51-4af4-af68-68a0102aa853)
```js
message Item {
	Bytes key,
	Bytes value,
	Metadata metadata,
	Integer chunk
}
```


### Database Agnostic Abstraction
- KV 추상화는 기본 데이터베이스의 구현 세부 정보를 숨기도록 설계됨
- 최적의 스토리지 시스템과 관계없이 app 개발자에게 일관된 인터페이스 제공
	- [[Apache Cassandra|Cassandra]]는 하나의 예시
	- EVCache, DynamoDB, RocksDB에서 작동 가능
- Cassandra로 구현할 경우
	- Cassandra의 분할 및 클러스터 기능 활용
	- `Record ID`는 partition key 역할
	- `item Key`는 clustering 역할
	- ![image](https://github.com/user-attachments/assets/3c051836-e0ca-432b-9c4a-ea5633dc6b11)

```sql
CREATE TABLE IF NOT EXISTS <ns>.<table> (
	id text,
	key blob,
	value blob,
	value_metadata blob,

PRIMARY KEY (id, key))
WITH CLUSTERING ORDER BY (key <ASC|DESC>)

```


### Namespace: Logical and Physical Configuration
- namespace
	- 데이터가 어디에 어떻게 저장되는지 정의
	- 기본 스토리지 시스템 추상화 및 논리적, 물리적 분리 제공
- 일관성 또는 대기 시간 대상과 같은 액세스 패턴의 **중앙 구성 역할**
- 각 ns는 `Cassandra`, `EVCache` 또는 여러 개의 조합과 같은 다른 백엔드 사용 가능
- 이러한 유연성 때문에 Data Platform은
	- 성능, 내구성, 일관성 요구사항에 따라
	- 다양한 사용 사례를 가장 적합한 스토리지 시스템으로 라우팅 가능
	- 개발자는 DB Solution이 아닌 데이터 문제만 제공
- 예시
	- 아래 네임스페이스(`ngsegment`)는 Cassandra cluster와 EVCache 캐싱 계층으로 백업
	- 내구성이 뛰어난 영구 저장소와 지연 시간이 짧은 지점 읽기 가능

```python
"persistence_configuration":[                                                     
  {                                                                             
    "id":"PRIMARY_STORAGE",                                                   
    "physical_storage": {                                                      
      "type":"CASSANDRA",                                                   
      "cluster":"cassandra_kv_ngsegment",                                  
      "dataset":"ngsegment",                                               
      "table":"ngsegment",                                                 
      "regions": ["us-east-1"],  
      "config": {  
        "consistency_scope": "LOCAL",  
        "consistency_target": "READ_YOUR_WRITES"  
      }                                              
    }                                                                         
  },                                                                            
  {                                                                             
    "id":"CACHE",                                                             
    "physical_storage": {                                                      
      "type":"CACHE",                                                       
      "cluster":"evcache_kv_ngsegment"                                     
     },                                                                        
     "config": {                                                                
       "default_cache_ttl": 180s                                               
     }                                                                         
  }                                                                             
]
```