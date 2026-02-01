use clap::{Parser, Subcommand};

mod config;
mod auth;
mod api;
mod commands;
mod output;
mod error;
mod utils;

use config::Config;
use auth::AuthManager;
use api::ApiClient;
use output::OutputFormat;
use error::Result;

#[derive(Parser)]
#[command(name = "lark-cli")]
#[command(about = "Lark API 命令行工具", long_about = None)]
#[command(version)]
struct Cli {
    /// 详细输出模式
    #[arg(short, long)]
    verbose: bool,

    /// 输出格式 (text 或 json)
    /// 示例: text
    ///
    /// 可选值:
    /// - json: 结构化JSON格式（默认）
    /// - text: 人类可读的文本格式
    ///
    /// 使用建议:
    /// - json格式适合程序处理，字段清晰
    /// - text格式适合终端查看，简洁易读
    /// - 使用 -v 或 --verbose 可以显示更多调试信息
    #[arg(long, default_value = "json")]
    format: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "获取知识空间节点信息",
        long_about = "获取知识空间节点信息\n\n示例:\n  lark get-node doccnz1abcdefg123456789\n  lark get-node doccnz1abcdefg123456789 --obj-type doc\n\n字段说明:\n  - token: 知识空间节点token，从文档URL中获取\n    例如: https://bytedance.larkoffice.com/docx/doccnz1abcdefg123456789\n    其中 doccnz1abcdefg123456789 就是token\n  - obj_type: 节点类型，可选值: doc, sheet, bitable, file, folder, wiki\n    如果不指定，API会自动识别类型\n\n返回结构体字段说明:\n  - title: 节点标题\n  - node_token: 节点唯一标识符\n  - obj_token: 关联对象token\n  - obj_type: 对象类型（doc、sheet、bitable等）\n  - node_type: 节点类型\n  - has_child: 是否有子节点\n  - creator: 创建者用户ID\n  - owner: 所有者用户ID\n  - space_id: 知识空间ID\n  - parent_node_token: 父节点token\n  - node_create_time: 节点创建时间（ISO格式）\n  - obj_create_time: 对象创建时间（ISO格式）\n  - obj_edit_time: 对象最后编辑时间（ISO格式）"
    )]
    GetNode {
        /// 知识空间节点 token
        token: String,
        /// 知识空间节点类型 (可选)
        /// 可选值: doc, sheet, bitable, file, folder, wiki
        #[arg(long)]
        obj_type: Option<String>,
    },
    #[command(
        about = "获取文档内容",
        long_about = "获取文档的原始内容\n\n示例:\n  lark get-content doccnz1abcdefg123456789\n  lark get-content doccnz1abcdefg123456789 --format text\n\n字段说明:\n  - document_id: 文档ID，从文档URL中获取\n    例如: https://bytedance.larkoffice.com/docx/doccnz1abcdefg123456789\n    其中 doccnz1abcdefg123456789 就是document_id\n\n输出说明:\n  - 返回文档的原始Markdown内容\n  - 包含所有的文本、表格、图片等信息\n  - 使用 --format json 可以获取结构化数据\n  - 使用 --format text 可以获取简洁的文本输出\n\n返回结构体字段说明:\n  - content: 文档的原始内容（Markdown格式）\n    包含文档中所有的文本、表格、图片引用等\n    图片以Markdown格式引用: ![image.png](image_token)\n    表格以Markdown表格格式返回\n    代码块保留原始格式和语言标识"
    )]
    GetContent {
        /// 文档 ID
        document_id: String,
    },
    #[command(
        about = "添加协作者权限",
        long_about = "为云文档添加协作者权限\n\n示例:\n  lark add-permission doccnz1abcdefg123456789 doc --member-type email --member-id user@example.com --perm view\n  lark add-permission doccnz1abcdefg123456789 sheet --member-type open_id --member-id ou_123456 --perm edit\n  lark add-permission doccnz1abcdefg123456789 wiki --member-type userid --member-id 123456 --perm full_access --notification\n\n参数说明:\n  - token: 云文档token\n  - doc_type: 云文档类型 (doc、sheet、file、wiki、bitable、docx、folder、mindnote、minutes、slides)\n  - member_type: 协作者ID类型 (email、openid、unionid、openchat、opendepartmentid、userid、groupid、wikispaceid)\n  - member_id: 协作者ID\n  - perm: 权限角色 (view、edit、full_access)\n  - perm_type: 权限角色类型 (container、single_page)，仅知识库文档有效\n  - collaborator_type: 协作者类型 (user、chat、department、group、wiki_space_member、wiki_space_viewer、wiki_space_editor)\n  - notification: 是否通知对方\n\n返回结构体字段说明:\n  - member_type: 成员类型\n  - member_id: 成员ID\n  - perm: 权限级别\n  - perm_type: 权限类型\n  - collaborator_type: 协作者类型"
    )]
    AddPermission {
        /// 云文档 token
        token: String,
        /// 云文档类型 (doc、sheet、file、wiki、bitable、docx、folder、mindnote、minutes、slides)
        #[arg(long)]
        doc_type: String,
        /// 协作者ID类型 (email、openid、unionid、openchat、opendepartmentid、userid、groupid、wikispaceid)
        #[arg(long)]
        member_type: String,
        /// 协作者ID
        #[arg(long)]
        member_id: String,
        /// 权限角色 (view、edit、full_access)
        #[arg(long)]
        perm: String,
        /// 权限角色类型 (container、single_page，仅知识库文档有效)
        #[arg(long, default_value = "container")]
        perm_type: Option<String>,
        /// 协作者类型 (user、chat、department、group、wiki_space_member、wiki_space_viewer、wiki_space_editor)
        #[arg(long, default_value = "user")]
        collaborator_type: Option<String>,
        /// 添加权限后是否通知对方
        #[arg(long)]
        notification: bool,
    },
    #[command(
        about = "创建云文档",
        long_about = "创建一个新的云文档\n\n示例:\n  lark create-document\n  lark create-document --title \"My Document\"\n  lark create-document --folder-token foldcnz1abcdefg123456789 --title \"Team Doc\"\n\n参数说明:\n  - folder_token: 可选，指定创建文档的文件夹token\n  - title: 可选，文档标题，1-800字符\n\n返回结构体字段说明:\n  - document_id: 文档唯一标识符\n  - revision_id: 文档版本号\n  - title: 文档标题\n  - url: 文档URL地址"
    )]
    CreateDocument {
        /// 文件夹 token (可选)
        #[arg(long)]
        folder_token: Option<String>,
        /// 文档标题 (可选，1-800字符)
        #[arg(long)]
        title: Option<String>,
    },
    #[command(
        about = "添加高亮块到文档",
        long_about = "在飞书文档中添加高亮块（提示框）\n\n示例:\n  lark add-callout doccnz1abcdefg123456789 \"This is important information\"\n  lark add-callout doccnz1abcdefg123456789 \"Warning message\" --callout-type warning\n  lark add-callout doccnz1abcdefg123456789 \"Success!\" --callout-type success --icon \"✅\"\n\n参数说明:\n  - document_id: 文档ID\n  - content: 高亮块内容\n  - parent_id: 父块ID，空字符串表示在根级别创建\n  - index: 插入位置索引，默认-1表示末尾\n  - callout_type: 高亮块类型（info、warning、error、success），默认info\n  - icon: 自定义图标（可选），如\"🔥\"，默认根据类型自动选择\n\n返回结构体字段说明:\n  - block_id: 创建的高亮块ID\n  - document_revision_id: 文档版本号"
    )]
    AddCallout {
        /// 文档 ID
        document_id: String,
        /// 高亮块内容
        content: String,
        /// 父块 ID（可选，空字符串表示在根级别创建）
        #[arg(long, default_value = "")]
        parent_id: Option<String>,
        /// 插入位置索引（默认: -1 表示末尾）
        #[arg(long, default_value = "-1")]
        index: Option<i32>,
        /// 高亮块类型（info、warning、error、success，默认: info）
        #[arg(long, default_value = "info")]
        callout_type: String,
        /// 自定义图标（可选，如 "🔥"，默认根据类型自动选择）
        #[arg(long)]
        icon: Option<String>,
    },
    #[command(
        about = "添加画板到文档",
        long_about = "在飞书文档中添加画板（绘图画布）\n\n示例:\n  lark add-board doccnz1abcdefg123456789\n  lark add-board doccnz1abcdefg123456789 --parent-id block_123 --index 0\n\n参数说明:\n  - document_id: 文档ID\n  - parent_id: 父块ID，空字符串表示在根级别创建\n  - index: 插入位置索引，默认-1表示末尾\n\n返回结构体字段说明:\n  - block_id_relations: 块ID映射关系数组\n    * block_id: 创建的块ID\n    * temporary_block_id: 临时块ID\n  - children: 子块信息数组\n    * block_id: 块ID\n    * block_type: 块类型\n    * board: 画板信息\n      - token: 画板唯一标识符（画板ID）\n      - align: 对齐方式\n    * parent_id: 父块ID\n  - client_token: 客户端令牌\n  - document_revision_id: 文档版本号"
    )]
    AddBoard {
        /// 文档 ID
        document_id: String,
        /// 父块 ID（可选，空字符串表示在根级别创建）
        #[arg(long, default_value = "")]
        parent_id: Option<String>,
        /// 插入位置索引（默认: -1 表示末尾）
        #[arg(long, default_value = "-1")]
        index: Option<i32>,
    },
    #[command(
        about = "下载画板为图片",
        long_about = "获取画板的缩略图片，下载为图片文件\n\n示例:\n  lark get-board-image Ru8nwrWFOhEmaFbEU2VbPRsHcxb ./board.png\n  lark get-board-image Ru8nwrWFOhEmaFbEU2VbPRsHcxb ./downloads/\n\n参数说明:\n  - whiteboard_id: 画板唯一标识符\n    * 可通过获取文档所有块接口获取\n    * block_type 为 43 的 block 即为画板\n    * block.token 就是画板的 whiteboard_id\n  - output_path: 输出文件路径或目录\n    * 如果指定为文件路径，则保存到该文件\n    * 如果指定为目录（以/或\\结尾），则自动使用 whiteboard_id 作为文件名\n\n返回结构体字段说明:\n  - file_path: 保存的文件路径\n  - file_size: 文件大小（字节）\n  - content_type: MIME类型（图片格式）\n    * image/png: PNG格式\n    * image/jpeg: JPEG格式\n    * image/gif: GIF格式\n    * image/svg+xml: SVG格式\n  - file_extension: 图片格式扩展名\n\n注意事项:\n  - 需要确保应用拥有画板的查看权限（board:whiteboard:node:read）\n  - 接口频率限制: 10次/秒\n  - HTTP状态码说明:\n    * 200: 下载成功\n    * 400: 参数错误（2890001/2890002/2890003）\n    * 401: 认证失败\n    * 403: 没有阅读权限\n    * 429: 请求频率超限\n    * 500: 服务端错误"
    )]
    GetBoardImage {
        /// 画板唯一标识符
        whiteboard_id: String,
        /// 输出文件路径或目录
        output_path: String,
    },
    #[command(
        about = "添加内容到文档（支持从文件、目录或直接内容添加）",
        long_about = "将内容添加到飞书文档中，支持多种导入方式\n\n示例:\n  # 从文件导入\n  lark add-content doccnz1abcdefg123456789 ./content.md\n\n  # 从目录批量导入\n  lark add-content doccnz1abcdefg123456789 ./docs --source-type dir --recursive\n\n  # 直接添加内容\n  lark add-content doccnz1abcdefg123456789 \"# 标题\\n\\n内容\" --source-type content\n\n  # 指定插入位置和父块\n  lark add-content doccnz1abcdefg123456789 ./content.md --block-id block_123 --index 0\n\n字段说明:\n  - document_id: 目标文档ID\n  - source: 导入源，根据source_type不同含义不同\n  - source_type: 源类型\n    * file: 单个文件路径\n    * dir: 目录路径\n    * content: 直接内容字符串\n  - content_type: 内容格式，支持markdown和html\n  - block_id: 父块ID，空字符串表示文档根级别\n  - index: 插入位置，-1表示末尾，0表示开头\n  - recursive: 是否递归处理子目录\n  - pattern: 文件匹配模式，如\"*.md\", \"*.txt\"\n  - batch_size: 并发数，建议3-5个\n  - skip_existing: 跳过已存在的文件\n\n返回结构体字段说明:\n  - block_id_relations: 块ID映射关系数组\n    * block_id: 实际创建的块ID\n    * temporary_block_id: 临时块ID（用于关联）\n  - document_revision_id: 文档版本号\n  - client_token: 客户端令牌（可选）\n  - children: 子块信息数组\n\n批量导入结果说明:\n  - success_count: 成功导入的文件数\n  - failure_count: 失败的文件数\n  - skipped_count: 跳过的文件数\n  - results: 详细结果列表\n    * file_path: 文件路径\n    * success: 是否成功\n    * error: 错误信息（如果失败）\n    * block_ids: 创建的块ID列表（如果成功）"
    )]
    AddContent {
        /// 文档 ID
        document_id: String,
        /// 导入源：文件路径、目录路径或直接内容
        source: String,
        /// 源类型：file（文件）、dir（目录）、content（内容）
        #[arg(long, default_value = "file")]
        source_type: String,
        /// 内容类型：markdown 或 html
        #[arg(long, default_value = "markdown")]
        content_type: String,
        /// 父块 ID（空字符串表示在根级别创建）
        #[arg(long, default_value = "")]
        block_id: String,
        /// 插入位置索引（默认: -1 表示末尾）
        #[arg(long, default_value = "-1")]
        index: i32,
        /// 是否递归处理子目录
        #[arg(long)]
        recursive: bool,
        /// 文件匹配模式（如 "*.md"）
        #[arg(long)]
        pattern: Option<String>,
        /// 批处理的并发数（默认: 3）
        #[arg(long, default_value = "3")]
        batch_size: usize,
        /// 跳过已存在的文件
        #[arg(long)]
        skip_existing: bool,
    },
    #[command(
        about = "获取文档的所有块内容",
        long_about = "获取飞书文档中的所有块内容，支持分页获取\n\n示例:\n  lark get-blocks doccnz1abcdefg123456789\n  lark get-blocks doccnz1abcdefg123456789 --page-size 100\n  lark get-blocks doccnz1abcdefg123456789 --all\n\n参数说明:\n  - document_id: 文档ID\n  - page_size: 分页大小，默认500，最大500\n  - page_token: 分页标记，用于获取下一页\n  - document_revision_id: 文档版本ID，默认-1表示最新版本\n  - user_id_type: 用户ID类型，默认open_id\n  - all: 自动获取所有块（处理分页）\n\n返回结构体字段说明:\n  - items: 块内容数组\n    * block_id: 块唯一标识符\n    * block_type: 块类型代码\n    * parent_id: 父块ID\n    * children: 子块ID数组（可选）\n    * content: 块内容（JSON格式）\n  - page_token: 分页标记（可选）\n  - has_more: 是否还有更多结果"
    )]
    GetBlocks {
        /// 文档 ID
        document_id: String,
        /// 分页大小（默认: 500，最大: 500）
        #[arg(long, default_value = "500")]
        page_size: i32,
        /// 分页标记（可选）
        #[arg(long)]
        page_token: Option<String>,
        /// 文档版本 ID（默认: -1 表示最新版本）
        #[arg(long)]
        document_revision_id: Option<i32>,
        /// 用户 ID 类型（默认: open_id）
        #[arg(long, default_value = "open_id")]
        user_id_type: String,
        /// 获取所有块（自动处理分页）
        #[arg(long)]
        all: bool,
    },
    #[command(
        about = "批量更新文档块",
        long_about = "批量更新飞书文档中的多个块内容\n\n示例:\n  lark batch-update-blocks doccnz1abcdefg123456789 '[{\"block_id\":\"block_123\",\"update_text\":{\"elements\":[{\"text_run\":{\"content\":\"新内容\"}}]}}]'\n\n参数说明:\n  - document_id: 文档ID\n  - requests: 更新请求列表（JSON格式）\n  - document_revision_id: 文档版本ID，默认-1表示最新版本\n  - client_token: 可选的UUIDv4，用于幂等更新\n  - user_id_type: 用户ID类型，默认open_id\n\n返回结构体字段说明:\n  - blocks: 更新后的块数组\n  - client_token: 客户端令牌（可选）\n  - document_revision_id: 文档版本号"
    )]
    BatchUpdateBlocks {
        /// 文档 ID
        document_id: String,
        /// 更新请求列表（JSON 格式）
        requests: String,
        /// 文档版本 ID（默认: -1 表示最新版本）
        #[arg(long)]
        document_revision_id: Option<i32>,
        /// 可选的 UUIDv4，用于幂等更新
        #[arg(long)]
        client_token: Option<String>,
        /// 用户 ID 类型（默认: open_id）
        #[arg(long, default_value = "open_id")]
        user_id_type: String,
    },
    #[command(
        about = "删除文档块",
        long_about = "删除飞书文档中的指定范围的块内容\n\n示例:\n  lark delete-blocks doccnz1abcdefg123456789 block_123 0 5\n  lark delete-blocks doccnz1abcdefg123456789 block_123 2 4 --document-revision-id 123\n\n参数说明:\n  - document_id: 文档ID\n  - block_id: 父块ID\n  - start_index: 删除起始索引（包含）\n  - end_index: 删除结束索引（不包含）\n  - document_revision_id: 文档版本ID，默认-1表示最新版本\n  - client_token: 可选的UUIDv4，用于幂等操作\n\n返回结构体字段说明:\n  - document_revision_id: 文档版本号\n  - client_token: 客户端令牌（可选）"
    )]
    DeleteBlocks {
        /// 文档 ID
        document_id: String,
        /// 父块 ID
        block_id: String,
        /// 删除起始索引（包含）
        start_index: i32,
        /// 删除结束索引（不包含）
        end_index: i32,
        /// 文档版本 ID（默认: -1 表示最新版本）
        #[arg(long)]
        document_revision_id: Option<i32>,
        /// 可选的 UUIDv4，用于幂等操作
        #[arg(long)]
        client_token: Option<String>,
    },
    #[command(
        about = "读取文件内容和大小",
        long_about = "读取指定文件的内容和大小信息\n\n示例:\n  lark read-file ./example.txt\n  lark read-file /path/to/document.pdf\n\n参数说明:\n  - file_path: 文件路径\n\n返回结构体字段说明:\n  - content: 文件内容（字节数组）\n  - size: 文件大小（字节）\n  - encoding: 文件编码类型"
    )]
    ReadFile {
        /// 文件路径
        file_path: String,
    },
    #[command(
        about = "写入文件内容",
        long_about = "将内容写入指定文件，支持Base64编码\n\n示例:\n  lark write-file ./output.txt \"SGVsbG8gV29ybGQ=\"\n  lark write-file ./data.bin \"AQIDBA==\" --overwrite\n\n参数说明:\n  - file_path: 文件路径\n  - content: 文件内容（Base64编码）\n  - overwrite: 是否覆盖已存在的文件\n\n返回结构体字段说明:\n  - success: 写入操作是否成功\n  - size: 写入的字节数\n  - message: 状态消息"
    )]
    WriteFile {
        /// 文件路径
        file_path: String,
        /// 文件内容（Base64 编码）
        content: String,
        /// 是否覆盖已存在的文件
        #[arg(long)]
        overwrite: bool,
    },
    #[command(
        about = "上传媒体文件",
        long_about = "上传媒体文件到飞书文档\n\n示例:\n  lark upload-media ./image.png docx_image doccnz1abcdefg123456789\n  lark upload-media ./document.pdf doc_file doccnz1abcdefg123456789 --checksum 12345678\n\n参数说明:\n  - file_path: 文件路径\n  - parent_type: 上传点类型 (doc_image/docx_image/sheet_image/doc_file/docx_file)\n  - parent_node: 上传点token（目标云文档token或block_id）\n  - checksum: Adler-32校验和（可选）\n  - extra: 额外信息，格式: {\"drive_route_token\":\"文档token\"}（可选）\n\n返回结构体字段说明:\n  - file_token: 上传文件的唯一标识符"
    )]
    UploadMedia {
        /// 文件路径
        file_path: String,
        /// 上传点类型 (doc_image/docx_image/sheet_image/doc_file/docx_file)
        parent_type: String,
        /// 上传点token（目标云文档token或block_id）
        parent_node: String,
        /// Adler-32校验和 (可选)
        #[arg(long)]
        checksum: Option<String>,
        /// 额外信息，格式: {"drive_route_token":"文档token"} (可选)
        #[arg(long)]
        extra: Option<String>,
    },
    #[command(
        about = "下载素材文件",
        long_about = "下载云文档中的素材文件，支持分片下载\n\n示例:\n  lark download-media boxcnrHpsg1QDqXAAAyachabcef ./downloaded_image.png\n  lark download-media boxcnrHpsg1QDqXAAAyachabcef ./downloads/\n  lark download-media boxcnrHpsg1QDqXAAAyachabcef ./file.png --range \"bytes=0-1024\"\n  lark download-media boxcnrHpsg1QDqXAAAyachabcef ./file.png --extra \"{\\\"drive_route_token\\\":\\\"doc_token\\\"}\"\n\n参数说明:\n  - file_token: 素材文件的token，可通过获取文档块、电子表格等接口获取\n  - output_path: 输出文件路径或目录\n    * 如果指定为文件路径，则保存到该文件\n    * 如果指定为目录（以/或\\结尾），则自动使用服务器返回的文件名\n  - extra: 额外扩展信息（可选），用于高级权限的多维表格鉴权\n    * 格式: JSON字符串，如 {\\\"drive_route_token\\\":\\\"文档token\\\"}\n  - range: 分片下载范围（可选），格式: bytes=start-end\n    * 示例: bytes=0-1024 表示下载前1024字节\n    * 支持 HTTP Range 请求标准\n\nfile_token 获取方式:\n  - 新版文档: 通过获取文档块接口获取图片块或文件块的token\n  - 电子表格: 通过读取范围接口获取附件的fileToken\n  - 多维表格: 通过查询记录接口获取附件的file_token\n\n返回结构体字段说明:\n  - file_path: 保存的文件路径\n  - file_size: 文件大小（字节）\n  - content_type: MIME类型\n  - file_name: 文件名（从服务器响应头获取，可选）\n\n注意事项:\n  - 需要确保应用拥有素材的下载权限\n  - 本接口仅支持下载云文档而非云空间中的资源文件\n  - 调用频率限制: 5 QPS，10000次/天\n  - HTTP状态码说明:\n    * 200: 下载成功\n    * 206: 部分内容下载成功（使用Range时）\n    * 400: 参数错误（高级权限多维表格需要extra参数）\n    * 403: 没有下载权限\n    * 404: 素材不存在或被删除\n    * 500: 服务端错误"
    )]
    DownloadMedia {
        /// 素材文件 token
        file_token: String,
        /// 输出文件路径或目录
        output_path: String,
        /// 额外扩展信息，格式: {"drive_route_token":"文档token"} (可选)
        #[arg(long)]
        extra: Option<String>,
        /// 分片下载范围，格式: bytes=start-end (可选)
        #[arg(long)]
        range: Option<String>,
    },
    #[command(
        about = "发送消息",
        long_about = "发送消息给指定接收者\n\n示例:\n  lark send-message ou_123456 open_id text '{\"text\":\"Hello World\"}'\n  lark send-message chat_123456 chat_id text '{\"text\":\"Group message\"}'\n  lark send-message user@example.com email text '{\"text\":\"Email message\"}' --uuid unique-id-123\n\n参数说明:\n  - receive_id: 消息接收者ID\n  - receive_id_type: 接收者ID类型 (open_id/union_id/user_id/email/chat_id)\n  - msg_type: 消息类型 (text/post/image/file/audio/media/sticker/interactive/share_chat/share_user/system)\n  - content: 消息内容（JSON格式字符串）\n  - uuid: 唯一标识符，用于幂等控制（可选）\n\n返回结构体字段说明:\n  - message_id: 消息唯一标识符\n  - root_id: 根消息ID（用于线程）\n  - parent_id: 父消息ID\n  - thread_id: 线程标识符\n  - msg_type: 消息类型\n  - create_time: 消息创建时间\n  - update_time: 最后更新时间\n  - deleted: 删除状态\n  - updated: 更新状态\n  - chat_id: 聊天标识符（可选）\n  - sender: 发送者信息\n    * id: 发送者ID\n    * id_type: ID类型\n    * sender_type: 发送者类型\n    * tenant_key: 租户标识符\n  - body: 消息内容\n    * content: 消息文本内容\n  - mentions: @提及列表（可选）\n    * key: 提及键\n    * id: 被提及实体ID\n    * id_type: ID类型\n    * name: 被提及实体名称\n    * tenant_key: 租户标识符\n  - upper_message_id: 上一条消息ID（可选）"
    )]
    SendMessage {
        /// 消息接收者 ID
        receive_id: String,
        /// 接收者 ID 类型 (open_id/union_id/user_id/email/chat_id)
        #[arg(long)]
        receive_id_type: String,
        /// 消息类型 (text/post/image/file/audio/media/sticker/interactive/share_chat/share_user/system)
        #[arg(long)]
        msg_type: String,
        /// 消息内容 (JSON 格式字符串)
        content: String,
        /// 唯一标识符，用于幂等控制 (可选)
        #[arg(long)]
        uuid: Option<String>,
    },
    #[command(
        about = "搜索群列表",
        long_about = "搜索飞书群聊列表\n\n示例:\n  lark search-chats\n  lark search-chats --query \"project\"\n  lark search-chats --query \"team\" --page-size 20\n\n参数说明:\n  - user_id_type: 用户ID类型，默认open_id\n  - query: 关键词搜索（可选）\n  - page_token: 分页标记（可选）\n  - page_size: 分页大小，1-100，默认50\n\n返回结构体字段说明:\n  - items: 群聊信息数组\n    * chat_id: 群聊唯一标识符\n    * avatar: 群头像URL（可选）\n    * name: 群名称\n    * description: 群描述（可选）\n    * owner_id: 群主ID（可选）\n    * owner_id_type: 群主ID类型（可选）\n    * external: 是否为外部群\n    * tenant_key: 租户标识符\n    * chat_status: 群状态\n  - page_token: 分页标记（可选）\n  - has_more: 是否还有更多结果"
    )]
    SearchChats {
        /// 用户 ID 类型 (open_id/union_id/user_id)
        #[arg(long, default_value = "open_id")]
        user_id_type: String,
        /// 关键词搜索
        #[arg(long)]
        query: Option<String>,
        /// 分页标记
        #[arg(long)]
        page_token: Option<String>,
        /// 分页大小 (1-100, 默认: 20)
        #[arg(long, default_value = "50")]
        page_size: i32,
    },
    #[command(
        about = "获取会话历史消息",
        long_about = "获取指定会话的历史消息记录\n\n示例:\n  lark get-message-history --container-id-type chat --container-id chat_123456\n  lark get-message-history --container-id-type thread --container-id thread_123 --start-time 1640995200 --end-time 1641081600\n\n参数说明:\n  - container_id_type: 容器类型 (chat/thread)\n  - container_id: 容器ID\n  - start_time: 起始时间（秒级时间戳，可选）\n  - end_time: 结束时间（秒级时间戳，可选）\n  - sort_type: 排序方式，默认ByCreateTimeDesc\n  - page_size: 分页大小，1-50，默认50\n  - page_token: 分页标记（可选）\n\n返回结构体字段说明:\n  - has_more: 是否还有更多消息\n  - page_token: 分页标记（可选）\n  - items: 消息数组\n    * message_id: 消息ID\n    * root_id: 根消息ID（可选）\n    * parent_id: 父消息ID（可选）\n    * thread_id: 线程ID（可选）\n    * msg_type: 消息类型\n    * create_time: 创建时间\n    * update_time: 更新时间\n    * deleted: 删除状态\n    * updated: 更新状态\n    * chat_id: 聊天ID（可选）\n    * sender: 发送者信息\n      - id: 发送者ID\n      - id_type: ID类型\n      - sender_type: 发送者类型\n      - tenant_key: 租户标识符\n    * body: 消息内容\n      - content: 消息文本内容\n    * mentions: @提及列表（可选）\n      - key: 提及键\n      - id: 被提及实体ID\n      - id_type: ID类型\n      - name: 被提及实体名称\n      - tenant_key: 租户标识符\n    * upper_message_id: 上一条消息ID（可选）"
    )]
    GetMessageHistory {
        /// 容器类型 (chat/thread)
        #[arg(long)]
        container_id_type: String,
        /// 容器 ID
        #[arg(long)]
        container_id: String,
        /// 起始时间 (秒级时间戳)
        #[arg(long)]
        start_time: Option<String>,
        /// 结束时间 (秒级时间戳)
        #[arg(long)]
        end_time: Option<String>,
        /// 排序方式 (ByCreateTimeAsc/ByCreateTimeDesc)
        #[arg(long, default_value = "ByCreateTimeDesc")]
        sort_type: String,
        /// 分页大小 (1-50, 默认: 20)
        #[arg(long, default_value = "50")]
        page_size: i32,
        /// 分页标记
        #[arg(long)]
        page_token: Option<String>,
    },
    #[command(
        about = "获取单个用户信息",
        long_about = "获取通讯录中某一用户的信息，包括用户 ID、名称、邮箱、手机号、状态以及所属部门等信息\n\n示例:\n  lark get-user-info ou_7dab8a3d3cdcc9da365777c7ad535d62\n  lark get-user-info ou_7dab8a3d3cdcc9da365777c7ad535d62 --user-id-type open_id\n  lark get-user-info 123456 --user-id-type user_id --department-id-type open_department_id\n\n参数说明:\n  - user_id: 用户ID，ID类型与user_id_type保持一致\n  - user_id_type: 用户ID类型 (open_id/union_id/user_id)，默认open_id\n    * open_id: 标识一个用户在某个应用中的身份\n    * union_id: 标识一个用户在某个应用开发商下的身份\n    * user_id: 标识一个用户在某个租户内的身份\n  - department_id_type: 部门ID类型 (department_id/open_department_id)，默认open_department_id\n\n返回结构体字段说明:\n  - union_id: 用户的union_id\n  - user_id: 用户的user_id\n  - open_id: 用户的open_id\n  - name: 用户名称\n  - en_name: 英文名\n  - nickname: 别名\n  - email: 邮箱\n  - mobile: 手机号\n  - mobile_visible: 手机号码是否可见\n  - gender: 性别 (0:保密, 1:男, 2:女, 3:其他)\n  - avatar: 用户头像信息\n  - status: 用户状态\n  - department_ids: 用户所属部门的ID列表\n  - leader_user_id: 用户的直接主管的用户ID\n  - city: 工作城市\n  - country: 国家或地区Code缩写\n  - work_station: 工位\n  - join_time: 入职时间（秒级时间戳）\n  - is_tenant_manager: 是否为租户超级管理员\n  - employee_no: 工号\n  - employee_type: 员工类型 (1:正式员工, 2:实习生, 3:外包, 4:劳务, 5:顾问)\n  - orders: 用户排序信息\n  - custom_attrs: 自定义字段\n  - enterprise_email: 企业邮箱\n  - job_title: 职务\n  - geo: 数据驻留地\n  - job_level_id: 职级ID\n  - job_family_id: 序列ID\n  - assign_info: 用户席位列表\n  - department_path: 部门路径列表（需要用户身份调用）\n  - dotted_line_leader_user_ids: 虚线上级的用户ID\n\n注意事项:\n  - 使用应用身份调用时，响应结果中不会返回部门路径字段（department_path）\n  - 如需获取部门路径字段，请使用用户身份（user_access_token）调用接口\n  - 需要获取通讯录基本信息权限"
    )]
    GetUserInfo {
        /// 用户 ID
        user_id: String,
        /// 用户 ID 类型 (open_id/union_id/user_id)
        #[arg(long)]
        user_id_type: Option<String>,
        /// 部门 ID 类型 (department_id/open_department_id)
        #[arg(long)]
        department_id_type: Option<String>,
    },
    #[command(
        about = "导入图表到画板",
        long_about = "将图表导入到飞书画板中，支持PlantUML和Mermaid语法\n\n示例:\n  lark import-diagram whiteboard_123 ./diagram.puml\n  lark import-diagram whiteboard_123 \"@startuml\nAlice -> Bob: Hello\n@enduml\" --source-type content\n  lark import-diagram whiteboard_123 ./flowchart.mmd --syntax mermaid --diagram-type flowchart\n\n参数说明:\n  - whiteboard_id: 画板ID\n  - source: 图表代码或文件路径\n  - source_type: 源类型，file（文件）或content（直接内容），默认file\n  - syntax: 图表语法类型，plantuml或mermaid，默认plantuml\n  - diagram_type: 图表类型，auto、mindmap、sequence、activity、class、er、flowchart、usecase、component，默认auto\n  - style: 样式类型，board（画板样式）或classic（经典样式），默认board\n\n返回结构体字段说明:\n  - node_id: 导入图表的节点ID"
    )]
    ImportDiagram {
        /// 画板ID
        whiteboard_id: String,
        /// 图表代码或文件路径
        source: String,
        /// 源类型：file（文件）或 content（直接内容）
        #[arg(long, default_value = "file")]
        source_type: String,
        /// 图表语法类型：plantuml 或 mermaid
        #[arg(long, default_value = "plantuml")]
        syntax: String,
        /// 图表类型：auto, mindmap, sequence, activity, class, er, flowchart, usecase, component
        #[arg(long, default_value = "auto")]
        diagram_type: String,
        /// 样式类型：board（画板样式）或 classic（经典样式）
        #[arg(long, default_value = "board")]
        style: String,
    },
    #[command(
        about = "创建画板节点",
        long_about = "在飞书画板中创建节点，支持批量创建、创建含父子关系的节点等\n\n示例:\n  lark create-board-notes whiteboard_123 '[{\"id\":\"o1:1\",\"type\":\"text_shape\",\"x\":100,\"y\":100,\"text\":{\"text\":\"Hello World\"}}]'\n  lark create-board-notes whiteboard_123 './nodes.json'\n  lark create-board-notes whiteboard_123 '[...]' --client-token uuid-123 --user-id-type open_id\n\n参数说明:\n  - whiteboard_id: 画板唯一标识符\n    * 可通过获取文档所有块接口获取\n    * block_type 为 43 的 block 即为画板\n    * block.token 就是画板的 whiteboard_id\n  - nodes_json: 节点数据的 JSON 字符串或 JSON 文件路径\n    * 如果是有效 JSON 格式，则直接解析为节点数据\n    * 如果是文件路径，则从文件中读取 JSON 数据\n  - client_token: 操作的唯一标识，用于幂等更新（可选）\n  - user_id_type: 用户 ID 类型（open_id/union_id/user_id），默认 open_id\n\n支持的节点类型:\n  - image: 图片\n  - text_shape: 文本\n  - group: 组合\n  - composite_shape: 基础图形（圆形、矩形、三角形等）\n  - svg: svg 图形\n  - connector: 连线\n  - table: 表格\n  - life_line: 对象生命线\n  - activation: 控制焦点\n  - section: 分区\n  - table_uml: 类图\n  - table_er: 实体关系图\n  - sticky_note: 便签\n  - mind_map: 思维导图\n  - paint: 画笔\n  - combined_fragment: 组合片段\n\n返回结构体字段说明:\n  - ids: 所创建的节点 id 列表\n  - client_token: 操作的唯一标识（可选）\n\n节点数据结构示例:\n  {\n    \"id\": \"o1:1\",\n    \"type\": \"text_shape\",\n    \"x\": 100,\n    \"y\": 100,\n    \"text\": {\n      \"text\": \"Hello World\",\n      \"font_size\": 14\n    }\n  }\n\n注意事项:\n  - 需要确保应用拥有画板的编辑权限（board:whiteboard:node:create）\n  - 接口频率限制: 50次/秒\n  - nodes 数组长度范围: 1-3000\n  - 父节点必须是已存在的节点或在本次创建的节点列表内\n  - HTTP状态码说明:\n    * 200: 创建成功\n    * 400: 参数错误（2890001/2890002/2890003）\n    * 401: 认证失败\n    * 403: 没有编辑权限\n    * 429: 请求频率超限\n    * 500: 服务端错误"
    )]
    CreateBoardNotes {
        /// 画板唯一标识符
        whiteboard_id: String,
        /// 节点数据的 JSON 字符串或 JSON 文件路径
        nodes_json: String,
        /// 操作的唯一标识，用于幂等更新（可选）
        #[arg(long)]
        client_token: Option<String>,
        /// 用户 ID 类型（open_id/union_id/user_id）
        #[arg(long, default_value = "open_id")]
        user_id_type: String,
    },
    #[command(
        about = "更新 lark-cli 到最新版本",
        long_about = "更新 lark-cli 到最新版本\n\n示例:\n  lark update\n  lark update --check\n  lark update --force\n\n参数说明:\n  - check: 仅检查是否有新版本，不执行更新\n  - force: 强制更新，即使当前已是最新版本\n\n更新流程:\n  1. 从 GitHub 获取最新版本信息\n  2. 比较当前版本与最新版本\n  3. 下载对应平台的二进制文件\n  4. 验证文件完整性（SHA256）\n  5. 备份当前版本并替换文件\n\n注意事项:\n  - 需要网络连接\n  - 需要文件写入权限\n  - 更新前会自动备份当前版本\n  - 支持 Linux/macOS/Windows 平台"
    )]
    Update {
        /// 仅检查更新，不执行下载和安装
        #[arg(long)]
        check: bool,
        /// 强制更新，即使当前已是最新版本
        #[arg(long)]
        force: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // 设置日志级别
    let log_level = if cli.verbose {
        "debug"
    } else {
        "error"
    };

    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .init();

    // 解析输出格式
    let output_format = match cli.format.as_str() {
        "json" => OutputFormat::Json,
        "text" => OutputFormat::Text,
        _ => {
            eprintln!("错误: 不支持的输出格式 '{}', 支持的格式: text, json", cli.format);
            std::process::exit(1);
        }
    };

    // 运行命令
    if let Err(e) = run_command(cli.command, output_format, cli.verbose).await {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

async fn run_command(command: Commands, output_format: OutputFormat, verbose: bool) -> Result<()> {
    // 加载配置
    let config = Config::load()?;

    // 创建认证管理器
    let auth_manager = AuthManager::new(config);

    // 创建 API 客户端
    let api_client = ApiClient::new(auth_manager);

    match command {
        Commands::GetNode { token, obj_type } => {
            commands::wiki::handle_get_node(api_client, token, obj_type, output_format).await
        }
        Commands::GetContent { document_id } => {
            commands::docx::handle_get_content(api_client, document_id, output_format).await
        }
        Commands::AddPermission {
            token,
            doc_type,
            member_type,
            member_id,
            perm,
            perm_type,
            collaborator_type,
            notification
        } => {
            commands::permission::handle_add_permission(
                api_client,
                token,
                doc_type,
                member_type,
                member_id,
                perm,
                perm_type,
                collaborator_type.unwrap_or_else(|| "user".to_string()),
                notification,
                output_format
            ).await
        }
        Commands::CreateDocument { folder_token, title } => {
            commands::document::handle_create_document(api_client, folder_token, title, output_format).await
        }
        Commands::AddCallout {
            document_id,
            content,
            parent_id,
            index,
            callout_type,
            icon
        } => {
            // 解析高亮块类型
            let callout_type = callout_type.parse()
                .map_err(|e: String| error::LarkError::ParseError(e))?;

            commands::add_callout::handle_add_callout(
                api_client,
                document_id,
                content,
                parent_id,
                index,
                callout_type,
                icon,
                output_format
            ).await
        }
        Commands::AddBoard {
            document_id,
            parent_id,
            index,
        } => {
            commands::add_board::handle_add_board(
                api_client,
                document_id,
                parent_id,
                index,
                output_format
            ).await
        }
        Commands::GetBoardImage {
            whiteboard_id,
            output_path,
        } => {
            commands::get_board_image::handle_get_board_image(
                api_client,
                whiteboard_id,
                output_path,
                output_format
            ).await
        }
        Commands::AddContent {
            document_id,
            source,
            source_type,
            content_type,
            block_id,
            index,
            recursive,
            pattern,
            batch_size,
            skip_existing,
        } => {
            commands::import::handle_add_content(
                api_client,
                document_id,
                source,
                source_type,
                content_type,
                block_id,
                index,
                recursive,
                pattern,
                batch_size,
                skip_existing,
                verbose,
                output_format
            ).await
        }
        Commands::GetBlocks {
            document_id,
            page_size,
            page_token,
            document_revision_id,
            user_id_type,
            all
        } => {
            commands::blocks::handle_get_blocks(
                api_client,
                document_id,
                page_size,
                page_token,
                document_revision_id,
                user_id_type,
                all,
                output_format
            ).await
        }
        Commands::BatchUpdateBlocks {
            document_id,
            requests,
            document_revision_id,
            client_token,
            user_id_type
        } => {
            commands::blocks::handle_batch_update_blocks(
                api_client,
                document_id,
                requests,
                document_revision_id,
                client_token,
                user_id_type,
                output_format
            ).await
        }
        Commands::DeleteBlocks {
            document_id,
            block_id,
            start_index,
            end_index,
            document_revision_id,
            client_token
        } => {
            commands::blocks::handle_delete_blocks(
                api_client,
                document_id,
                block_id,
                start_index,
                end_index,
                document_revision_id,
                client_token,
                output_format
            ).await
        }
        Commands::ReadFile { file_path } => {
            commands::file::handle_read_file(api_client, file_path, output_format).await
        }
        Commands::WriteFile { file_path, content, overwrite } => {
            commands::file::handle_write_file(api_client, file_path, content, overwrite, output_format).await
        }
        Commands::UploadMedia { file_path, parent_type, parent_node, checksum, extra } => {
            commands::file::handle_upload_media(
                api_client,
                file_path,
                parent_type,
                parent_node,
                checksum,
                extra,
                output_format
            ).await
        }
        Commands::DownloadMedia { file_token, output_path, extra, range } => {
            commands::download_media::handle_download_media(
                api_client,
                file_token,
                output_path,
                extra,
                range,
                output_format
            ).await
        }
        Commands::SendMessage { receive_id, receive_id_type, msg_type, content, uuid } => {
            commands::message::handle_send_message(
                api_client,
                receive_id,
                receive_id_type,
                msg_type,
                content,
                uuid,
                output_format
            ).await
        }
        Commands::SearchChats { user_id_type, query, page_token, page_size } => {
            commands::message::handle_search_chats(
                api_client,
                user_id_type,
                query,
                page_token,
                page_size,
                output_format
            ).await
        }
        Commands::GetMessageHistory { container_id_type, container_id, start_time, end_time, sort_type, page_size, page_token } => {
            commands::message::handle_get_message_history(
                api_client,
                container_id_type,
                container_id,
                start_time,
                end_time,
                sort_type,
                page_size,
                page_token,
                output_format
            ).await
        }
        Commands::GetUserInfo { user_id, user_id_type, department_id_type } => {
            commands::get_user_info::handle_get_user_info(
                api_client,
                user_id,
                user_id_type,
                department_id_type,
                output_format
            ).await
        }
        Commands::ImportDiagram {
            whiteboard_id,
            source,
            source_type,
            syntax,
            diagram_type,
            style
        } => {
            // 解析图表语法类型
            let syntax = syntax.parse()
                .map_err(|e: String| error::LarkError::ParseError(e))?;

            // 解析图表类型
            let diagram_type = diagram_type.parse()
                .map_err(|e: String| error::LarkError::ParseError(e))?;

            // 解析样式类型
            let style = style.parse()
                .map_err(|e: String| error::LarkError::ParseError(e))?;

            commands::import_diagram::handle_import_diagram(
                api_client,
                whiteboard_id,
                source,
                source_type,
                syntax,
                diagram_type,
                style,
                output_format
            ).await
        }
        Commands::CreateBoardNotes {
            whiteboard_id,
            nodes_json,
            client_token,
            user_id_type
        } => {
            commands::create_board_notes::handle_create_board_notes(
                api_client,
                whiteboard_id,
                nodes_json,
                client_token,
                Some(user_id_type),
                output_format
            ).await
        }
        Commands::Update { check, force } => {
            commands::update::update_command(check, force).await
        }
    }
}

