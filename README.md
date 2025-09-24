# TEQLA

**TEQLA** é uma L1 pós‑quântica, *feeless*, baseada em **DAG + Proof‑of‑Useful‑Work (PoUW)** — co‑criada por humano + IA.

> **Feeless. Post‑Quantum. Useful.**

## ✨ Diferenciais
- **Feeless UX** – micropagamentos viáveis.
- **Pós‑quântica** – Dilithium/SPHINCS+ desde o início.
- **PoUW simples e verificável** – anti‑spam com utilidade mínima determinística.
- **DX primeiro** – compatibilidade **EVM**, **SDK TypeScript** e **indexação nativa**.

## 📚 Documentação
- [Whitepaper v0.1](./docs/whitepaper-v0.1.md)

## 🛠 Estrutura do repositório
```
teqla/
├─ README.md
├─ LICENSE
├─ .gitignore
├─ docs/
│  └─ whitepaper-v0.1.md
├─ core/              # núcleo em Rust (DAG + PoUW + P2P)
│  ├─ Cargo.toml
│  └─ src/
│     ├─ main.rs
│     ├─ dag.rs
│     ├─ pouw.rs
│     └─ net.rs
├─ sdk/               # SDK TypeScript para dApps
│  ├─ package.json
│  ├─ tsconfig.json
│  └─ src/index.ts
├─ wallet/            # (placeholder) carteira web/mobile
├─ infra/             # docker-compose e configs
│  └─ docker-compose.yml
└─ tests/             # testes de caos/integração
```

## 🚀 Como começar (SDK)
```bash
cd sdk
npm i
npm run build
```

## 🔧 Como começar (Core – Rust)
```bash
cd core
cargo run
```

## 📈 Roadmap (resumo)
- MVP (6 meses): DAG + PoUW + PQC + SDK + testnet pública.
- Fase 2: wallet, indexação avançada, airdrop de devs.
- Fase 3: evolução para PoVU (stake + utilidade verificável).