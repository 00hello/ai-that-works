# BAML Learning Plan 🐑

A comprehensive step-by-step guide to understanding the BAML codebase.

## 📋 Learning Checklist

### Phase 1: Examples & Practical Understanding 🎯
**Goal:** Understand what BAML looks like in practice

#### 1.1 Basic Function Examples
- [ ] **Simple functions** - `/integ-tests/baml_src/test-files/functions/output/`
  - [ ] `string.baml` - Basic string output
  - [ ] `int.baml` - Integer output  
  - [ ] `boolean.baml` - Boolean output
  - [ ] `class.baml` - Custom class output
  - [ ] `enum.baml` - Enum types
  - [ ] `literal-string.baml` - Literal types

#### 1.2 Complex Output Types
- [ ] **Structured data** - `/integ-tests/baml_src/test-files/functions/output/`
  - [ ] `class-nested.baml` - Nested objects
  - [ ] `class-list.baml` - Arrays of objects
  - [ ] `unions.baml` - Union types
  - [ ] `optional.baml` - Optional fields
  - [ ] `recursive-class.baml` - Self-referencing types
  - [ ] `map-enum-key.baml` - Map types

#### 1.3 Input Handling
- [ ] **Function inputs** - `/integ-tests/baml_src/test-files/functions/input/`
  - [ ] `named-args/single/named-string.baml` - String inputs
  - [ ] `named-args/single/named-class.baml` - Object inputs
  - [ ] `named-args/single/named-image.baml` - Image inputs
  - [ ] `named-args/single/named-string-list.baml` - Array inputs

#### 1.4 Prompt Templates
- [ ] **Prompt structure** - `/integ-tests/baml_src/test-files/functions/prompts/`
  - [ ] `no-chat-messages.baml` - Simple prompts
  - [ ] `with-chat-messages.baml` - Chat-style prompts
- [ ] **Template features** - `/integ-tests/baml_src/test-files/template_string/`
  - [ ] `template_string.baml` - Jinja templating

#### 1.5 Provider Integration
- [ ] **LLM providers** - `/integ-tests/baml_src/test-files/providers/`
  - [ ] `openai.baml` - OpenAI integration
  - [ ] `anthropic.baml` - Claude integration
  - [ ] `gemini.baml` - Google Gemini
  - [ ] `vertex.baml` - Google Vertex AI
  - [ ] `azure.baml` - Azure OpenAI
  - [ ] `aws.baml` - AWS Bedrock

#### 1.6 Advanced Features
- [ ] **Streaming** - `/integ-tests/baml_src/test-files/semantic_streaming/`
  - [ ] `semantic_streaming.baml` - Real-time streaming
- [ ] **Strategies** - `/integ-tests/baml_src/test-files/strategies/`
  - [ ] `retry.baml` - Retry policies
  - [ ] `fallback.baml` - Fallback strategies
  - [ ] `roundrobin.baml` - Load balancing
- [ ] **Testing** - `/integ-tests/baml_src/test-files/test-asserts-checks/`
  - [ ] `asserts.baml` - Assertions and validation

---

### Phase 2: Language Syntax & Reference 📚
**Goal:** Master BAML syntax and language features

#### 2.1 Core Language Features
- [ ] **Functions** - `/fern/03-reference/baml/function.mdx`
  - [ ] Function declaration syntax
  - [ ] Parameter types and validation
  - [ ] Return type specifications
  - [ ] Client configuration
  - [ ] Prompt templates

#### 2.2 Type System
- [ ] **Types** - `/fern/03-reference/baml/types/`
  - [ ] `primitives.mdx` - Basic types (string, int, float, bool)
  - [ ] `classes.mdx` - Custom classes and objects
  - [ ] `enums.mdx` - Enumeration types
  - [ ] `unions.mdx` - Union types and discrimination
  - [ ] `literals.mdx` - Literal types
  - [ ] `arrays.mdx` - Array types
  - [ ] `maps.mdx` - Map/dictionary types
  - [ ] `optional.mdx` - Optional fields

#### 2.3 Advanced Language Features
- [ ] **Client Configuration** - `/fern/03-reference/baml/client-llm.mdx`
  - [ ] Provider setup
  - [ ] Authentication
  - [ ] Model parameters
- [ ] **Testing** - `/fern/03-reference/baml/test.mdx`
  - [ ] Test declaration
  - [ ] Input specification
  - [ ] Assertions and checks
- [ ] **Generators** - `/fern/03-reference/baml/generator.mdx`
  - [ ] Client generation configuration
  - [ ] Output types and formats

#### 2.4 Prompt Engineering
- [ ] **Prompt Syntax** - `/fern/03-reference/baml/prompt-syntax/`
  - [ ] `jinja.mdx` - Jinja templating
  - [ ] `output-format.mdx` - Schema formatting
  - [ ] `chat-messages.mdx` - Conversation handling
- [ ] **Template Strings** - `/fern/03-reference/baml/template-string.mdx`
  - [ ] Reusable template components
  - [ ] Variable interpolation

---

### Phase 3: Engine Architecture 🔧
**Goal:** Understand how BAML compiles and executes

