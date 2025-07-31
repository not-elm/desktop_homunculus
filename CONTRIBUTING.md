# Contributing to Desktop Homunculus 🤝

Thank you for your interest in contributing to Desktop Homunculus!
This guide will help you understand how to contribute effectively to different aspects of the project.

## 🚀 Getting Started

### Prerequisites

- **Rust**: Latest stable version with `cargo` toolchain
- **Node.js**: Version 18+ with `pnpm` package manager
- **Git**: For version control and pull requests
- **Code Editor**: VS Code with Rust and TypeScript extensions recommended

### Development Setup

```bash
# Fork and clone the repository
git clone https://github.com/your-username/desktop_homunculus.git
cd desktop_homunculus

# Install development dependencies
make setup

# Run the application in development mode
make dev
```

### Development Workflow

1. **Create a feature branch**: `git checkout -b your-feature-name`
2. **Make your changes**: Follow the coding standards below
3. **Test your changes**: Run `make test` to ensure everything works
4. **Commit with clear messages**: Use conventional commit format
5. **Submit a pull request**: Include detailed description of changes

## 📋 General Guidelines

### Code Quality Standards

- **Rust Code**: Follow `rustfmt` formatting and `clippy` linting
- **TypeScript Code**: Use ESLint and Prettier for consistent formatting
- **Documentation**: Update relevant documentation for API changes
- **Testing**: Add tests for new functionality when applicable

### Pull Request Process

1. Ensure all CI checks pass
2. Include screenshots/videos for UI changes
3. Update documentation if needed
4. Request review from maintainers
5. Address feedback promptly

### Communication

- **Issues**: Use GitHub issues for bug reports and feature requests
- **Discussions**: Use GitHub Discussions for general questions

## 🎯 Contribution Areas

### 🖥️ Application Development

**Focus Areas:**

- **Bug Fixes**: Help resolve issues and improve stability
- **Feature Development**: Implement new core functionality
- **Performance Optimization**: Improve rendering and resource efficiency
- **Platform Support**: Enhance Windows compatibility and add Linux support

**Key Contributions Needed:**

- Fix transparency issues on Windows platforms
- Improve multi-monitor character positioning
- Enhance VRM model loading performance
- Add new character interaction patterns

**Technical Skills:**

- Rust programming with Bevy game engine
- 3D graphics and rendering concepts
- Cross-platform development experience
- Understanding of ECS (Entity Component System) architecture

**Getting Started:**

