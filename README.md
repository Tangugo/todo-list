## 项目介绍
rust 实现的 todo list 项目，主要实现的功能就是增、删、查、改。

### 项目主要文件介绍
```
$ tree src
src
├── dao.rs      # 数据库操作
├── main.rs     # 项目入口
├── model.rs    # db模型
├── route.rs    # 接口路由
└── services.rs # 接口服务
```

### 项目运行
```bash
cargo run
```

## 性能压测benchmark
压测机器性能：
- 处理器：苹果M1 Pro
- 内核：10核
- 内存：16GB

性能设置：
1. postgres数据库最大连接数由默认的100改为200，程序中数据库连接池大小也设为200
2. 程序工作线程数设置为8（10核cpu）
3. ulimit -n 65535 （最大打开文件数）

利用k6压测工具对todo-list服务增加数据接口进行压测，压测的脚本`benchmark/c10k_test.js`，压测脚本要求响应平均时间少于300ms，错误率低于2%，压测结果如下：
```bash

         /\      Grafana   /‾‾/
    /\  /  \     |\  __   /  /
   /  \/    \    | |/ /  /   ‾‾\
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/

     execution: local
        script: benchmark/c10k_test.js
        output: -

     scenarios: (100.00%) 1 scenario, 10000 max VUs, 1m40s max duration (incl. graceful stop):
              * default: Up to 10000 looping VUs for 1m10s over 4 stages (gracefulRampDown: 30s, gracefulStop: 30s)

# 有210条这个错误的信息，都是由于数据库连接池数被用完，导致请求失败
ERRO[0066] Request failed with status 500: Failed to acquire connection from pool  source=console

     ✗ is status 200
      ↳  99% — ✓ 258935 / ✗ 210

     checks.........................: 99.91% 258935 out of 259145
     data_received..................: 62 MB  641 kB/s
     data_sent......................: 51 MB  521 kB/s
     http_req_blocked...............: avg=15.32µs min=0s    med=1µs     max=28.93ms p(90)=4µs      p(95)=10µs
     http_req_connecting............: avg=12.19µs min=0s    med=0s      max=28.91ms p(90)=0s       p(95)=0s
   ✓ http_req_duration..............: avg=97.72ms min=233µs med=27.38ms max=30.01s  p(90)=221.88ms p(95)=268.96ms
       { expected_response:true }...: avg=73.46ms min=233µs med=27.22ms max=445.3ms p(90)=221.55ms p(95)=268.11ms
   ✓ http_req_failed................: 0.08%  210 out of 259145
     http_req_receiving.............: avg=14.21µs min=3µs   med=10µs    max=2.02ms  p(90)=27µs     p(95)=40µs
     http_req_sending...............: avg=9.49µs  min=2µs   med=4µs     max=38.25ms p(90)=17µs     p(95)=26µs
     http_req_tls_handshaking.......: avg=0s      min=0s    med=0s      max=0s      p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=97.69ms min=220µs med=27.35ms max=30.01s  p(90)=221.86ms p(95)=268.94ms
     http_reqs......................: 259145 2673.276921/s
     iteration_duration.............: avg=1.09s   min=1s    med=1.02s   max=31.01s  p(90)=1.22s    p(95)=1.26s
     iterations.....................: 259145 2673.276921/s
     vus............................: 105    min=6                max=10000
     vus_max........................: 10000  min=10000            max=10000


running (1m36.9s), 00000/10000 VUs, 259145 complete and 0 interrupted iterations
default ✓ [======================================] 00000/10000 VUs  1m10s
```

### 压测结果分析
压测结果表明，todo-list服务的响应平均时间小于300ms，错误率低于2%，压测结果符合预期，从压测输出的错误日志来看，错误都是因为数据库连接池耗尽从而导致的，rust开发的todo-list是满足c10k。
#### 请求成功率
- 总请求数：259,145 次。
- 成功请求：258,935 次（99.91%）。
- 失败请求：210 次（0.08%）。

#### 响应时间
- 平均响应时间：97.72ms。
- 中位数响应时间：27.38ms。
- 95% 响应时间：268.96ms。
- 最大响应时间：30.01 秒。

#### 吞吐量
- 每秒大约2673

## 如何提高rust web服务高性能的一些思考
### 系统层面
1. 合理设置系统的最大文件描述符数

### 缓存层面
1. 合理利用redis缓存热点数据，减轻程序和db层的压力

### 数据库层面优化
1. 合理设置数据库最大连接数
2. 根据项目数据量情况，进行分库分表
3. 搭建主从数据库，实现读写分离，提高数据库抗压和性能
4. 合理设置数据库索引，提高查询效率（通过慢查询日志或者执行计划explain定位慢sql）
5. 对于数据库的一些插入或者更新操作，可以考虑批量操作

### 程序层面
1. 对于一些比较耗时的操作，可以考虑异步处理
2. 尽量使用一些无锁的数据结构，避免mutex锁所带来的性能瓶颈
3. 设置适当的数据库连接池数
4. 根据部署环境资源情况，合理设置工作线程数
5. 加入限流熔断机制，保证我们服务的稳健性
6. 增加trace id和日志，帮助我们定位排查问题以及调优
7. 引入Prometheus 和 Grafana，配置 Prometheus 收集性能指标，并使用 Grafana 可视化监控数据

### 服务部署层面
1. 条件允许情况下，可以增加服务器资源（cpu、内存、带宽、ssd磁盘等）
2. 分布式部署，多个后端提供服务能力，通过nginx或者haproxy实现负载均衡
