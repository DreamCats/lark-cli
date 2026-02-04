# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust CLI tool for interacting with Lark API to fetch knowledge space node information and document content. The tool uses a hierarchical command structure with convenient aliases and supports both text and JSON output formats.

## Development Commands

### Build and Run
```bash
# Build debug version
cargo build

# Build release version
cargo build --release

# Run the tool directly
cargo run -- <args>

# Run tests
cargo test

# Check code without building
cargo check

# Lint code
cargo clippy

# Format code
cargo fmt
```

### Testing Commands
```bash
# Test with real API calls (requires .env file with valid credentials)
cargo run -- get-node <node_token>
cargo run -- get-content <document_id>
cargo run -- --format json wiki get-node <node_token>

# Test with verbose logging
cargo run -- -v get-node <node_token>
```

## Architecture Overview

### Command Module Structure (Post-Refactoring)
The codebase has been refactored to use a modular command structure. The main.rs file was reduced from 736 to 520 lines by extracting command handlers into separate modules:

- **Location**: `src/commands/` - Contains all command handlers
- **Modules**:
  - `wiki.rs` - Knowledge space operations (get-node)
  - `docx.rs` - Document content retrieval (get-content)
  - `blocks.rs` - Block operations (5 commands: convert_blocks, create_nested_blocks, get_blocks, batch_update_blocks, delete_blocks)
  - `file.rs` - File operations (read_file, write_file, upload_media)
  - `message.rs` - Messaging operations (send_message, search_chats, get_message_history)
  - `import.rs` - Document import functionality
  - `permission.rs` - Permission management
  - `document.rs` - Document creation and conversion

### Configuration Management
- **Location**: `src/config/mod.rs`
- **Key Feature**: Loads `.env` file from the same directory as the executable
- Uses `std::env::current_exe()` to locate the executable directory
- Validates that required environment variables (`APP_ID`, `APP_SECRET`) are present and non-empty

### Authentication System
- **Location**: `src/auth/mod.rs`
- **Implementation**: Simplified token management that fetches fresh `tenant_access_token` for each request
- **Trade-off**: No caching mechanism for simplicity, but could be optimized for production use

### API Client Architecture
- **Base Client**: `src/api/mod.rs` - Generic HTTP client with Lark API response format handling
- **Specialized APIs**: Each API endpoint has its own module in `src/api/`:
  - Document operations: `document.rs`, `docx.rs`, `convert_blocks.rs`, `create_nested_blocks.rs`, `import_documents.rs`, `get_blocks.rs`, `batch_update_blocks.rs`, `delete_blocks.rs`
  - Wiki operations: `wiki.rs`
  - Permission operations: `permission.rs`
  - File operations: `file.rs`, `media.rs`
  - Message operations: `message.rs`, `search_chats.rs`, `get_message_history.rs`
- **Response Format**: All APIs follow Lark's standard `{code, msg, data}` response structure

### Critical Implementation Details

#### Nested Response Handling
The Wiki API has a critical implementation detail for handling nested JSON responses:
```rust
// Lark API returns: {code: 0, data: {node: {...}}, msg: "success"}
// Intermediate structure required:
#[derive(Debug, Deserialize)]
struct WikiNodeResponse {
    node: KnowledgeSpaceNode,
}
```

#### Manual HTTP Request Handling
Some API methods (particularly in `wiki.rs`) use manual HTTP request building instead of the generic `ApiClient::get()` method to properly handle nested response structures.

### CLI Interface Design
- **Structure**: Hierarchical commands with aliases for convenience
- **Commands**:
  - Primary: `wiki get-node`, `docx get-content`
  - Aliases: `get-node`, `get-content`
- **Global Options**: `-v/--verbose`, `--format <text|json>`
- **Framework**: Uses `clap` with derive macros for type-safe parsing

### Output Formatting
- **Location**: `src/output/mod.rs`
- **Formats**: Text (using Debug trait) and JSON
- **Generic Implementation**: Works with any type that implements `Debug` or `Serialize`

### Error Handling
- **Location**: `src/error.rs`
- **Strategy**: Comprehensive error types with Chinese error messages for user-friendliness
- **Error Types**: Config, Network, API, Parse, and Auth errors

## Environment Configuration

### Required Environment Variables
- `APP_ID`: Lark application ID
- `APP_SECRET`: Lark application secret

### Environment File Location
The tool searches for `.env` file in the following order:
1. Same directory as the executable
2. User config directory: `~/.config/lark-cli/.env`

Use `.env.template` as a template for the required format.

## API Endpoints Used

1. **Authentication**: `POST https://open.larkoffice.com/open-apis/auth/v3/tenant_access_token/internal`
2. **Wiki Node Info**: `GET https://open.larkoffice.com/open-apis/wiki/v2/spaces/get_node`
3. **Document Content**: `GET https://open.larkoffice.com/open-apis/docx/v1/documents/{document_id}/raw_content`
4. **Document Operations**: Various endpoints under `https://open.larkoffice.com/open-apis/docx/v1/`
5. **File Operations**: Media upload at `https://open.larkoffice.com/open-apis/im/v1/files`
6. **Message Operations**: Various endpoints under `https://open.larkoffice.com/open-apis/im/v1/`

## Development Notes

### Token Management Consideration
Current implementation fetches a fresh token for each request. For production use, consider implementing token caching with expiration handling to reduce API calls.

### Error Message Localization
All user-facing error messages are in Chinese to match the user requirements. Maintain this consistency when adding new error messages.

### Testing Strategy
The project relies on manual testing with real API calls due to the nature of external API integration. Ensure valid credentials are available in the `.env` file for testing.

### Recent Refactoring Notes
The codebase underwent significant refactoring to improve modularity:
- Command handlers extracted from main.rs into individual modules
- Each command now has its own async handler function
- Main.rs reduced from 736 to 520 lines (30% reduction)
- All existing functionality preserved with backward compatibility
- Tests updated to reflect new module structure