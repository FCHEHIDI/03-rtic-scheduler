#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use rtic_monotonics::systick::prelude::*;
systick_monotonic!(Mono, 1000); // tick = 1 ms

/// Une lecture capteur horodatée, envoyée via la queue inter-tâches.
#[derive(defmt::Format, Clone, Copy)]
pub struct SensorReading {
    pub timestamp_ms: u32,
    pub raw_value: u16,
    pub filtered_value: u16,
}

/// Accumulateur de charge CPU calculé dans idle.
#[derive(defmt::Format, Clone, Copy, Default)]
pub struct CpuStats {
    pub idle_cycles: u32,
    pub total_cycles: u32,
}

impl CpuStats {
    pub fn load_percent(&self) -> u8 {
        if self.total_cycles == 0 {
            return 0;
        }
        let busy = self.total_cycles.saturating_sub(self.idle_cycles);
        ((busy as u64 * 100) / self.total_cycles as u64) as u8
    }
}

#[rtic::app(
    device = stm32f4xx_hal::pac,
    peripherals = true,
    dispatchers = [EXTI0, EXTI1, EXTI2]
)]
mod app {
    use super::{CpuStats, SensorReading};
    use heapless::spsc::{Consumer, Producer, Queue};
    use stm32f4xx_hal::{
        gpio::{Output, PushPull, PA5},
        pac::USART2,
        serial::Tx,
    };
    use stm32f4xx_hal::timer::{CounterMs, Event,};
    use stm32f4xx_hal::prelude::*;
    use rtic_monotonics::systick::prelude::*;
    use crate::Mono;
    use rtic::Mutex;

    // -----------------------------------------------------------------------
    // Ressources partagées : accédées depuis plusieurs tâches → lock RTIC
    // -----------------------------------------------------------------------
    #[shared]
    struct Shared {
        sensor_buffer: [u16; 32], // dernier buffer de mesures brutes
        uart: Tx<USART2>,         // accès exclusif à l'UART
        cpu_stats: CpuStats,      // accumulateur idle/total — mis à jour par idle
    }

    // -----------------------------------------------------------------------
    // Ressources locales : appartiennent à une seule tâche → pas de lock
    // -----------------------------------------------------------------------
    #[local]
    struct Local {
        led: PA5<Output<PushPull>>, // LED LD2 — heartbeat
        tim2: CounterMs<stm32f4xx_hal::pac::TIM2>,                 // registre TIM2 pour clear le flag
        sensor_producer: Producer<'static, SensorReading, 16>,
        sensor_consumer: Consumer<'static, SensorReading, 16>,
    }

    // -----------------------------------------------------------------------
    // Init — configure les périphériques et retourne les ressources
    // -----------------------------------------------------------------------
    #[init(local = [q: Queue<SensorReading, 16> = Queue::new()])]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        defmt::info!("--- rtic-scheduler boot ---");

        // La queue est splitée une seule fois ici, en lieu sûr (pas d'IRQ active).
        let (producer, consumer) = cx.local.q.split();

        // Configurer les horloges (RCC)
        let rcc = cx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

        // LED PA5 (LD2)
        let gpioa = cx.device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        // USART2 sur PA2(TX)/PA3(RX) à 115_200 baud
        let tx_pin = gpioa.pa2.into_alternate();
        let serial = cx.device.USART2.tx(tx_pin, 115_200.bps(), &clocks).unwrap();

        // TIM2 -- interruption toutes les 10 ms
        let mut tim2 = cx.device.TIM2.counter_ms(&clocks);
        tim2.start(10_u32.millis()).unwrap();
        tim2.listen(Event::Update);

        // DWT (cycle counter) -- pour la mesure CPU plus tard

        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        // Initialiser la monotonique pour les délais dans les tâches async
        Mono::start(cx.core.SYST, 84_000_000); 

        // Spawner [heartbeat] la tâche doit se lancer au boot
        heartbeat::spawn().unwrap();

