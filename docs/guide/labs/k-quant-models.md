---
feature: k-quant-models
key: labs.k_quant_models
title: Smaller model files
for: []
ask: Accept smaller, unverified Q6_K, Q5_K and Q4_K model files?
group: Models
os: macOS
vibehex: false
---

# Smaller model files

**What it does.** Add model file… normally takes only F32, F16 and Q8_0 files. With this on, it and Use model file where it is… also take files with Q6_K, Q5_K or Q4_K weights, which are smaller.

**Turn it on when.** You found a smaller quantized version of a speech model and want to try it, for example to save disk space or memory.

**Leave it off when.** You use the models from VibeHex's list. VibeHex hasn't checked these files, so accuracy may vary.

**Needs.** macOS. Windows and Linux can't run these files, so the row isn't there.

**Good to know.** If you turn the lab off while such a model is chosen, the choice stays saved but VibeHex dictates with the default model, and `hex status` says its Labs toggle is off. `hex model use` refuses such a model until the lab is on.

**Where.** Labs › Smaller model files (`labs.k_quant_models`). Related: General › Speech model › Add model file….

**Asked as.** "Can I use a Q4_K model?"; "Smaller GGUF speech model"; «Можно ли подключить модель Q4_K»; «Модель поменьше весом».

<!-- sources: src/labs/card.rs:52,211-213; src/labs/catalog.rs:257,477; src/labs/group.rs:149; src/control/settings/registry.rs:588-592; src/rune_ui/labs.rs:686-690; src/rune_ui/model_import.rs:55-85; docs/features/labs.md (Smaller model files section) -->
