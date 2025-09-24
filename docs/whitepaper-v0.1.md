# TEQLA — Whitepaper v0.1

## 1. Visão
TEQLA é uma L1 *feeless*, pós‑quântica, baseada em DAG + Proof‑of‑Useful‑Work (PoUW).  
Cada transação referencia 2–4 transações anteriores (*tips*) e inclui uma prova de trabalho útil determinística e verificável.  
Finalidade rápida via checkpoints aleatórios (VRF), sem validadores fixos.

## 2. Motivação
- **Taxas**: inviabilizam micropagamentos.
- **Centralização**: validadores/PoS tendem a concentrar poder.
- **Quântico**: ECDSA/BLS vulneráveis ao algoritmo de Shor.
- **Utilidade**: trabalho de consenso geralmente não produz valor externo.

## 3. Arquitetura (MVP)
### 3.1 DAG
- Cada tx referencia `k∈[2,4]` *tips*.
- Regras de conflito: `nonce` monotônico por conta; empate por **maior peso cumulativo**, depois menor hash.

### 3.2 PoUW (simples, determinístico)
- **DataPool**: conjunto público de pequenos *blobs* (ex. slices de datasets).
- **Tarefa**: `pouw = SHA3(blob_slice || nonce || txid_suffix)` com *target* de dificuldade.
- **Objetivo**: tempo mediano ~150–300 ms em mobile (anti‑spam elástico).
- **Verificação**: recomputação SHA3 — O(1) no tamanho do *slice* (<1 KiB).

### 3.3 Checkpoints (VRF)
- A cada `T_epoch` (~8 s), nodes calculam VRF sobre *seed* do DAG. Saídas abaixo do limiar publicam **snapshot hash**.  
- Checkpoints *ancoram* a finalidade (reduzem prob. de rollback).

### 3.4 Criptografia Pós‑Quântica
- **Dilithium‑II** para contas quentes; **SPHINCS+** para cofres.  
- *Bundling* de assinaturas para reduzir overhead de rede.

## 4. Developer Experience
- **Compatibilidade EVM** (runtime dedicado) no SDK.
- **SDK (TS)**: `sendTx`, `awaitFinality`, `query`.
- **Indexação**: indexer básico com API (REST/GraphQL).

## 5. Métricas‑alvo (testnet)
- 1k–5k TPS em laboratório.
- Finalidade p95 ≤ 6–8 s.
- PoUW p95 em mobile ≤ 300 ms.

## 6. Evolução para PoVU
- Introdução de seleção por VRF + tarefas úteis mais ricas (com provas).
- Bônus de utilidade; mercado de tarefas.
- Mantendo a compatibilidade com o DAG base.

## 7. Segurança e ameaças
- **Spam**: mitigado por dificuldade elástica + peer scoring.
- **Censura**: *tips* aleatórios + ausência de líderes fixos.
- **Partições**: reconciliação por peso + checkpoints.

## 8. Licença
MIT.