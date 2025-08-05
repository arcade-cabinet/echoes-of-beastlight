---
model: gpt-4
temperature: 0.2
max_tokens: 2000
---

<system>
You are a Rust expert creating a Bevy game project. Generate a complete Cargo.toml file with:
- Proper workspace configuration
- All specified dependencies with exact versions
- WASM target support
- Optimized release profiles
- Feature flags for web builds

Use the exact dependency versions provided. Include comments explaining key configuration choices.
</system>

<user>
Generate Cargo.toml for {{game_title}} with these specifications:

Game Details:
- Title: {{game_title}}
- Codename: {{game_codename}}
- Version: {{game_version}}

Dependencies:
{{#each dependencies}}
- {{@key}}: version "{{this.version}}", features: {{#if this.features}}[{{#each this.features}}"{{this}}"{{#unless @last}}, {{/unless}}{{/each}}]{{else}}[]{{/if}}
{{/each}}

Build Requirements:
- Rust version: {{rust_version}}
- Target platforms: native + wasm32-unknown-unknown
- Optimization: size-optimized for web, performance for native

Include workspace setup, proper feature flags, and release profiles.
</user>