        async fn heartbeat(mut cx: heartbeat::Context<'_>) {
            loop {
                cx.local.led.toggle();
                cx.shared.cpu_stats.lock(|stats| {
                    defmt::info!("heartbeat | CPU load: {}%", stats.load_percent());
                });

                Mono::delay(1000.millis()).await;
            }
        }

        (
            Shared {
                sensor_buffer: [0; 32],
                uart: serial,   
                cpu_stats: CpuStats::default(),
            },
            Local {
                led: led,
                tim2: tim2,
                sensor_producer: producer,
                sensor_consumer: consumer,
            },
        )
    }

    // -----------------------------------------------------------------------
    // idle — priorité 0, tourne en continu quand aucune tâche n'est prête
    // -----------------------------------------------------------------------
    #[idle(shared = [cpu_stats])]
    fn idle(mut cx: idle::Context) -> ! {
        loop {
            // TODO étape 6 : mesure DWT
            cx.shared.cpu_stats.lock(|_stats| {
                // placeholder
            
            });
            cortex_m::asm::nop();
        }
    }

    // -----------------------------------------------------------------------
    // sensor_read — tâche hardware déclenchée par TIM2 (priorité 4)
    // -----------------------------------------------------------------------
    #[task(
        binds = TIM2,
        priority = 4,
        shared = [sensor_buffer],
        local  = [tim2, sensor_producer]
    )]
    fn sensor_read(mut cx: sensor_read::Context) {
        // workflow: lire ADC, pusher dans la queue, spawner filter_process
        // 1. Clear le flag update de TIM2 -- OBLIGATOIRE sinon IRQ infinie
        cx.local.tim2.wait().ok(); // clear le flag Update, ignore Err(WouldBlock)

        // 2. Récupérer timestamp (monotonique) pour la lecture
        let ts = Mono::now().ticks();
        
        // 3. Valeur simulée
        let raw = 42u16;

        // 4. Ecrire dans le buffer partagé
        cx.shared.sensor_buffer.lock(|buf| {
            buf[0] = raw;
        });

        // 5. Push la lecture dans la queue inter-tâches
        let reading = SensorReading {
            timestamp_ms: ts as u32,
            raw_value: raw,
            filtered_value: raw, // sera filtré par filter_process
        };

        if cx.local.sensor_producer.enqueue(reading).is_err() {
            defmt::warn!("sensor queue full!");
        }

        // 6. Déclencher filter_process
        filter_process::spawn().ok();
    }

    // -----------------------------------------------------------------------
    // filter_process — tâche software (priorité 3)
    // -----------------------------------------------------------------------
    #[task(
        priority = 3,
        shared = [sensor_buffer],
        local  = [sensor_consumer]
    )]
    async fn filter_process(mut cx: filter_process::Context) {
        // TODO étape 5 : filtre médian, spawner uart_send
        cx.shared.sensor_buffer.lock(|_buf| {
            // placeholder
        });
    }

    // -----------------------------------------------------------------------
    // uart_send — tâche software (priorité 2)
    // -----------------------------------------------------------------------
    #[task(
        priority = 2,
        shared = [uart]
    )]
    async fn uart_send(mut cx: uart_send::Context, reading: SensorReading) {
        // TODO étape 5 : formater et envoyer via USART2
        cx.shared.uart.lock(|_uart| {
            defmt::info!("uart_send placeholder: {}", reading);
        });
    }

    // -----------------------------------------------------------------------
    // heartbeat — tâche software (priorité 1)
    // -----------------------------------------------------------------------
    #[task(
        priority = 1,
        shared = [cpu_stats],
        local  = [led]
    )]
    async fn heartbeat(mut cx: heartbeat::Context<'_>) {
        // TODO étape 4 : toggle LED, attendre 1 s via monotonique, logguer charge CPU
        cx.shared.cpu_stats.lock(|stats| {
            defmt::info!("CPU load: {}%", stats.load_percent());
        });
    }
}