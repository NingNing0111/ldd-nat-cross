# 项目介绍

&emsp;ldd-nat-cross是一款基于SpringBoot3.x+Netty+Protobuf实现的内网穿透工具，支持多个客户端接入同一个服务端。

### 核心功能(开发中)

- [x] TCP穿透
- [ ] UDP穿透
- [ ] C/S 端监控
- [ ] admin端的操作控制

### 配置文件参考

&emsp;C/S端都是基于SpringBoot实现的，因此配置文件格式与SpringBoot配置文件无异。

#### 服务端:

```yaml
server:
  port: 8964 
  password: 123456
```

- `server.port`: 必填。服务端对外暴露的端口，供客户端接入.
- `server.password`: 必填。密码，客户端接入时认证

#### 客户端:

```yaml
client:
  serverHost: localhost
  serverPort: 8964
  password: 123456
  proxies:
    - host: localhost
      port: 4456
      protocol: tcp
      openPort: 8890
    - host: localhost
      port: 9011
      protocol: tcp
      openPort: 8891
    - host: localhost
      port: 3306
      protocol: tcp
      openPort: 3361
```

- `client.serverHost`: 必填。服务端主机名称；
- `client.serverPort`: 必填。服务端端口；
- `client.password`: 必填。接入服务端时需要的密码，用于接入认证；
- `client.proxies`: 必填。内网代理配置，支持多选；
  - host: 必填。内网主机（应用程序）
  - port: 必填。端口
  - openPort: 必填。服务端暴露的端口



### 启动

&emsp;与启动SpringBoot项目无异。可以在jar包所在位置创建一个config目录，config目录下放配置文件.

#### 服务端

```shell
java -jar server.jar
```

#### 客户端

```shell
java -jar client.jar
```