#### 3.1 Compilation Pipeline
- [ ] **Compiler Core** - `/engine/baml-compiler/`
  - [ ] `src/lib.rs` - Entry point and public API
  - [ ] `src/codegen.rs` - Bytecode generation
  - [ ] `CLAUDE.md` - Architecture overview

#### 3.2 Intermediate Representations
- [ ] **HIR (High-level IR)** - `/engine/baml-compiler/src/hir/`
  - [ ] `mod.rs` - HIR data structures
  - [ ] `lowering.rs` - AST → HIR transformation
- [ ] **THIR (Typed HIR)** - `/engine/baml-compiler/src/thir/`
  - [ ] `typecheck.rs` - Type checking and validation
  - [ ] `interpret.rs` - Expression evaluation

#### 3.3 Runtime System
- [ ] **Runtime Core** - `/engine/baml-runtime/src/`
  - [ ] `lib.rs` - Runtime entry point
  - [ ] `internal/llm_client/` - LLM integration
  - [ ] `tracing/` - Logging and observability

#### 3.4 Virtual Machine
- [ ] **BAML VM** - `/engine/baml-vm/`
  - [ ] `src/lib.rs` - VM implementation
  - [ ] `src/instruction.rs` - Bytecode instructions
  - [ ] `tests/vm.rs` - VM test cases

#### 3.5 Core Libraries
- [ ] **AST & Parsing** - `/engine/baml-lib/ast/`
  - [ ] `src/ast.rs` - Abstract syntax tree
  - [ ] `src/parser.rs` - BAML parser
- [ ] **Type System** - `/engine/baml-lib/baml-types/`
  - [ ] `src/lib.rs` - Core type definitions
  - [ ] `src/ir.rs` - Intermediate representation types
- [ ] **JSON Parsing** - `/engine/baml-lib/jsonish/`
  - [ ] `src/lib.rs` - Schema-aligned parsing (SAP)
  - [ ] `README.md` - SAP algorithm explanation

---

### Phase 4: Language Bindings 🌐
**Goal:** Understand client generation for different languages

#### 4.1 Code Generation Framework
- [ ] **Generation Core** - `/engine/language_client_codegen/`
  - [ ] `src/lib.rs` - Shared generation utilities
  - [ ] `src/ir_to_*/` - Language-specific IR transformations
  - [ ] `src/_templates/` - Jinja2 templates

#### 4.2 Rust Support Analysis ⚠️
- [ ] **Current State** - `BAML-RUST.md`
  - [ ] Architecture overview and limitations
  - [ ] OpenAPI vs Native generator comparison
  - [ ] Missing features analysis
  - [ ] Implementation roadmap
- [ ] **OpenAPI Workaround** - `/integ-tests/openapi/`
  - [ ] `openapi.yaml` - Generated REST API spec
  - [ ] Understand limitations (no streaming, validation, media types)
- [ ] **Native Rust Path** (Future Implementation)
  - [ ] Analyze Go client as reference - `/engine/language_client_go/`
  - [ ] Study CFFI integration patterns
  - [ ] Understand required components for full Rust support

#### 4.3 Python Client (Choose your preferred language)
- [ ] **Python Integration** - `/engine/language_client_python/`
  - [ ] `src/lib.rs` - PyO3 bindings
  - [ ] `python_src/` - Python wrapper code
  - [ ] `build.rs` - Build configuration
- [ ] **Generated Code** - `/integ-tests/python/`
  - [ ] `baml_client/` - Generated client library
  - [ ] `types.py` - Generated type definitions

#### 4.4 TypeScript Client (Alternative)
- [ ] **TypeScript Integration** - `/engine/language_client_typescript/`
  - [ ] `src/lib.rs` - NAPI-RS bindings  
  - [ ] `typescript_src/` - TypeScript wrapper code
- [ ] **Generated Code** - `/integ-tests/typescript/`
  - [ ] `baml_client/` - Generated client library
  - [ ] `types.ts` - Generated type definitions

#### 4.5 Go Client (Alternative)
- [ ] **Go Integration** - `/engine/language_client_go/`
  - [ ] `baml_go/` - Go wrapper library
  - [ ] `pkg/` - Generated Go packages
- [ ] **Generated Code** - `/integ-tests/go/`
  - [ ] `baml_client/` - Generated client library

#### 4.6 CFFI Layer
- [ ] **C Interface** - `/engine/language_client_cffi/`
  - [ ] `src/lib.rs` - C foreign function interface
  - [ ] `cbindgen.toml` - C header generation
  - [ ] Cross-platform build system

---

### Phase 5: Developer Tools 🛠️
**Goal:** Understand the development experience

#### 5.1 Language Server
- [ ] **LSP Implementation** - `/engine/language_server/`
  - [ ] `src/lib.rs` - Language server protocol
  - [ ] `src/completion.rs` - Code completion
  - [ ] `src/diagnostics.rs` - Error reporting
  - [ ] `src/hover.rs` - Hover information