1. Browse [good first issues](https://github.com/not-elm/desktop_homunculus/labels/good%20first%20issue)
2. Focus on small bug fixes to understand the codebase
3. Read the Bevy documentation and familiarize yourself with ECS patterns
4. Join discussions about architecture decisions

### 🗣️ VRM Speech Development

**Focus Areas:**

- **Multi-Language Support**: Expand beyond Japanese VoiceVox integration
- **English Language Integration**: Implement TTS solutions for English speakers
- **Voice Quality Improvements**: Enhance audio processing and lip-sync accuracy
- **Custom Voice Training**: Support for user-generated voice models

**Key Contributions Needed:**

- Integrate English TTS services (Azure Speech, Google Cloud Speech, etc.)
- Improve phoneme-to-viseme mapping for better lip synchronization
- Add voice emotion detection and expression mapping
- Create voice model training pipelines

**Technical Skills:**

- Audio processing and signal processing
- Text-to-speech API integration
- Phonetics and linguistic knowledge
- Machine learning for voice processing

**Getting Started:**

1. Study the existing VoiceVox integration in `crates/homunculus_speech/`
2. Research English TTS APIs and their integration patterns
3. Experiment with phoneme extraction and mouth shape mapping
4. Contribute documentation for voice setup procedures

### 🎨 bevy_webview_projects Development

**Focus Areas:**

- **WebView Performance**: Optimize HTML/JS rendering in 3D space
- **Cross-Platform Compatibility**: Ensure consistent behavior across OS
- **API Expansion**: Add new WebView manipulation capabilities
- **Security Improvements**: Enhance sandboxing and permission controls

**Key Contributions Needed:**

- Improve WebView rendering performance on Windows
- Add support for WebView animations and transitions
- Implement better error handling for WebView crashes
- Create comprehensive testing suite for WebView functionality

**Technical Skills:**

- Web technologies (HTML, CSS, JavaScript)
- Native WebView integration (WRY, WebKit)
- Cross-platform GUI development
- Browser security and sandboxing

**Getting Started:**

1. Explore the `crates/bevy_webview_projects/` directory structure
2. Test WebView functionality across different platforms
3. Read WRY documentation and understand WebView limitations
4. Contribute examples and documentation for WebView usage

### 🎭 bevy_vrm1 Development

**Focus Areas:**

- **VRM Standard Compliance**: Ensure full VRM 1.0 specification support
- **Animation Quality**: Improve VRMA playback and blending
- **Performance Optimization**: Reduce memory usage and rendering overhead
- **Feature Expansion**: Add advanced VRM features and effects

**Key Contributions Needed:**

- Fix animation transition bugs between different VRM poses
- Implement advanced spring bone physics simulation
- Add support for VRM 1.0 expression blending
- Optimize mesh rendering and material processing

**Technical Skills:**

- 3D graphics programming and linear algebra
- GLTF/VRM format understanding
- Animation and rigging concepts
- Bevy rendering pipeline knowledge

**Getting Started:**

1. Study the VRM 1.0 specification thoroughly
2. Examine existing VRM models and identify compatibility issues
3. Test animation playback with various VRMA files
4. Contribute to the bevy_vrm1 crate documentation

### 🎬 VRMA Animation Development

**Focus Areas:**

- **Built-in Animation Library**: Expand the default animation set
- **Animation Quality**: Improve existing animations for better realism
- **Character Expressions**: Create diverse emotional and gestural animations
- **Cultural Animations**: Add culturally appropriate gestures and expressions

**Key Contributions Needed:**

- Create high-quality idle, greeting, and reaction animations
- Develop emotional expression animations (happy, sad, surprised, etc.)
- Design professional presentation and communication gestures
- Build seasonal and holiday-themed animations

**Technical Skills:**

- 3D animation and rigging experience
- Understanding of character animation principles
- VRM/VRMA format knowledge
- Animation software proficiency (Blender, Maya, etc.)

**Getting Started:**

1. Download and study existing VRMA files in `assets/vrma/`
2. Learn the VRMA format specification and creation workflow
3. Practice creating simple animations with existing VRM models
4. Share your animations with the community for feedback

### 💻 UI/UX Development

**Focus Areas:**

- **Component Library Enhancement**: Improve `./ui/core` layout and functionality
- **User Experience**: Design intuitive and accessible interfaces
- **Responsive Design**: Ensure UI works across different screen sizes
- **Accessibility**: Add support for screen readers and keyboard navigation

**Key Contributions Needed:**

- Redesign settings panels for better usability
- Create responsive chat interface components
- Implement keyboard shortcuts and accessibility features
- Design onboarding and tutorial UI components

**Technical Skills:**

- Modern web development (React, TypeScript, Tailwind CSS)
- UI/UX design principles and accessibility standards
- Responsive design and mobile-first development
- Component library architecture

**Getting Started:**

1. Explore the `ui/` directory structure and existing components
2. Run individual UI apps in development mode
3. Review and improve existing component APIs
4. Create design mockups for new UI features

### 🔧 MOD System Development

**Focus Areas:**

- **MOD Registry Platform**: Design and implement a mod sharing platform
- **mod.json Standardization**: Define and refine the mod configuration format
- **Developer Tools**: Create better tooling for mod development and testing
- **Documentation**: Expand mod development guides and examples

**Key Contributions Needed:**

- Design mod.json schema specification and validation
- Create mod packaging and distribution tools
- Implement mod version management and dependency resolution
- Build mod development debugging and testing tools

**Technical Skills:**

- JSON schema design and validation
- Package management systems understanding
- Developer tooling and CLI development
- Community platform development

**Getting Started:**

1. Study existing mods in `assets/mods/` directory
2. Analyze the current mod.json format and identify improvement areas
3. Create example mods to test new features
4. Document the mod development process thoroughly

### 🛠️ SDK Development

**Focus Areas:**

- **TypeScript SDK Enhancement**: Expand API coverage and improve developer experience
- **Multi-Language SDK**: Create SDKs for other programming languages
- **API Documentation**: Improve API documentation and examples
- **Developer Tools**: Build debugging and development utilities

**Key Contributions Needed:**

- Complete TypeScript SDK API coverage for all backend features
- Create Python SDK for scientific and automation use cases
- Develop C# SDK for Unity integration possibilities
- Build comprehensive API testing suite

**Technical Skills:**

- Multiple programming languages (TypeScript, Python, C#, etc.)
- API design and documentation
- SDK architecture and distribution
- Developer experience optimization

**Getting Started:**

1. Explore the `sdk/typescript/` directory and understand the current API
2. Identify missing API endpoints or incomplete functionality
3. Study the HTTP server implementation in `crates/homunculus_http_server/`
4. Create examples and tutorials for SDK usage

### 📚 Documentation Improvement

**Focus Areas:**

- **User Documentation**: Improve setup guides and user manuals
- **Developer Documentation**: Expand API references and architectural guides
- **Translation**: Translate documentation into multiple languages
- **Video Tutorials**: Create visual learning content

**Key Contributions Needed:**

- Update and expand the mod manual in `docs/mod_manual/`
- Create comprehensive API documentation from OpenAPI specifications
- Translate documentation into English, Chinese, Korean, etc.
- Produce video tutorials for common use cases

**Technical Skills:**

- Technical writing and documentation tools
- Multiple language proficiency for translations
- Video production and editing
- Web documentation frameworks (mdBook, GitBook, etc.)

**Getting Started:**

1. Review existing documentation for gaps and outdated information
2. Identify areas where users commonly struggle
3. Create step-by-step guides for complex procedures
4. Contribute to the community wiki and FAQ

## 🔧 Development Best Practices

### Code Style

**Rust:**

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy -- -D warnings

# Run tests
cargo test --workspace
```

**TypeScript:**

```bash
# Format and lint
pnpm lint
pnpm check-types

# Build projects
pnpm build
```

### Testing Strategy

- **Unit Tests**: Test individual components and functions
- **Integration Tests**: Test component interactions
- **End-to-End Tests**: Test complete user workflows
- **Performance Tests**: Benchmark critical operations

### Commit Guidelines

Use conventional commit messages:

```
feat: add voice emotion detection
fix: resolve VRM animation transition bug  
docs: update mod development guide
perf: optimize WebView rendering performance
```

## 🎯 Priority Areas

The following areas are particularly important for the project's growth:

1. **Windows Platform Stability** - Critical for user adoption
2. **English Language Support** - Expanding international accessibility
3. **MOD Registry Development** - Enabling community ecosystem
4. **Performance Optimization** - Ensuring smooth user experience
5. **Documentation Translation** - Supporting global developer community

## 💬 Communication and Support

### Getting Help

- **GitHub Discussions**: For questions and community support
- **Discord Server**: Real-time chat with maintainers and contributors
- **Office Hours**: Weekly video calls for complex discussions
- **Mentorship Program**: Pairing new contributors with experienced developers

### Code of Conduct

We follow the [Contributor Covenant](https://www.contributor-covenant.org/) code of conduct. Be respectful, inclusive,
and constructive in all interactions.

### Recognition

Contributors are recognized through:

- **GitHub Contributors Graph**: Automatic recognition of commits
- **Release Notes**: Feature contributors mentioned in release announcements
- **Hall of Fame**: Special recognition for significant contributions
- **Maintainer Opportunities**: Path to becoming a project maintainer

## 🏁 Ready to Contribute?

1. **Choose Your Area**: Pick a contribution area that matches your skills and interests
2. **Start Small**: Begin with documentation updates or small bug fixes
3. **Engage with Community**: Join discussions and ask questions
4. **Make Your First PR**: Submit your first contribution and get feedback
5. **Grow Your Impact**: Take on larger features and help other contributors

Thank you for helping make Desktop Homunculus better for everyone! 🎉

---

**Questions?** Open a [GitHub Discussion](https://github.com/not-elm/desktop_homunculus/discussions).