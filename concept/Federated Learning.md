---
date: 2024-04-28
datetime: 2024-04-28 13:47:56
book: 
page: 
tags: 
references: 
aliases:
---
여러 장치 또는 서버가 중앙 집중식 데이터 베이스에 공유 하지 않고도,
- 그들의 데이터를 로컬에서 모델 학습시 사용,
- 학습된 모델의 업데이트만을 중앙 서버와 공유
- 모델 개선하는 머신러닝 접근 방식
데이터 프라이버시와 보안을 향상 시키는 장점
- 개인 정보 보호가 중요한 의료, 금융 분양서 유용

## 장점
- 데이터 프라이버시와 보안
- 통신 효율성
	- 모델의 작은 업데이트만 공유
- 확장성
	- 여러 장치에서 동시 학습 진행
	- 대규모 분산 학습 시스템 구축에 적합

## 프로세스
- 중앙 서버에 초기 모델 배포
- 각 장치는 자신의 데이터를 사용하여 모델 로컬 학습
- 학습된 모델의 업데이트(e.g. 가중치)만을 중앙 서버로 전송
- 중앙 서버는 여러 장치로 받은 업데이트를 통합, 모델 개선
- 개선된 모델은 장치들에게 배포, 다음 라운드의 학습시 사용