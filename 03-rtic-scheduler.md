# Projet 03 — Scheduler temps réel avec RTIC (Real-Time Interrupt-driven Concurrency)

## Contexte

Tu implémentes un firmware multitâche temps réel en utilisant le framework **RTIC v2** pour Rust.
RTIC exploite le modèle de priorité matérielle du Cortex-M (NVIC) pour garantir l'ordonnancement sans données races,
sans besoin d'un OS complet.  
Ce projet démontre la maîtrise de la concurrence embarquée sûre, un prérequis dans l'industrie (automobile, médical, robotique).

**Cible matérielle** : STM32F411 (Nucleo-F411RE) — ARM Cortex-M4  
**Framework** : `rtic 2.x` + `rtic-monotonics`  
**Environnement** : `#![no_std]` `#![no_main]`

---

## Objectifs du projet

1. Implémenter un **scheduler RTIC** avec au moins 4 tâches de priorités différentes
2. Gérer des **ressources partagées** entre tâches avec les garanties d'accès RTIC (`lock`)
3. Implémenter des **tâches software** (spawnable) et des **tâches hardware** (interruptions)
4. Implémenter un **timer monotonique** pour les délais et timeouts
5. Implémenter un système de **messagerie inter-tâches** via une queue lock-free
6. Mesurer et afficher la **charge CPU** de chaque tâche via DWT cycle counter

---

## Spécifications techniques

### Architecture des tâches

| Tâche | Type | Priorité | Période | Rôle |
|---|---|---|---|---|
| `sensor_read` | hardware (TIM2) | 4 (haute) | 10 ms | Lecture ADC capteur |
| `filter_process` | software | 3 | déclenchée | Filtre médian sur buffer |
| `uart_send` | software | 2 | déclenchée | Envoi données formatées |
| `heartbeat` | software | 1 (basse) | 1 000 ms | Blink LED watchdog |
| `idle` | idle task | 0 | continu | Mesure charge CPU |

### Ressources partagées

```rust
#[shared]
struct Shared {
    sensor_buffer: [u16; 32],   // protégé par RTIC lock
    uart: Usart2,               // accès exclusif
    cpu_stats: CpuStats,        // accumulateur de charge
}

#[local]
struct Local {
    led: PA5<Output<PushPull>>,
    tim2: TIM2,
    filter_state: MedianFilter,
}
```

### API messagerie inter-tâches

```rust
// Queue statique lock-free (heapless)
static SENSOR_QUEUE: Queue<SensorReading, 16> = Queue::new();

#[derive(defmt::Format)]
pub struct SensorReading {
    pub timestamp_ms: u32,
    pub raw_value: u16,
    pub filtered_value: u16,
}
```

### Mesure charge CPU

```rust
// Dans idle : compter les cycles passés en idle vs total
fn measure_cpu_load(dwt: &mut DWT) -> u8 {
    // Retourne charge en % (0-100)
}
```

---

## Livrables attendus

- [ ] Application RTIC complète avec les 4 tâches opérationnelles
- [ ] Ressources partagées sans data race (vérifiées par le compilateur)
- [ ] Timer monotonique fonctionnel avec `rtic-monotonics`
- [ ] Queue inter-tâches avec `heapless::spsc::Queue`
- [ ] Mesure CPU affichée via `defmt` toutes les secondes
- [ ] Diagramme de séquence des tâches (ASCII ou Mermaid) dans le README
- [ ] Bench : démontrer que `sensor_read` à priorité haute ne bloque jamais > 50 µs

---

## Critères de qualité

- Zéro `Mutex` de std, zéro `RefCell` — uniquement primitives RTIC
- `cargo clippy -- -D warnings` : zéro warning
- Période de 10 ms respectée (vérifiable à l'oscilloscope ou via DWT)
- Le code compile avec `--release` et `--target thumbv7em-none-eabihf`
- README avec explication du modèle de priorité RTIC utilisé

---

## Ressources RTIC

- Documentation officielle : https://rtic.rs/2/book/en/
- `rtic-monotonics` pour les delays : https://docs.rs/rtic-monotonics
- `heapless` pour les structures de données no_std : https://docs.rs/heapless