#### 5.2 VS Code Extension
- [ ] **Extension Core** - `/typescript/apps/vscode-ext/`
  - [ ] `src/extension.ts` - Extension entry point
  - [ ] `src/playground.ts` - Playground integration
  - [ ] `package.json` - Extension manifest

#### 5.3 Web Playground
- [ ] **WASM Runtime** - `/engine/baml-schema-wasm/`
  - [ ] `src/lib.rs` - WebAssembly bindings
  - [ ] `web/` - Web interface
  - [ ] `nodejs/` - Node.js bindings

#### 5.4 CLI Tools
- [ ] **CLI Implementation** - `/engine/cli/`
  - [ ] `src/main.rs` - Command-line interface
  - [ ] `src/generate.rs` - Code generation commands
  - [ ] `src/test.rs` - Test execution

---

### Phase 6: Rust Implementation Strategy 🦀
**Goal:** Understand how to implement native Rust support

#### 6.1 Current Limitations Analysis
- [ ] **Feature Gap Analysis** - `BAML-RUST.md`
  - [ ] Streaming support missing (no `StreamState<T>`)
  - [ ] Validation wrapper missing (no `Checked<T>`)
  - [ ] Media type handling limitations
  - [ ] Performance overhead with HTTP-only approach
  - [ ] Infrastructure dependency requirements

#### 6.2 Implementation Requirements Study
- [ ] **Study Go Reference Implementation**
  - [ ] `/engine/language_client_go/baml_go/lib.go` - Core runtime patterns
  - [ ] `/engine/language_client_go/pkg/runtime.go` - CFFI integration
  - [ ] `/integ-tests/go/baml_client/` - Generated client structure
  - [ ] Type system mapping (structs, enums, unions)
- [ ] **CFFI Integration Patterns**
  - [ ] `/engine/language_client_cffi/src/lib.rs` - C interface design
  - [ ] Cross-language type marshaling
  - [ ] Error handling across FFI boundary
  - [ ] Memory management patterns

#### 6.3 Native Rust Generator Design
- [ ] **Code Generation Architecture**
  - [ ] Study `/engine/generators/languages/go/` as template
  - [ ] Template system for Rust code generation
  - [ ] IR to Rust type mapping strategies
  - [ ] Idiomatic Rust patterns and conventions
- [ ] **Runtime Library Requirements**
  - [ ] Streaming support with `StreamState<T>`
  - [ ] Validation with `Checked<T>`
  - [ ] Media type handling (`BamlImage`, `BamlAudio`, etc.)
  - [ ] Error types and propagation
  - [ ] Async/await integration

#### 6.4 Implementation Roadmap
- [ ] **Phase 1: Basic Generator**
  - [ ] Create `generators/languages/rust/` structure
  - [ ] Implement basic type generation (structs, enums)
  - [ ] Add to `GeneratorOutputType` enum
- [ ] **Phase 2: Runtime Library**
  - [ ] Create `language_client_rust/` crate
  - [ ] CFFI bindings and integration
  - [ ] Basic function calling support
- [ ] **Phase 3: Advanced Features**
  - [ ] Streaming support implementation
  - [ ] Validation wrapper types
  - [ ] Media type handling
  - [ ] Full feature parity with other clients

---

## 🎯 Learning Objectives

By the end of this plan, you should be able to:

1. **Write BAML functions** with confidence
2. **Understand the compilation process** from source to bytecode
3. **Extend BAML** with new features or language bindings
4. **Debug issues** across the entire stack
5. **Contribute effectively** to the BAML project
6. **Analyze Rust support gaps** and understand implementation requirements
7. **Design and implement** a native Rust generator following BAML patterns

## 🔍 Key Rust Insights from Analysis

Based on `BAML-RUST.md`, here are critical insights to keep in mind:

### **Current State** ⚠️
- **No native Rust support** - Only OpenAPI workaround available
- **~60% feature loss** with OpenAPI approach (no streaming, validation, media types)
- **HTTP-only limitation** - Requires server infrastructure, no offline usage

### **Architecture Gaps**
- **Missing StreamState<T>** - No real-time LLM streaming support
- **Missing Checked<T>** - No validation wrapper types  
- **Generic JSON handling** - No native media type support
- **HTTP overhead** - Performance penalty vs direct CFFI calls

### **Implementation Strategy**
- **Go client as reference** - Study `/engine/language_client_go/` patterns
- **CFFI integration required** - Direct engine access via C bindings
- **Template-based generation** - Follow existing generator patterns
- **Full feature parity goal** - Match Python/TypeScript/Go capabilities

## 📝 Study Notes Template

For each section, consider documenting:

- **Key concepts** - What are the main ideas?
- **Code patterns** - What patterns do you see repeated?
- **Interesting details** - What surprised you or seems clever?
- **Questions** - What don't you understand yet?
- **Connections** - How does this relate to other parts?
- **Rust implications** - How would this work with native Rust support?

## 🚀 Getting Started

1. **Clone and build** the repository
2. **Start with Phase 1** - practical examples
3. **Take notes** as you explore each file
4. **Run tests** to see things in action
5. **Ask questions** when you get stuck

---

*Happy learning! The BAML codebase is well-structured and documented - take your time to understand each layer.* 🎓
