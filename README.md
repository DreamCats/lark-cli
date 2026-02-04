# Lark CLI

A powerful command-line interface for interacting with the Lark (Feishu) Open Platform API, built with Rust for performance and reliability.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

Lark CLI provides a comprehensive set of commands for managing Lark documents, wikis, permissions, messaging, and more. It features hierarchical command structure with convenient aliases, supports both text and JSON output formats, and offers detailed logging for debugging.

## Features

- **Document Operations** - Create, read, and modify Lark documents with full block-level control
- **Wiki Management** - Access and manage knowledge space nodes
- **Permission Control** - Add collaborators with granular permission settings
- **Messaging** - Send messages (text, rich text, images, files, cards) to users and groups
- **Media Handling** - Upload and download media files, board images, and assets
- **Whiteboard Support** - Create board nodes, import diagrams (PlantUML/Mermaid), and export as images
- **Flexible Output** - Text and JSON output formats with verbose logging option

## Installation

### Quick Install (Recommended)

**macOS / Linux:**

```bash
curl -fsSL https://raw.githubusercontent.com/DreamCats/lark-cli/main/install.sh | bash
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/DreamCats/lark-cli/main/install.ps1 | iex
```

### Download Binary

Download the latest release for your platform from [GitHub Releases](https://github.com/DreamCats/lark-cli/releases).

| Platform | Download |
|----------|----------|
| macOS (Apple Silicon) | `lark-cli-aarch64-apple-darwin.tar.gz` |
| macOS (Intel) | `lark-cli-x86_64-apple-darwin.tar.gz` |
| Linux (x64) | `lark-cli-x86_64-unknown-linux-gnu.tar.gz` |
| Linux (musl) | `lark-cli-x86_64-unknown-linux-musl.tar.gz` |
| Windows (x64) | `lark-cli-x86_64-pc-windows-msvc.zip` |

### Build from Source

Prerequisites: Rust 1.70 or later

```bash
git clone https://github.com/DreamCats/lark-cli.git
cd lark-cli
cargo build --release
```

The binary will be available at `target/release/lark-cli`.

## Configuration

Lark CLI loads configuration from `.env` files in the following order of precedence:

1. **Executable Directory** - `{executable_path}/.env`
2. **User Config Directory** - `~/.config/lark-cli/.env`

### Setup

```bash
# Option 1: User-level configuration (recommended)
mkdir -p ~/.config/lark-cli
cat > ~/.config/lark-cli/.env << EOF
APP_ID=your_app_id_here
APP_SECRET=your_app_secret_here
EOF

# Option 2: Executable directory
cp .env.template .env
# Edit .env with your credentials
```

### Obtaining Credentials

1. Visit the [Lark Open Platform](https://open.larkoffice.com/)
2. Create an application
3. Copy APP_ID and APP_SECRET from the application details page

## Usage

### Command Structure

```bash
lark-cli [OPTIONS] <COMMAND> [ARGS]
```

### Global Options

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Enable verbose logging |
| `--format <FORMAT>` | Output format: `text` (default) or `json` |
| `-h, --help` | Display help information |
| `-V, --version` | Display version information |

## Commands

### Wiki Operations

#### Get Node Information

Retrieve metadata for a knowledge space node.

```bash
lark-cli get-node <node_token> [--obj-type <type>]
```

### Document Operations

#### Get Document Content

Fetch raw text content from a document.

```bash
lark-cli get-content <document_id>
```

#### Create Document

Create a new Lark document.

```bash
lark-cli create-document [--folder-token <token>] [--title <title>]
```

#### Add Content

Import content into a document from various sources.

```bash
lark-cli add-content <document_id> <source> [OPTIONS]
```

**Options:**
- `--source-type` - Source type: `file`, `dir`, or `content`
- `--content-type` - Content type: `markdown` or `html`
- `--block-id` - Parent block ID (empty for root level)
- `--index` - Insertion index (-1 for end)
- `--recursive` - Process subdirectories recursively
- `--pattern` - File matching pattern (e.g., `*.md`)
- `--batch-size` - Concurrent batch size (default: 3)
- `--skip-existing` - Skip existing files

**Examples:**

```bash
# Import a Markdown file
lark-cli add-content doc_xxx ./document.md

# Import an entire directory recursively
lark-cli add-content doc_xxx ./docs --source-type dir --recursive

# Import inline content
lark-cli add-content doc_xxx "# Title\n\nContent" --source-type content
```

### Block Operations

#### Get All Blocks

Retrieve all blocks from a document.

```bash
lark-cli get-blocks <document_id> [OPTIONS]
```

**Options:**
- `--page-size` - Page size (default: 500, max: 500)
- `--page-token` - Pagination token
- `--all` - Fetch all blocks automatically

#### Create Nested Blocks

Create nested block structures within a document.

```bash
lark-cli create-nested-blocks <document_id> <children_ids> <descendants> [OPTIONS]
```

#### Batch Update Blocks

Update multiple blocks in a single request.

```bash
lark-cli batch-update-blocks <document_id> --requests <json>
```

#### Delete Blocks

Remove a range of child blocks.

```bash
lark-cli delete-blocks <document_id> <parent_block_id> <start_index> <end_index>
```

### Permission Management

#### Add Collaborator

Grant access permissions to users, groups, or departments.

```bash
lark-cli add-permission <token> \
  --doc-type <type> \
  --member-type <type> \
  --member-id <id> \
  --perm <permission> \
  [--notification]
```

**Document Types:** `doc`, `sheet`, `file`, `wiki`, `bitable`, `docx`, `folder`, `mindnote`, `minutes`, `slides`

**Member Types:** `email`, `openid`, `unionid`, `openchat`, `opendepartmentid`, `userid`, `groupid`, `wikispaceid`

**Permissions:** `view`, `edit`, `full_access`

### Messaging

#### Send Message

Send messages to users or group chats.

```bash
lark-cli send-message <receive_id> \
  --receive-id-type <type> \
  --msg-type <type> \
  <content>
```

**Receiver ID Types:** `open_id`, `union_id`, `user_id`, `email`, `chat_id`

**Message Types:** `text`, `post`, `image`, `file`, `audio`, `media`, `sticker`, `interactive`, `share_chat`, `share_user`

**Examples:**

```bash
# Send text message
lark-cli send-message ou_xxx --receive-id-type open_id --msg-type text \
  '{"text":"Hello, World!"}'

# Send rich text to group
lark-cli send-message oc_xxx --receive-id-type chat_id --msg-type post \
  '{"title":"Notice","content":[[{"tag":"text","text":"Important update"}]]}'

# Send interactive card
lark-cli send-message ou_xxx --receive-id-type open_id --msg-type interactive \
  '{"elements":[{"tag":"markdown","content":"**Reminder**: Check your tasks"}]}'
```

#### Search Chats

Search for group chats by keyword.

```bash
lark-cli search-chats [--query <keyword>] [--page-size <size>]
```

#### Get Message History

Retrieve historical messages from a chat.

```bash
lark-cli get-message-history \
  --container-id-type <type> \
  --container-id <id> \
  [--start-time <timestamp>] \
  [--end-time <timestamp>] \
  [--sort-type <order>]
```

### File Operations

#### Read File

Read file content and metadata.

```bash
lark-cli read-file <file_path>
```

#### Write File

Write Base64-encoded content to a file.

```bash
lark-cli write-file <file_path> <base64_content> [--overwrite]
```

#### Upload Media

Upload media files to documents or sheets.

```bash
lark-cli upload-media <file_path> <parent_type> <parent_node>
```

**Parent Types:** `doc_image`, `docx_image`, `sheet_image`, `doc_file`, `docx_file`

#### Download Media

Download asset files from Lark.

```bash
lark-cli download-media <file_token> <output_path> [--range <bytes>]
```

### Whiteboard Operations

#### Add Board

Insert a whiteboard block into a document.

```bash
lark-cli add-board <document_id> [--align <alignment>] [--width <px>] [--height <px>]
```

#### Import Diagram

Import PlantUML or Mermaid diagrams into a whiteboard.

```bash
lark-cli import-diagram <whiteboard_id> <source> \
  [--syntax <plantuml|mermaid>] \
  [--diagram-type <type>]
```

#### Get Board Image

Export a whiteboard as an image.

```bash
lark-cli get-board-image <whiteboard_id> <output_path>
```

#### Create Board Nodes

Create nodes on a whiteboard.

```bash
lark-cli create-board-notes <whiteboard_id> <nodes_json>
```

**Supported Node Types:** `text_shape`, `composite_shape`, `image`, `connector`, `table`, `group`, `svg`, `sticky_note`, `paint`, `section`, `life_line`, `activation`, `table_uml`, `table_er`, `mind_map`, `combined_fragment`

### Document Enhancement

#### Add Callout

Insert a highlighted callout block.

```bash
lark-cli add-callout <document_id> <content> \
  [--callout-type <info|warning|error|success>] \
  [--icon <emoji>]
```

## Output Examples

### JSON Format

```bash
lark-cli --format json get-node HyKOw87PHivqfIkIlzXcJu5An1f
```

```json
{
  "node_token": "HyKOw87PHivqfIkIlzXcJu5An1f",
  "obj_token": "E4Q5dudHKoHfJBxA7J0liAJEgmb",
  "obj_type": "docx",
  "title": "Document Title",
  "space_id": "7540300278621290524",
  "has_child": false
}
```

### Text Format

```bash
lark-cli get-node HyKOw87PHivqfIkIlzXcJu5An1f
```

```
KnowledgeSpaceNode {
    node_token: "HyKOw87PHivqfIkIlzXcJu5An1f",
    obj_token: "E4Q5dudHKoHfJBxA7J0liAJEgmb",
    obj_type: "docx",
    title: "Document Title",
    ...
}
```

## Project Structure

```
lark-cli/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config/              # Configuration management
│   ├── auth/                # Authentication handling
│   ├── api/                 # API client modules
│   │   ├── mod.rs           # Base API client
│   │   ├── wiki.rs          # Wiki API
│   │   ├── docx.rs          # Document API
│   │   ├── permission.rs    # Permission API
│   │   ├── message.rs       # Messaging API
│   │   ├── media.rs         # Media upload API
│   │   ├── board.rs         # Whiteboard API
│   │   └── ...
│   ├── commands/            # Command handlers
│   ├── output/              # Output formatting
│   └── error.rs             # Error types
├── Cargo.toml
├── .env.template
└── README.md
```

## Development

### Build

```bash
cargo build          # Debug build
cargo build --release  # Release build
```

### Test

```bash
cargo test
```

### Lint

```bash
cargo clippy
cargo fmt --check
```

## API Endpoints

| Endpoint | Description |
|----------|-------------|
| `POST /auth/v3/tenant_access_token/internal` | Authentication |
| `GET /wiki/v2/spaces/get_node` | Wiki node info |
| `GET /docx/v1/documents/{id}/raw_content` | Document content |
| `POST /docx/v1/documents` | Create document |
| `GET /docx/v1/documents/{id}/blocks` | Get blocks |
| `PATCH /docx/v1/documents/{id}/blocks/batch_update` | Update blocks |
| `POST /drive/v1/permissions/{token}/members` | Add permission |
| `POST /im/v1/messages` | Send message |
| `GET /im/v1/chats/search` | Search chats |
| `POST /board/v1/whiteboards/{id}/nodes` | Create board nodes |

## Error Handling

Lark CLI provides descriptive error messages for common issues:

- Configuration file not found or invalid
- Network connectivity problems
- API authentication failures
- Resource not found errors
- Rate limiting responses

## FAQ

**Q: Where should I place the `.env` file?**

A: The recommended location is `~/.config/lark-cli/.env` for global access. Alternatively, place it in the same directory as the executable.

**Q: How do I enable debug logging?**

A: Use the `-v` or `--verbose` flag: `lark-cli -v get-node <token>`

**Q: What output formats are supported?**

A: Text (default) and JSON. Use `--format json` for JSON output.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please read the contributing guidelines before submitting pull requests.

---

Built with Rust
