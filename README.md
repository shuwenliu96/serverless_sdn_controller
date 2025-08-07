# serverless_sdn_controller

This repository accompanies the paper:

> **Functional Control: Leveraging Function-as-a-Service Platforms for Software-Defined Networking Controllers**  
> *Shuwen Liu, Craig Shue.*  
> *To appear in ACM MobiHoc 2025*  

---

# Overview

This work explores a new design paradigm for Software-Defined Networking (SDN) controllers by leveraging **Function-as-a-Service (FaaS)** platforms, such as serverless cloud or edge computing frameworks. We term this model **Functional Control**, in which traditional monolithic or distributed SDN controllers are refactored into a collection of stateless, event-driven functions.

These functions are deployed as independent, on-demand computation units that react to network events—such as topology changes, flow requests, or link failures—without relying on a continuously running control plane. We have deployed this controller on Fastly Compute@Edge and AWS Lambda.

---

## Deployment Instructions

### 🚀 Fastly Compute@Edge

1. **Install Fastly CLI and Rust**:
   ```bash
   brew install fastly/tap/fastly
   curl -sSf https://sh.rustup.rs | sh
   ```

2. **Initialize or use existing Fastly project**:
   ```bash
   fastly compute init
   ```

3. **Build and deploy to Fastly edge**:
   ```bash
   fastly compute build
   fastly compute deploy
   ```

> Rust-based FaaS logic is located under `rust_sdn_controller/src`.

---

## Citation

If you use this work, please cite:

```bibtex
@inproceedings{liu2025functional,
  author    = {Shuwen Liu and Craig Shue},
  title     = {Functional Control: Leveraging Function-as-a-Service Platforms for Software-Defined Networking Controllers},
  booktitle = {Proceedings of the 26th ACM International Symposium on Mobile Ad Hoc Networking and Computing (MobiHoc)},
  year      = {2025}
}
```

---
