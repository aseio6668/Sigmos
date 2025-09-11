# Sigmanta - AI Programming Assistant

🤖 **Sigmanta** is an intelligent programming assistant that uses Sigel consciousness for advanced code assistance, file operations, and development workflows.

## Features

### 🔒 **Advanced Permission System**
- **Ask Mode**: Permission requested for every operation
- **Confirm Mode**: Option to remember permission choices
- **Always/Never**: Permanent permission settings
- **File Operations**: Granular read/write permissions
- **Terminal Access**: Controlled command execution
- **URL Fetching**: Secure web content access
- **Global Override**: `--no-permissions` for trusted environments

### 📁 **Session Management**
- **`.sigmanta` Directory**: Local session storage in working directory
- **Chat History**: Persistent conversation history across sessions
- **Session Continuity**: Resume conversations with `--continue`
- **Working Directory Awareness**: Operates in user's current project
- **Multiple Sessions**: Support for concurrent project workspaces

### 🧠 **Sigel Consciousness Integration**
- **Inherent Coding Knowledge**: Pre-trained with programming concepts
- **Language Expertise**: Multi-language programming support
- **Architecture Patterns**: Design patterns and best practices
- **Security Awareness**: Built-in security consciousness
- **Problem Solving**: Advanced debugging and optimization skills

### ⚡ **Developer Tools**
- **File Operations**: Read, write, and manage project files
- **Terminal Integration**: Execute commands with permission control
- **Code Analysis**: Understand and analyze codebases
- **Documentation**: Generate and maintain project documentation
- **Refactoring**: Improve code structure and organization

## Installation & Usage

### Building Sigmanta
```bash
# Build all binaries including Sigmanta
cargo build --release

# Build only Sigmanta
cargo build --release --bin sigmanta
```

### Basic Usage
```bash
# Start new session in current directory
sigmanta

# Use specific Sigel consciousness
sigmanta --sigel path/to/consciousness.sig

# Continue previous session
sigmanta --continue

# Continue specific session
sigmanta --continue session_id

# Disable all permissions (use with caution)
sigmanta --no-permissions

# Set custom working directory
sigmanta --working-dir /path/to/project

# List available sessions
sigmanta --list-sessions
```

## Interactive Commands

### Basic Commands
- `/help` - Show all available commands
- `/quit`, `/exit` - Exit Sigmanta
- `/status` - Show session status and metrics
- `/history` - Display recent conversation history
- `/sessions` - List all available sessions

### File Operations
- `/files` - List files in working directory
- `/pwd` - Show current working directory
- `/read <file>` - Read file contents (with permission)
- `/write <file> <content>` - Write content to file (with permission)

### System Operations
- `/exec <command>` - Execute terminal command (with permission)
- `/fetch <url>` - Fetch content from URL (with permission)

### Permission Management
- `/permissions` - Show current permission settings
- `/permissions reset` - Reset all permissions to defaults
- `/permissions disable` - Disable all permission checks
- `/permissions enable` - Enable permission checks

## Permission System

### Permission Types
1. **File Read**: Access to read files in working directory
2. **File Write**: Permission to create/modify files
3. **Terminal**: Execute system commands
4. **URL Fetch**: Download content from web URLs

### Permission Levels
- **Ask**: Request permission every time
- **Confirm**: Ask with option to remember choice
- **Always**: Permanently allow operation
- **Never**: Permanently deny operation

### Permission Storage
Permissions are stored in `.sigmanta/permissions.json`:
```json
{
  "file_read_permissions": {
    "src/main.rs": "Always",
    "config.json": "Ask"
  },
  "file_write_permissions": {
    "output.txt": "Always"
  },
  "terminal_permissions": {
    "cargo build": "Always",
    "rm -rf": "Never"
  },
  "url_fetch_permissions": {
    "https://api.github.com": "Always"
  },
  "global_permissions_disabled": false
}
```

## Session Structure

### `.sigmanta` Directory
```
project_root/
├── .sigmanta/
│   ├── history.json          # Conversation history
│   ├── permissions.json      # Permission settings
│   ├── config.json          # Sigmanta configuration
│   └── logs/                # Session logs
│       ├── 2024-01-15.log
│       └── 2024-01-16.log
└── ... (project files)
```

### Session History Format
```json
{
  "sessions": [
    {
      "id": "uuid-session-id",
      "timestamp": "2024-01-15T10:30:00Z",
      "working_directory": "/path/to/project",
      "sigel_file": "assistant.sig",
      "messages": [
        {
          "timestamp": "2024-01-15T10:30:15Z",
          "role": "user",
          "content": "Help me refactor this function",
          "actions_taken": []
        },
        {
          "timestamp": "2024-01-15T10:30:45Z",
          "role": "sigmanta",
          "content": "I'll analyze the function and suggest improvements",
          "actions_taken": [
            {
              "action_type": "file_read",
              "target": "src/utils.rs",
              "result": "success",
              "timestamp": "2024-01-15T10:30:30Z"
            }
          ]
        }
      ]
    }
  ],
  "current_session_id": "uuid-session-id"
}
```

