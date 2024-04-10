# DBT Model with OpenFGA
- DBT Model을 OpenFGA의 개념으로 녹여 관리할 수 있을까?

## OpenFGA의 Google Drive 사용 예시
- https://openfga.dev/docs/modeling/advanced/gdrive

## 현재 필요한 부분
- 특정 모델마다 정의된 상태
- 모델의 owner, writer, reader 등이 필요함
  - owner는 모델을 생성, 삭제 할 수 있는 사람
    - 모델을 생성한 사람
    - 모델의 메타 수정 가능
    - 모델 데이터를 수정할 수 있음
    - 모델 제거 가능
  - writer 모델을 생성, 조작할 수 있는 사람
    - 모델 메타 수정 가능
    - 모델 데이터를 수정할 수 있음
  - reader 모델을 조회할 수 있는 사람
    - 모델 메타 조회 가능
    - 모델 데이터 조회 가능
    - 모델을 읽어, 후속 테이블 생성이 가능함
- 모델을 생성하게 되면 기본적으로 `owner`의 권한을 가짐

```yaml
model
  schema 1.1

type user

type document
  relations
    define owner: [user]
    define write: [user]
    define child_creator: [user] : 해당 모델 데이터를 읽고, 후속 모델 작성이 가능한 사람
    define viewer: [user]

```
```python
body = ClientWriteRequest(
        writes=[
                ClientTuple(
                    user="user:beth",
                    relation="commenter",
                    object="document:2021-budget",
                ),
        ],
)
```


## 모델 구조 수립
- model에 대해 owner, child_creator, viewer 권한 생성
```yaml

type user

type model
  relations
    define parent: [model]
    define owner: [user, group#member] or owner from parent
    define child_creator: [user, group#member] or owner or writer from parent
    define viewer: [user, group#member] or child_creator or child_creator from parent

# owner는 특정 그룹의 멤버에게 권한 부여..를 하면
# 일단 viewer만 가능하다고 가정
type group
  relations
    define member: [user]
```

## 모델 튜플 생성
```python

### 모델 owner 지정
body = ClientWriteRequest(
        writes=[
                ClientTuple(
                    user="user:wshid",
                    relation="owner",
                    object="model:wdl_base_service_type_d",
                ),
        ],
)
response = await fga_client.write(body, options)

## 모델 체크
body = ClientCheckRequest(
    user="user:beth",
    relation="commenter",
    object="document:2021-budget",
)

response = await fga_client.check(body, options)
```

## 도메인 생성 코드
```python
options = {
    "authorization_model_id": "1uHxCSuTP0VKPYSnkq1pbb1jeZw"
}
body = ClientWriteRequest(
        writes=[
                ClientTuple(
                    # make anne, beth, charles a member of the xyz domain
                    user="user:anne",
                    relation="member",
                    object="group:de",
                ),
                ClientTuple(
                    user="user:beth",
                    relation="member",
                    object="group:de",
                ),
                ClientTuple(
                    # make members of xyz domain viewers of document:2021-budget
                    user="group:de#member",
                    relation="viewer",
                    object="model:wdl_base_service_type_d",
                ),
        ],
)

response = await fga_client.write(body, options)
```
- `wdl_base_service_type_d`라는 모델에 대해 
  - `group:de#member`는 viewer 권한을 가짐
- `group:de#member`의 구성원으로 `user:anne`, `user:beth` 존재

## 폴더 생성 코드(예시)
- 어떻게 구조를 가져갈지 고민되는 부분
  - 해당 모델에 대한 하위 내용을 볼 수 있어야 할까?
  - 각각이 asset이 되므로, 굳이 필요가 없을 것으로 보임
    - raw table의 생성자는 하위 service 모델을 볼 수는 없음
```python
options = {
    "authorization_model_id": "1uHxCSuTP0VKPYSnkq1pbb1jeZw"
}
body = ClientWriteRequest(
        writes=[
                ClientTuple(
                    # Diane is a viewer of document:2021-planning
                    user="user:diane",
                    relation="viewer",
                    object="document:2021-planning",
                ),
                ClientTuple(
                    # document:2021-planning is a parent of document:2021-budget
                    user="document:2021-planning",
                    relation="parent",
                    object="document:2021-budget",
                ),
        ],
)

response = await fga_client.write(body, options)
```