## 주식/코인 정보 자동 조회 및 시각화

### 코인 정보 자동화
- upbit API를 데이터 가져오기
- 일 캔들 API
	- 로그인이 필요하지 않음
	- 인증 x
	- API에 대한 python api 예시도 제공함

#### 예시 활용
- 01.py 생성 및 예시 코드 복사
```bash
pip install requests
```

#### Cursor
- 파일로 결과를 저장할 예정
- 웹페이지를 만들때의 source로 활용하기 위함
```md
기준일은 2025-03-24로 변경하고, 일봉 100개를 가져오도록 수정해
응답받은 데이터는 {market}_{to}_{count}.json 형식으로 저장해
market, to, count는 request params의 값이야
```

```md
market을 비트코인으로 수정해줘
```

- cursor 설정에서 `Codebase Indexing`를 켜게 되면 따로 Json의 내용을 설명하지 않아도 처리 진행함

#### 간단한 웹페이지 만들기
```md
간단한 홈페이지 Html 파일 생성해줘
```
- extension에서 Live Preview를 설치함

#### 웹페이지 세팅
```bash
json 파일의 내용과 구조를 분석해
json 파일의 내용을 웹페이지에서 볼 수 있도록 Html로 시각화 해
```

