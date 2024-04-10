### Config.properties
- Trino 서버 구성 정보
```yaml
# coordinator configuration
coordinator=true 
node-scheduler.include-coordinator=false 
http-server.http.port=9999
query.max-memory=4GB #(Worker)
query.max-memory-per-node=1GB 
discovery.uri=http://{cluster_node}:9999 # coodinator

# worker configuration
coordinator=False
http-server.http.port=9999
query.max-memory=4GB #(Worker)
query.max-memory-per-node=1GB 
discovery.uri=http://{cluster_node}:9999 # coodinator
```

### node.properties
- node의 설정, node ip, env 등을 가짐
```yaml
node.environment=production # trino web Ui header tag node
node.id={cluster_node}
node.data-dir=/trino/data
```

### jvm.config
```yaml
-server
-Xmx8G # maximum heap space Trino
-XX:-UseBiasedLocking
-XX:+UseG1GC
-XX:G1HeapRegionSize=32M
-XX:+ExplicitGCInvokesConcurrent
-XX:+ExitOnOutOfMemoryError
-XX:+HeapDumpOnOutOfMemoryError
-XX:ReservedCodeCacheSize=512M
-XX:PerMethodRecompilationCutoff=10000
-XX:PerBytecodeRecompilationCutoff=10000
-Djdk.attach.allowAttachSelf=true
-Djdk.nio.maxCachedBufferSize=2000000
```

### catalog.properties
- Connector 구성
- Catalog에 탑재된 Connector를 통해 데이터 접근 가능
- e.g. `catalog/hive.properties` -> hive에 접근 가능
- `catalog` 디렉터리내 catalog 속성 파일을 생성하여 등록
  - e.g. `/catalog/jmx.properties` 마운트시, `connector.name=jmx`로 작성
```yml
# hive catalog
connector.name=hive
hive.metastore.uri=thrift://{hivemetastore uri} # Hive Thrift uri
```
