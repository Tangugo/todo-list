import http from 'k6/http';
import { check, sleep } from 'k6';

// 配置测试参数
export let options = {
    stages: [
        { duration: '10s', target: 100 },   // 第一阶段：逐渐增加到 100 用户
        { duration: '20s', target: 1000 },  // 第二阶段：逐渐增加到 1000 用户
        { duration: '30s', target: 10000 }, // 第三阶段：逐渐增加到 1 万用户
        { duration: '10s', target: 10000 }, // 持续 10 秒保持 1 万用户
    ],
    thresholds: {
        http_req_duration: ['p(95)<300'], // 95% 请求的响应时间应小于 300ms
        http_req_failed: ['rate<0.02'],  // 错误率应低于 2%
    },
};

// 测试函数
export default function () {
    const payload = JSON.stringify({
        title: "Test Todo",
        description: "This is a test todo",
    });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    // 发送 POST 请求
    const res = http.post('http://localhost:8080/todos', payload, params);

    // 检查响应状态码
    if (res.status !== 200) {
        console.error(`Request failed with status ${res.status}: ${res.body}`);
    }
    check(res, {
        'is status 200': (r) => r.status === 200,
    });

    // 模拟用户思考时间
    sleep(1);
}