# Inventory, equipment e vault

O port Rust separa a camada de items em quatro blocos:

- `mu_gameplay::items`
  - codec do pacote legado de item com `group`, `number`, `level`,
    `durability`, flags opcionais e sockets;
  - `ItemOptionFlags` preserva os bits legacy `HasOption`, `HasLuck`,
    `HasSkill`, `HasExcellent`, `HasAncient`, `HasHarmony`, `HasGuardian` e
    `HasSockets`;
  - `ItemPacketData::encode()` e `ItemPacketData::decode()` falham em pacote
    curto, campo fora de faixa, excesso de sockets e bytes sobrando.
- `mu_gameplay::inventory`
  - inventory principal 8x8 mais 4 paginas de extensao 8x4;
  - `InventoryManager` cobre inserir, mover, stackar, usar e remover itens;
  - `InventorySlot` usa paginacao legacy, com a primeira pagina como inventory
    principal.
- `mu_gameplay::equipment`
  - 12 slots legacy de equipamento;
  - `EquipmentManager` aplica a escolha de slot principal e os atalhos para
    mao esquerda e anel esquerdo quando o slot preferido esta ocupado;
  - `ItemRequirements` valida level, stats e classe antes de equipar.
- `mu_gameplay::vault`
  - vault com 2 paginas de 8x15;
  - `VaultManager` controla lock, password verification, dinheiro e transfer
    pendente para o status 0/1/10/11/12/13;
  - `VaultMoneyDirection` e `VaultPendingTransfer` modelam o fluxo de sync.
- `mu_ui::inventory`
  - snapshot da surface gameplay usada para inventory, equip, store e vault
    access.
- `mu_ui::vault`
  - snapshot da storage dialog com estado de lock/pin, zen, toggle da view
    expandida e geometria das 2 paginas de vault.

O control plane local de `mu_app` tambem oferece `inventory-move` para mover
itens entre slots lineares do inventory e reenviar o movimento pelo helper
`item_move_request_extended`.

Uso esperado:

```rust
use mu_gameplay::{InventoryManager, Item, ItemPacketData, ItemSize};

let item = Item::stackable(ItemPacketData::new(1, 10), ItemSize::new(1, 1), 20, 5);
let mut inventory = InventoryManager::new();
let slot = inventory.add(item)?;
```

Os testes do slice cobrem:

- roundtrip do codec de item;
- falhas de sync capturadas por fake server;
- comportamento de inventory/equipment/vault com recursos Bevy;
- snapshot da tela `inventory`.