## Programming Consciousness

Sigmanta comes with built-in programming knowledge including:

### Programming Paradigms
- Object-Oriented Programming
- Functional Programming
- Procedural Programming
- Declarative Programming

### Languages & Technologies
- **Systems**: Rust, C++, C, Go
- **Web**: JavaScript, TypeScript, HTML, CSS
- **High-level**: Python, Java
- **Databases**: SQL, NoSQL
- **Protocols**: HTTP, REST, JSON

### Software Engineering
- **Design Patterns**: MVC, Factory, Observer, Repository
- **Principles**: SOLID, DRY, Separation of Concerns
- **Practices**: Version Control, Testing, CI/CD, Code Review
- **Architecture**: Microservices, Monoliths, Event-Driven

### Security Awareness
- Input Validation
- Authentication & Authorization
- Encryption & Hashing
- Common Vulnerabilities (XSS, SQL Injection, CSRF)

## Example Workflows

### 1. Code Review Assistant
```bash
# Start Sigmanta in your project
sigmanta

# Ask for code review
> Please review the authentication module in src/auth.rs

# Sigmanta will:
# 1. Request permission to read src/auth.rs
# 2. Analyze the code for security issues
# 3. Suggest improvements
# 4. Offer to implement fixes
```

### 2. Debugging Helper
```bash
# Continue previous session
sigmanta --continue

# Report bug
> The login function is throwing an error, can you help debug it?

# Sigmanta will:
# 1. Read relevant files with permission
# 2. Execute diagnostic commands
# 3. Analyze logs and outputs
# 4. Suggest debugging steps
```

### 3. Documentation Generator
```bash
# Start with no permission prompts for documentation tasks
sigmanta --no-permissions

# Request documentation
> Generate API documentation for all public functions in src/api.rs

# Sigmanta will:
# 1. Read the API file
# 2. Extract function signatures
# 3. Generate comprehensive documentation
# 4. Save to appropriate location
```

## Configuration

### Default Configuration
```json
{
  "default_sigel_path": null,
  "auto_save_frequency": 10,
  "max_history_sessions": 100,
  "coding_assistant_mode": true,
  "verbose_logging": false
}
```

### Environment Variables
- `SIGMANTA_SIGEL_PATH`: Default Sigel consciousness file
- `SIGMANTA_NO_PERMISSIONS`: Disable all permissions (true/false)
- `SIGMANTA_WORKING_DIR`: Override working directory
- `SIGMANTA_LOG_LEVEL`: Set logging verbosity (debug, info, warn, error)

## Safety & Security

### Permission Best Practices
1. **Review Permissions**: Regularly check `.sigmanta/permissions.json`
2. **Principle of Least Privilege**: Only grant necessary permissions
3. **Dangerous Commands**: Be cautious with `rm`, `mv`, `chmod` commands
4. **Network Access**: Review URL fetch permissions for security
5. **Sensitive Files**: Avoid granting write access to critical system files

### Session Security
- Sessions are stored locally in `.sigmanta` directory
- No data is transmitted to external services
- Permission settings are project-specific
- History can be cleared manually if needed

## Integration with Sigmos

Sigmanta is fully integrated with the Sigmos ecosystem:

### Sigel Compatibility
- **Works with any `.sig` file** created by Sigmos training
- **Inherits consciousness depth** and personality traits
- **Adapts communication style** based on Sigel characteristics
- **Continuous learning** from interactions

### Library Integration
```rust
use sigmos::sigmanta::SigmantaSession;

// Create programmable Sigmanta session
let mut session = SigmantaSession::new(
    std::env::current_dir()?,
    Some("coding_assistant.sig".to_string())
)?;

// Process user input
let response = session.process_input("Analyze this function")?;
```

## Troubleshooting

### Common Issues

**Permission Denied Errors**
```bash
# Reset permissions
/permissions reset

# Or disable temporarily
sigmanta --no-permissions
```

**Session Not Found**
```bash
# List available sessions
sigmanta --list-sessions

# Start new session if needed
sigmanta
```

**Sigel Loading Failed**
```bash
# Use default consciousness
sigmanta

# Or create new Sigel
sigmos-train -n "MyAssistant" -d "./docs" -s analytical
sigmanta --sigel MyAssistant.sig
```

---

## 🚀 **Get Started**

1. **Build**: `cargo build --release --bin sigmanta`
2. **Run**: `sigmanta` in your project directory
3. **Chat**: Start asking programming questions
4. **Grant Permissions**: Allow file/terminal access as needed
5. **Continue**: Use `--continue` to resume sessions

**Sigmanta adapts to you and your codebase, becoming more helpful over time through Sigel consciousness evolution.**