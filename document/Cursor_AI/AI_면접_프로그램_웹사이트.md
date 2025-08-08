### Next.js 기본 세팅
```bash
npx create-next-app@latest .
# 모든 것 Yes
# Turbopack No
# import alias No
```

### PRD(Product Requirements Document) 만들기
- cursor내에서 prdprompt.md에 명세 작성
```
## 이 서비스는 다음과 같이 진행이 됩니다
- 사용자의 행동 라인 순서대로 나열
## 기술 스택
- 어떤 스택을 사용해야 하는지
## 지침
- 문서를 어떻게 작성해야 하는가
```
- 이후 GPT에 가서 질문
	- PRD를 만들어줌
	- 필요 없는 부분은 제거하도록 요청
- 이후 GPT에서
	- 지금까지 이야기한 내용을 바탕으로 최종 prd 문서를 markdown 형태로 작성해 달라 요청
- 이후 답변 받은 내용을 토대로 `prd.md` 파일 생성

### cursor에 문의
```bash
지금부터 ai 면접 프로그램을 만들꺼야
우선 이 prd 문서를 참고하여 진행을 하는데
맨 처음에 너가 진행을 해야할 것은 바로 전화번호를 입력하는 프론트엔드 디자인이야
다음 첨부한 사진을 참고하여 전화번호 입력 페이지를 먼저 만들어줘


이후 준비한 이미지를 첨부함
(단순 번호를 입력하는 이미지)
```
- `crtl + j`로 터미널 바로 활성화 가능
- 제작된 코드를 가지고 다음과 같이 명령 수행
```bash
npm run dev
```
- CSS에 관련된 부분은 추가 질의하여 정리 진행

### Make를 활용한 전화번호 에어테이블 등록
- Make 시나리오 등록
- Custom Webhook 생성: AIInterview
- cursor 요청
	```md
	사용자가 전화번호 입력 후 확인을 누르면
	{복사한 webhook 주소}
	위 웹훅으로 전화번호 전달해줘
	```
- Make: webhook response(200) 추가
- Make: save 및 활성화, Run once
- 이후 Local app에서 전화번호 입력 후 전송
- Make 상에서 webhook 수신 되었는지 확인
```md
웹훅으로 필요없는 정보들이 같이 반영이 되고 있어
다음 하단의 내역은 필요가 없으니 웹훅으로 전달하지마
{불필요 필드 전달}
```
- 데이터베이스로 Airtable을 사용
	- Google Sheets와 유사하게 생김
	- 자동화에 특화
- AirTable 접속 - Start from scratch
- AirTable: 전화번호 컬럼만 유지, 공백 Row는 제거함
	- Sheet를 사용하듯 여러 타입의 컬럼 추가가 가능함
	- 'AI 면접' 이름 정의
	- Table 1 -> '전화번호'
- Make: Create a record - AirTable 선택
	- `Custom webhook -> airfTable -> webhook response` 구조
- Make: Airtable Token, key 입력 필요
	- Airtable의 프로필 - Builder hub - PAT 관련 설정 존재
	- Create 이후: Scope는 모든 항목 일일히 선택
	- Acccess: All Resources
	- 생성된 토큰 복사 및 Make에 등록
	- Base: Airtable에서 생성한 프로젝트 이름 기입('AI 면접')
	- Table: '전화번호'
		- 만약 업데이트 되지 않았다면 새로고침
	- Record: 웹훅에서 전달받은 `contact` 필드 지정
- Make: Run Once 수행 이후, AirTable에 레코드 등록되었는지 확인

```md
다 구현이 제대로 되었는데
Make 웹훅에서 제대로 등록이 완료되었다는 응답을 받았을 때
등록 완료라는 문구가 나오도록 수정 하고
Make에서는 어떤 형태로 응답 전달을 하면 좋을지 응답 코드도 같이 알려줘
# Make에서 정의할 내용 다 정의를 해줌

지금 웹훅으로부터 응답을 받았을 때 전화번호 등록이 완료되었다는 팝업창이 나오는데,
이 부분이 웹훅으로 응답을 받았을 때 반응을 하도록 해줘
내가 말한 부분은 이 첨부된 이미지야
{현재 완료 페이지 같이 전달}


지금 모달 팝업이 두번 나와. 우선 사용자가 번호 등록 후 확인을 누르면
바로 모달 창이 하나 나오고 이후에 make로 응답을 받으면 동일한 모달 창이 뜨고 있어
사용자가 확인을 누른 후 바로 나오는 모달 팝업창은 없애주고
응답을 받을때까지 로딩 화면이 나오고 응답을 받으면 모달 팝업이 나오도록 변경해줘
```
- 정상 구현 확인