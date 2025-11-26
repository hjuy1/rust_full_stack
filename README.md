# Rust Web 全栈开发教程【完结】（非原创，代码来自杨旭老师）

## 内容说明

这是 B 站 BV1RP4y1G7KF 这个视频 7-16 节用 axum 代替 actix 的代码

原视频地址：[Rust Web 全栈开发教程【完结】](https://www.bilibili.com/video/BV1RP4y1G7KF/)

## 技术栈

- **语言**: Rust 2024 edition
- **Web 框架**: Axum
- **异步运行时**: Tokio
- **HTTP 服务**: Tower and Tower-http
- **序列化**: Serde
- **数据库**: SQLx with PostgreSQL
- **时间处理**: Chrono

## 项目结构

```
web_service/
├── src/
│   ├── bin/                 # 可执行文件
│   │   └── teacher_service.rs
│   ├── db_access/           # 数据库访问层
│   │   └── course.rs
│   ├── handlers/            # 请求处理函数
│   │   ├── course.rs
│   │   └── general.rs
│   ├── models/              # 数据模型
│   │   └── course.rs
│   ├── error.rs             # 错误处理
│   ├── main.rs              # 主程序入口
│   ├── routers.rs           # 路由配置
│   └── state.rs             # 应用状态
├── static/                  # 静态文件
│   ├── 404.css
│   └── 404.html
├── Cargo.toml               # 包配置文件
└── Cargo.lock

根目录还包含数据库初始化脚本和其他辅助文件。
```

## 功能特性

1. **模块化架构**

   - 清晰分离路由、处理函数、数据模型和数据库访问逻辑
   - 易于维护和扩展的代码组织方式

2. **RESTful API 设计**

   - 提供课程相关操作的完整 CRUD 接口
   - 支持教师和课程的增删改查操作

3. **数据库集成**

   - 使用 SQLx 连接 PostgreSQL 数据库
   - 实现了完整的数据库访问层

4. **错误处理**

   - 统一的错误处理机制
   - 自定义错误类型和响应格式

5. **静态文件服务**
   - 内置静态文件服务支持
   - 自定义 404 页面处理

## API 接口

### 健康检查

- `GET /health` - 服务健康状态检查

### 课程管理

- `POST /course/` - 创建新课程
- `GET /course/{teacher_id}` - 获取指定教师的所有课程
- `GET /course/{teacher_id}/{course_id}` - 获取特定课程详情
- `PUT /course/{teacher_id}/{course_id}` - 更新课程信息
- `DELETE /course/{teacher_id}/{course_id}` - 删除课程

## 快速开始

### 环境要求

- Rust 1.75 或更高版本
- PostgreSQL 数据库

### 安装步骤

1. 克隆项目到本地：

```bash
git clone <repository-url>
cd rust_full_stack
```

2. 设置环境变量：
   在项目根目录创建 `.env` 文件，并设置数据库连接：

```env
DATABASE_URL=postgresql://username:password@localhost/database_name
```

3. 运行数据库迁移：
   根据 [db.sql](db.sql) 和 [db_new.sql](db_new.sql) 文件创建必要的表结构。

4. 启动服务：

```bash
cargo run --package web_service --bin teacher_service
```

服务将在 `http://127.0.0.1:8080` 上运行。

## 测试

项目包含针对各个处理函数的单元测试，可以通过以下命令运行：

```bash
cargo test
```

## 许可证

此项目仅供学习和参考使用。
