# Lark CLI 使用说明书

本目录包含 Lark CLI 所有命令的详细使用说明。

## 快速开始

### 基本命令结构
```bash
lark-cli [全局选项] <命令> [命令参数]
```

### 全局选项
- `-v, --verbose`: 启用详细日志输出
- `--format <FORMAT>`: 设置输出格式（text 或 json，默认: json）

## 命令分类

### 📚 知识库相关
- [get-node](get-node.md) - 获取知识空间节点信息

### 📄 文档相关
- [get-content](get-content.md) - 获取文档内容
- [create-document](create-document.md) - 创建新文档
- [add-content](add-content.md) - 添加内容到文档

### 🔓 权限管理
- [add-permission](add-permission.md) - 添加协作者权限

### 🧱 块操作
- [blocks-commands](blocks-commands.md) - 所有块相关命令
  - create-nested-blocks - 创建嵌套块
  - get-blocks - 获取文档块
  - batch-update-blocks - 批量更新块
  - delete-blocks - 删除块

### 📁 文件操作
- [file-commands](file-commands.md) - 所有文件相关命令
  - read-file - 读取文件
  - write-file - 写入文件
  - upload-media - 上传媒体文件

### 💬 消息功能
- [message-commands](message-commands.md) - 所有消息相关命令
  - send-message - 发送消息
  - search-chats - 搜索群聊
  - get-message-history - 获取消息历史

## 环境配置

在使用任何命令前，需要配置环境变量：

1. 创建 `.env` 文件（与可执行文件同目录）
2. 添加以下内容：
```
APP_ID=your_app_id
APP_SECRET=your_app_secret
```

## 输出格式

所有命令支持两种输出格式：

### JSON 格式（默认）
```json
{
  "code": 0,
  "msg": "success",
  "data": {
    // 具体数据
  }
}
```

### Text 格式
使用 `--format text` 查看调试信息。

## 错误处理

所有命令在出错时会返回非零退出码，并在 stderr 输出错误信息。

## 示例工作流

### 1. 创建文档并添加内容
```bash
# 创建新文档
DOC_ID=$(lark-cli create-document --title "项目文档" --format json | jq -r '.document_id')

# 添加 Markdown 文件
lark-cli add-content $DOC_ID ./project-docs --source-type dir --recursive
```

### 2. 设置权限并分享
```bash
# 添加协作者
lark-cli add-permission $DOC_ID \
  --doc-type docx \
  --member-type email \
  --member-id teammate@example.com \
  --perm edit \
  --notification
```

### 3. 获取文档结构
```bash
# 获取文档块结构
lark-cli get-blocks $DOC_ID --all
```

## 注意事项

1. **权限要求**：确保有相应的文档访问和编辑权限
2. **API 限制**：注意飞书 API 的调用频率限制
3. **文件大小**：上传文件时注意大小限制
4. **网络稳定性**：批量操作时确保网络稳定

## 获取帮助

如需更多帮助，可以使用：
```bash
lark-cli --help
lark-cli <命令> --help
```