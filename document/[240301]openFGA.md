# Zenzibar
- https://zanzibar.academy/
- https://www.osohq.com/post/zanzibar

## Zenzibar?
- Google에서 적용한 unique authorization model

### Basic
```bash
name: "doc"
relation { name: "editor" }
relation { name: "viewer" }
relation { name: "owner" }
```
- doc에 대한 권한 정의
- doc에는 editor, viewer, owner라는 권한이 있음

### Editors -> Viewer
- 권한 상속이 가능함
- owner -> editor -> viewr
```bash
name: "doc"
relation { name: "owner" }
relation {
  name: "editor"
  userset_rewrite {
    # union, intersection, exclusion등의 집합 연산 사용 가능
    union {
      # computed_userset이 없는 _this: owner가 아닌 editor 자체를 의미
      child { _this {} }
      # 동일한 개체의 relation간 상속 정의 가능
      # 개체의 owner가 editor와 연관 있다는 의미
      child { computed_userset { relation: "owner" } }
}}}
relation {
 name: "viewer"
 userset_rewrite {
  union {
    child { _this {} }
    child { computed_userset { relation: "editor" } }
}}}
```

### Group
```bash
# namespace:object_id#relation
## tuple의 사용자로 정의하는 문법
## e.g. org:contoso#member; org:contoso와 멤버로 관련된 사용자 집합

### Organization
name: "org"
relation { name: "member" }

### Document
name: "doc"
relation { name: "owner" }
relation {
  name: "editor"
  userset_rewrite {
    union {
      child { _this {} }
      child { computed_userset { relation: "owner" } }
}}}
relation {
  name: "viewer"
  userset_rewrite {
  union {
    child { _this {} }
    child { computed_userset { relation: "editor" } }
}}}
```

### Folders
```bash
# doc의 parent가 될 folder 정의
name: "folder"
relation { name: "parent" }
relation { name: "owner" }
relation {
  name: "editor"
  userset_rewrite {
    union {
      child { _this {} }
      child { computed_userset { relation: "owner" } }
}}}
relation {
  name: "viewer"
  userset_rewrite {
  union {
    child { _this {} }
    child { computed_userset { relation: "editor" } }
}}}

# doc의 parent(상위 관계)를 지정
name: "doc"
relation { name: "parent" }
relation { name: "owner" }
relation {
  name: "editor"
  userset_rewrite {
    union {
      child { _this {} }
      child { computed_userset { relation: "owner" } }
      child { tuple_to_userset { 
        tupleset { relation: "parent" }
        computed_userset {
          object: $TUPLE_USERSET_OBJECT  # parent folder
          relation: "editor" }}}
}}}
relation {
  name: "viewer"
  # doc의 parent:viewer 관계가 있는 모든 user 집합이 문서 자체와도 연관이 있다는 의미
  userset_rewrite {
  union {
    child { _this {} }
    child { computed_userset { relation: "editor" } }
    child { tuple_to_userset {
      tupleset { relation: "parent" }
      computed_userset {
        object: $TUPLE_USERSET_OBJECT  # parent folder
        relation: "viewer" }}}
}}}

name: "org"
relation { name: "member" }
```
