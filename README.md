# HMLR (HyperMedia Language Runtime)

[![Ecosystem: hyperlibs](https://img.shields.io/badge/ecosystem-hyperlibs-blue.svg)](https://github.com/hyperlibs)
[![Subsystems: 6 Active/Planned](https://img.shields.io/badge/subsystems-6_active%2Fplanned-green.svg)](MANIFEST.mx)
[![Format: Pure .mx / .fx](https://img.shields.io/badge/schema-pure_.mx%2F.fx-purple.svg)](MANIFEST.mx)
[![Runtime Size: <50KB](https://img.shields.io/badge/runtime-<50KB-brightgreen.svg)](#wasm-execution-vm)

**HMLR** is the universal runtime, execution engine, and developer inspection layer for the [hyperlibs](https://github.com/hyperlibs) ecosystem (`htmxUI` + `htmFX` + `HMLR`).

It executes `.fx` functional programs and `.mx` spatial documents natively, provides an ultra-light (<50KB) WebAssembly/Edge VM for Cloudflare Workers, and ships as a Chromium/Firefox DevTools extension to audit hypermedia velocity and inspect live reactive signals and 3D viewports.

---

## 🏛️ Subsystems Matrix

| Subsystem | Name | Description |
| :--- | :--- | :--- |
| `parser_mx` | **Streaming .mx Parser** | Zero-copy parser for flat spatial coordinate grammar (`@pin`, `@3d`, `@model`, TSV tables) with 75% token reduction vs JSON. |
| `diagnostics` | **AI Self-Healing Engine** | Deterministic `@diag FX-XXXX` structured diagnostics for autonomous AI code repair. |
| `extension` | **Browser DevTools Ext** | Manifest V3 extension with 4 panels: Signals, Hypermedia Audit, 3D Spatial HUD, and Agentic .mx Telemetry. |
| `wasm_vm` | **Wasm / Edge VM** | High-performance <50KB runtime for Cloudflare Workers & browsers (`fetch(req) -> Response`). Zero Node.js legacy deps. |
| `compiler_fx` | **.fx Compiler Kernel** | Functional, immutable compiler with pattern matching, `Result<T,E>`, and HMLR Intermediate Representation (HIR). |
| `bus` | **Ecosystem Sync Bus** | Real-time synchronization with `UPDATE.mx` event bus from `hyperlibs/htmxUI`. |

---

## 🌐 Ecosystem Communication Bus (`UPDATE.mx`)

HMLR connects to the single source of truth master event bus:
```
https://raw.githubusercontent.com/hyperlibs/htmxUI/master/docs/UPDATE.mx
```

Sync command:
```bash
npx hmlr sync
```

---

## ⛔ Hard Ecosystem Boundaries

1. **NO UI REIMPLEMENTATION**: UI primitives belong exclusively to `htmxUI`.
2. **NO 3D SHADER REIMPLEMENTATION**: 3D spatial rendering belongs exclusively to `htmFX`.
3. **NO JSON FOR INTERNAL SCHEMAS**: All internal schemas, diagnostics, and data streams inside HMLR use `.mx` and `.fx`.
4. **NO HEAVY RUNTIMES**: Core runtime targets Wasm, Edge Workers, and WebExtensions natively without Node.js legacy built-ins.

---

## 🚀 Quick Start

```bash
# Install dependencies
npm install

# Build TypeScript
npm run build

# Run unit and integration tests
npm test

# Run sync bus
npm run sync
```

## 📜 License
MIT © hyperlibs
