#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
    r" Always include the device crate which contains the vector table"] use
    stm32f4xx_hal :: pac as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    #[doc =
    r" Holds the maximum priority level for use by async HAL drivers."]
    #[no_mangle] static RTIC_ASYNC_MAX_LOGICAL_PRIO : u8 = 3u8; use super ::
    { CpuStats, SensorReading }; use heapless :: spsc ::
    { Consumer, Producer, Queue }; use stm32f4xx_hal ::
    { gpio :: { Output, PushPull, PA5 }, pac :: USART2, serial :: Tx, }; use
    stm32f4xx_hal :: timer :: { CounterMs, Event, }; use stm32f4xx_hal ::
    prelude :: * ; use rtic_monotonics :: systick :: prelude :: * ; use crate
    :: Mono; #[doc = r" User code end"] impl < 'a >
    __rtic_internal_initLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_initLocalResources
            {
                q : & mut * __rtic_internal_local_init_q.get_mut(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } #[doc = r"Shared resources"] struct Shared
    {
        sensor_buffer : [u16; 32], uart : Tx < USART2 > , cpu_stats :
        CpuStats,
    } #[doc = r"Local resources"] struct Local
    {
        led : PA5 < Output < PushPull > > , tim2 : CounterMs < stm32f4xx_hal
        :: pac :: TIM2 > , sensor_producer : Producer < 'static,
        SensorReading, 16 > , sensor_consumer : Consumer < 'static,
        SensorReading, 16 > ,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `init` has access to"] pub struct
    __rtic_internal_initLocalResources < 'a >
    {
        #[allow(missing_docs)] pub q : & 'static mut Queue < SensorReading, 16
        > , #[doc(hidden)] pub __rtic_internal_marker : :: core :: marker ::
        PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_init_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > ,
        #[doc = r" The space used to allocate async executors in bytes."] pub
        executors_size : usize, #[doc = r" Core peripherals"] pub core : rtic
        :: export :: Peripherals, #[doc = r" Device peripherals (PAC)"] pub
        device : stm32f4xx_hal :: pac :: Peripherals,
        #[doc = r" Critical section token for init"] pub cs : rtic :: export
        :: CriticalSection < 'a > ,
        #[doc = r" Local Resources this task has access to"] pub local : init
        :: LocalResources < 'a > ,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn
        new(core : rtic :: export :: Peripherals, executors_size : usize) ->
        Self
        {
            __rtic_internal_init_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, core :
                core, device : stm32f4xx_hal :: pac :: Peripherals :: steal(),
                cs : rtic :: export :: CriticalSection :: new(),
                executors_size, local : init :: LocalResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[doc(inline)] pub use super :: __rtic_internal_initLocalResources as
        LocalResources; #[doc(inline)] pub use super ::
        __rtic_internal_init_Context as Context;
    } #[inline(always)] #[allow(non_snake_case)] fn
    init(mut cx : init :: Context) -> (Shared, Local)
    {
        defmt :: info! ("--- rtic-scheduler boot ---"); let
        (producer, consumer) = cx.local.q.split(); let rcc =
        cx.device.RCC.constrain(); let clocks =
        rcc.cfgr.sysclk(84.MHz()).freeze(); let gpioa =
        cx.device.GPIOA.split(); let led = gpioa.pa5.into_push_pull_output();
        let tx_pin = gpioa.pa2.into_alternate(); let serial =
        cx.device.USART2.tx(tx_pin, 115_200.bps(), & clocks).unwrap(); let mut
        tim2 = cx.device.TIM2.counter_ms(& clocks);
        tim2.start(10_u32.millis()).unwrap(); tim2.listen(Event :: Update);
        cx.core.DCB.enable_trace(); cx.core.DWT.enable_cycle_counter(); Mono
        :: start(cx.core.SYST, 84_000_000); heartbeat :: spawn().unwrap();
        (Shared
        {
            sensor_buffer : [0; 32], uart : serial, cpu_stats : CpuStats ::
            default(),
        }, Local
        {
            led : led, tim2 : tim2, sensor_producer : producer,
            sensor_consumer : consumer,
        },)
    } impl < 'a > __rtic_internal_idleSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_idleSharedResources
            {
                cpu_stats : shared_resources ::
                cpu_stats_that_needs_to_be_locked :: new(),
                __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `idle` has access to"] pub struct
    __rtic_internal_idleSharedResources < 'a >
    {
        #[allow(missing_docs)] pub cpu_stats : shared_resources ::
        cpu_stats_that_needs_to_be_locked < 'a > , #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_idle_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Shared Resources this task has access to"] pub
        shared : idle :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_idle_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_idle_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, shared :
                idle :: SharedResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
    {
        #[doc(inline)] pub use super :: __rtic_internal_idleSharedResources as
        SharedResources; #[doc(inline)] pub use super ::
        __rtic_internal_idle_Context as Context;
    } #[allow(non_snake_case)] fn idle(mut cx : idle :: Context) -> !
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; let mut
        last_measurement = cortex_m :: peripheral :: DWT :: cycle_count();
        loop
        {
            let idle_start = cortex_m :: peripheral :: DWT :: cycle_count();
            cortex_m :: asm :: nop(); let idle_end = cortex_m :: peripheral ::
            DWT :: cycle_count(); let idle_cycles =
            idle_end.wrapping_sub(idle_start);
            cx.shared.cpu_stats.lock(| stats |
            {
                stats.idle_cycles =
                stats.idle_cycles.wrapping_add(idle_cycles);
                stats.total_cycles =
                stats.total_cycles.wrapping_add(idle_end.wrapping_sub(last_measurement));
            }); last_measurement = idle_end;
        }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn TIM2()
    {
        const PRIORITY : u8 = 4u8; rtic :: export ::
        run(PRIORITY, || { sensor_read(sensor_read :: Context :: new()) });
    } impl < 'a > __rtic_internal_sensor_readLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_sensor_readLocalResources
            {
                tim2 : & mut *
                (& mut *
                __rtic_internal_local_resource_tim2.get_mut()).as_mut_ptr(),
                sensor_producer : & mut *
                (& mut *
                __rtic_internal_local_resource_sensor_producer.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_sensor_readSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_sensor_readSharedResources
            {
                sensor_buffer : shared_resources ::
                sensor_buffer_that_needs_to_be_locked :: new(),
                __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `sensor_read` has access to"] pub struct
    __rtic_internal_sensor_readLocalResources < 'a >
    {
        #[allow(missing_docs)] pub tim2 : & 'a mut CounterMs < stm32f4xx_hal
        :: pac :: TIM2 > , #[allow(missing_docs)] pub sensor_producer : & 'a
        mut Producer < 'static, SensorReading, 16 > , #[doc(hidden)] pub
        __rtic_internal_marker : :: core :: marker :: PhantomData < & 'a () >
        ,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `sensor_read` has access to"] pub struct
    __rtic_internal_sensor_readSharedResources < 'a >
    {
        #[allow(missing_docs)] pub sensor_buffer : shared_resources ::
        sensor_buffer_that_needs_to_be_locked < 'a > , #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_sensor_read_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Local Resources this task has access to"] pub
        local : sensor_read :: LocalResources < 'a > ,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        sensor_read :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_sensor_read_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_sensor_read_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                sensor_read :: LocalResources :: new(), shared : sensor_read
                :: SharedResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod sensor_read
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_sensor_readLocalResources as LocalResources;
        #[doc(inline)] pub use super ::
        __rtic_internal_sensor_readSharedResources as SharedResources;
        #[doc(inline)] pub use super :: __rtic_internal_sensor_read_Context as
        Context;
    } #[allow(non_snake_case)] fn sensor_read(mut cx : sensor_read :: Context)
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ;
        cx.local.tim2.wait().ok(); let ts = Mono :: now().ticks(); let raw =
        42u16; cx.shared.sensor_buffer.lock(| buf | { buf [0] = raw; }); let
        reading = SensorReading
        { timestamp_ms : ts as u32, raw_value : raw, filtered_value : raw, };
        if cx.local.sensor_producer.enqueue(reading).is_err()
        { defmt :: warn! ("sensor queue full!"); } filter_process ::
        spawn().ok();
    } impl < 'a > __rtic_internal_filter_processLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_filter_processLocalResources
            {
                sensor_consumer : & mut *
                (& mut *
                __rtic_internal_local_resource_sensor_consumer.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_filter_processSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_filter_processSharedResources
            {
                sensor_buffer : shared_resources ::
                sensor_buffer_that_needs_to_be_locked :: new(),
                __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_uart_sendSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_uart_sendSharedResources
            {
                uart : shared_resources :: uart_that_needs_to_be_locked ::
                new(), __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_heartbeatLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_heartbeatLocalResources
            {
                led : & mut *
                (& mut *
                __rtic_internal_local_resource_led.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_heartbeatSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_heartbeatSharedResources
            {
                cpu_stats : shared_resources ::
                cpu_stats_that_needs_to_be_locked :: new(),
                __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `filter_process` has access to"] pub struct
    __rtic_internal_filter_processLocalResources < 'a >
    {
        #[allow(missing_docs)] pub sensor_consumer : & 'a mut Consumer <
        'static, SensorReading, 16 > , #[doc(hidden)] pub
        __rtic_internal_marker : :: core :: marker :: PhantomData < & 'a () >
        ,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `filter_process` has access to"] pub struct
    __rtic_internal_filter_processSharedResources < 'a >
    {
        #[allow(missing_docs)] pub sensor_buffer : shared_resources ::
        sensor_buffer_that_needs_to_be_locked < 'a > , #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_filter_process_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Local Resources this task has access to"] pub
        local : filter_process :: LocalResources < 'a > ,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        filter_process :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_filter_process_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_filter_process_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                filter_process :: LocalResources :: new(), shared :
                filter_process :: SharedResources :: new(),
            }
        }
    } #[doc = r" Spawns the task directly"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_filter_process_spawn() -> :: core ::
    result :: Result < (), () >
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(filter_process, &
            __rtic_internal_filter_process_EXEC); if exec.try_allocate()
            {
                exec.spawn(filter_process(unsafe
                { filter_process :: Context :: new() })); rtic :: export ::
                pend(stm32f4xx_hal :: pac :: interrupt :: EXTI2); Ok(())
            } else { Err(()) }
        }
    } #[doc = r" Gives waker to the task"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_filter_process_waker() -> :: core ::
    task :: Waker
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(filter_process, &
            __rtic_internal_filter_process_EXEC);
            exec.waker(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_1_args(filter_process, &
                __rtic_internal_filter_process_EXEC); exec.set_pending(); rtic
                :: export :: pend(stm32f4xx_hal :: pac :: interrupt :: EXTI2);
            })
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod filter_process
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_filter_processLocalResources as LocalResources;
        #[doc(inline)] pub use super ::
        __rtic_internal_filter_processSharedResources as SharedResources;
        #[doc(inline)] pub use super :: __rtic_internal_filter_process_Context
        as Context; #[doc(inline)] pub use super ::
        __rtic_internal_filter_process_spawn as spawn; #[doc(inline)] pub use
        super :: __rtic_internal_filter_process_waker as waker;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `uart_send` has access to"] pub struct
    __rtic_internal_uart_sendSharedResources < 'a >
    {
        #[allow(missing_docs)] pub uart : shared_resources ::
        uart_that_needs_to_be_locked < 'a > , #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_uart_send_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Shared Resources this task has access to"] pub
        shared : uart_send :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_uart_send_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_uart_send_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, shared :
                uart_send :: SharedResources :: new(),
            }
        }
    } #[doc = r" Spawns the task directly"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_uart_send_spawn(_0 : SensorReading,)
    -> :: core :: result :: Result < (), SensorReading >
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_2_args(uart_send, & __rtic_internal_uart_send_EXEC); if
            exec.try_allocate()
            {
                exec.spawn(uart_send(unsafe { uart_send :: Context :: new() },
                _0)); rtic :: export ::
                pend(stm32f4xx_hal :: pac :: interrupt :: EXTI1); Ok(())
            } else { Err(_0) }
        }
    } #[doc = r" Gives waker to the task"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_uart_send_waker() -> :: core :: task
    :: Waker
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_2_args(uart_send, & __rtic_internal_uart_send_EXEC);
            exec.waker(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_2_args(uart_send, & __rtic_internal_uart_send_EXEC);
                exec.set_pending(); rtic :: export ::
                pend(stm32f4xx_hal :: pac :: interrupt :: EXTI1);
            })
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod uart_send
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_uart_sendSharedResources as SharedResources;
        #[doc(inline)] pub use super :: __rtic_internal_uart_send_Context as
        Context; #[doc(inline)] pub use super ::
        __rtic_internal_uart_send_spawn as spawn; #[doc(inline)] pub use super
        :: __rtic_internal_uart_send_waker as waker;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `heartbeat` has access to"] pub struct
    __rtic_internal_heartbeatLocalResources < 'a >
    {
        #[allow(missing_docs)] pub led : & 'a mut PA5 < Output < PushPull > >
        , #[doc(hidden)] pub __rtic_internal_marker : :: core :: marker ::
        PhantomData < & 'a () > ,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `heartbeat` has access to"] pub struct
    __rtic_internal_heartbeatSharedResources < 'a >
    {
        #[allow(missing_docs)] pub cpu_stats : shared_resources ::
        cpu_stats_that_needs_to_be_locked < 'a > , #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_heartbeat_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Local Resources this task has access to"] pub
        local : heartbeat :: LocalResources < 'a > ,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        heartbeat :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_heartbeat_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_heartbeat_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                heartbeat :: LocalResources :: new(), shared : heartbeat ::
                SharedResources :: new(),
            }
        }
    } #[doc = r" Spawns the task directly"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_heartbeat_spawn() -> :: core ::
    result :: Result < (), () >
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(heartbeat, & __rtic_internal_heartbeat_EXEC); if
            exec.try_allocate()
            {
                exec.spawn(heartbeat(unsafe
                { heartbeat :: Context :: new() })); rtic :: export ::
                pend(stm32f4xx_hal :: pac :: interrupt :: EXTI0); Ok(())
            } else { Err(()) }
        }
    } #[doc = r" Gives waker to the task"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_heartbeat_waker() -> :: core :: task
    :: Waker
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(heartbeat, & __rtic_internal_heartbeat_EXEC);
            exec.waker(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_1_args(heartbeat, & __rtic_internal_heartbeat_EXEC);
                exec.set_pending(); rtic :: export ::
                pend(stm32f4xx_hal :: pac :: interrupt :: EXTI0);
            })
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod heartbeat
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_heartbeatLocalResources as LocalResources;
        #[doc(inline)] pub use super ::
        __rtic_internal_heartbeatSharedResources as SharedResources;
        #[doc(inline)] pub use super :: __rtic_internal_heartbeat_Context as
        Context; #[doc(inline)] pub use super ::
        __rtic_internal_heartbeat_spawn as spawn; #[doc(inline)] pub use super
        :: __rtic_internal_heartbeat_waker as waker;
    } #[allow(non_snake_case)] async fn filter_process < 'a >
    (mut cx : filter_process :: Context < 'a >)
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; if let
        Some(mut reading) = cx.local.sensor_consumer.dequeue()
        {
            let filtered =
            cx.shared.sensor_buffer.lock(| buf |
            {
                buf.rotate_right(1); buf [0] = reading.raw_value; let mut
                window = [buf [0], buf [1], buf [2]]; window.sort_unstable();
                window [1]
            }); reading.filtered_value = filtered; uart_send ::
            spawn(reading).ok();
        }
    } #[allow(non_snake_case)] async fn uart_send < 'a >
    (mut cx : uart_send :: Context < 'a > , reading : SensorReading)
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ;
        cx.shared.uart.lock(| uart |
        {
            use core :: fmt :: Write; writeln!
            (uart, "ts={}ms raw={} filt={}\r", reading.timestamp_ms,
            reading.raw_value, reading.filtered_value).ok(); defmt :: info!
            ("sent: ts={}ms raw={} filt={}", reading.timestamp_ms,
            reading.raw_value, reading.filtered_value);
        });
    } #[allow(non_snake_case)] async fn heartbeat < 'a >
    (mut cx : heartbeat :: Context < 'a >)
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; loop
        {
            cx.local.led.toggle();
            cx.shared.cpu_stats.lock(| stats |
            {
                defmt :: info!
                ("heartbeat | CPU load: {}%", stats.load_percent()); * stats =
                CpuStats :: default();
            }); Mono :: delay(1000.millis()).await;
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
    __rtic_internal_shared_resource_sensor_buffer : rtic :: RacyCell < core ::
    mem :: MaybeUninit < [u16; 32] >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: sensor_buffer_that_needs_to_be_locked < 'a >
    {
        type T = [u16; 32]; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut [u16; 32]) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 4u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_sensor_buffer.get_mut()
                as * mut _, CEILING, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS,
                f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_shared_resource_uart : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Tx < USART2 > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: uart_that_needs_to_be_locked < 'a >
    {
        type T = Tx < USART2 > ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut Tx < USART2 >) -> RTIC_INTERNAL_R)
        -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 2u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_uart.get_mut() as * mut
                _, CEILING, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic2"] static
    __rtic_internal_shared_resource_cpu_stats : rtic :: RacyCell < core :: mem
    :: MaybeUninit < CpuStats >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: cpu_stats_that_needs_to_be_locked < 'a >
    {
        type T = CpuStats; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut CpuStats) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_cpu_stats.get_mut() as *
                mut _, CEILING, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } mod shared_resources
    {
        #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        sensor_buffer_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () > , }
        impl < 'a > sensor_buffer_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                sensor_buffer_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        uart_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () > , }
        impl < 'a > uart_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                uart_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        cpu_stats_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () > , }
        impl < 'a > cpu_stats_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                cpu_stats_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic3"] static
    __rtic_internal_local_resource_led : rtic :: RacyCell < core :: mem ::
    MaybeUninit < PA5 < Output < PushPull > > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic4"] static
    __rtic_internal_local_resource_tim2 : rtic :: RacyCell < core :: mem ::
    MaybeUninit < CounterMs < stm32f4xx_hal :: pac :: TIM2 > >> = rtic ::
    RacyCell :: new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic5"] static
    __rtic_internal_local_resource_sensor_producer : rtic :: RacyCell < core
    :: mem :: MaybeUninit < Producer < 'static, SensorReading, 16 > >> = rtic
    :: RacyCell :: new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic6"] static
    __rtic_internal_local_resource_sensor_consumer : rtic :: RacyCell < core
    :: mem :: MaybeUninit < Consumer < 'static, SensorReading, 16 > >> = rtic
    :: RacyCell :: new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_local_init_q : rtic :: RacyCell <
    Queue < SensorReading, 16 > > = rtic :: RacyCell :: new(Queue :: new());
    #[allow(non_upper_case_globals)] static
    __rtic_internal_filter_process_EXEC : rtic :: export :: executor ::
    AsyncTaskExecutorPtr = rtic :: export :: executor :: AsyncTaskExecutorPtr
    :: new(); #[allow(non_upper_case_globals)] static
    __rtic_internal_uart_send_EXEC : rtic :: export :: executor ::
    AsyncTaskExecutorPtr = rtic :: export :: executor :: AsyncTaskExecutorPtr
    :: new(); #[allow(non_upper_case_globals)] static
    __rtic_internal_heartbeat_EXEC : rtic :: export :: executor ::
    AsyncTaskExecutorPtr = rtic :: export :: executor :: AsyncTaskExecutorPtr
    :: new(); #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch async tasks at priority 1"]
    #[no_mangle] unsafe fn EXTI0()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 1u8; rtic :: export ::
        run(PRIORITY, ||
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(heartbeat, & __rtic_internal_heartbeat_EXEC);
            exec.poll(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_1_args(heartbeat, & __rtic_internal_heartbeat_EXEC);
                exec.set_pending(); rtic :: export ::
                pend(stm32f4xx_hal :: pac :: interrupt :: EXTI0);
            });
        });
    } #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch async tasks at priority 2"]
    #[no_mangle] unsafe fn EXTI1()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 2u8; rtic :: export ::
        run(PRIORITY, ||
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_2_args(uart_send, & __rtic_internal_uart_send_EXEC);
            exec.poll(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_2_args(uart_send, & __rtic_internal_uart_send_EXEC);
                exec.set_pending(); rtic :: export ::
                pend(stm32f4xx_hal :: pac :: interrupt :: EXTI1);
            });
        });
    } #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch async tasks at priority 3"]
    #[no_mangle] unsafe fn EXTI2()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 3u8; rtic :: export ::
        run(PRIORITY, ||
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(filter_process, &
            __rtic_internal_filter_process_EXEC);
            exec.poll(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_1_args(filter_process, &
                __rtic_internal_filter_process_EXEC); exec.set_pending(); rtic
                :: export :: pend(stm32f4xx_hal :: pac :: interrupt :: EXTI2);
            });
        });
    } #[doc(hidden)] #[no_mangle] unsafe extern "C" fn main() -> !
    {
        rtic :: export :: assert_send :: < [u16; 32] > (); rtic :: export ::
        assert_send :: < Tx < USART2 > > (); rtic :: export :: assert_send ::
        < CpuStats > (); rtic :: export :: assert_send :: < SensorReading >
        (); rtic :: export :: interrupt :: disable(); let mut core : rtic ::
        export :: Peripherals = rtic :: export :: Peripherals ::
        steal().into(); let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: EXTI0; let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: EXTI1; let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: EXTI2; const _ : () = if
        (1 << stm32f4xx_hal :: pac :: NVIC_PRIO_BITS) < 1u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'EXTI0' is more than supported by hardware");
        };
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: EXTI0, rtic :: export ::
        cortex_logical2hw(1u8, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS),); rtic
        :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: EXTI0); const _ : () = if
        (1 << stm32f4xx_hal :: pac :: NVIC_PRIO_BITS) < 2u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'EXTI1' is more than supported by hardware");
        };
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: EXTI1, rtic :: export ::
        cortex_logical2hw(2u8, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS),); rtic
        :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: EXTI1); const _ : () = if
        (1 << stm32f4xx_hal :: pac :: NVIC_PRIO_BITS) < 3u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'EXTI2' is more than supported by hardware");
        };
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: EXTI2, rtic :: export ::
        cortex_logical2hw(3u8, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS),); rtic
        :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: EXTI2); const _ : () = if
        (1 << stm32f4xx_hal :: pac :: NVIC_PRIO_BITS) < 4u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'TIM2' is more than supported by hardware");
        };
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: TIM2, rtic :: export ::
        cortex_logical2hw(4u8, stm32f4xx_hal :: pac :: NVIC_PRIO_BITS),); rtic
        :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: TIM2); #[inline(never)] fn __rtic_init_resources < F >
        (f : F) where F : FnOnce() { f(); } let mut executors_size = 0; let
        executor = :: core :: mem :: ManuallyDrop ::
        new(rtic :: export :: executor :: AsyncTaskExecutor ::
        new_1_args(filter_process)); executors_size += :: core :: mem ::
        size_of_val(& executor);
        __rtic_internal_filter_process_EXEC.set_in_main(& executor); let
        executor = :: core :: mem :: ManuallyDrop ::
        new(rtic :: export :: executor :: AsyncTaskExecutor ::
        new_2_args(uart_send)); executors_size += :: core :: mem ::
        size_of_val(& executor);
        __rtic_internal_uart_send_EXEC.set_in_main(& executor); let executor =
        :: core :: mem :: ManuallyDrop ::
        new(rtic :: export :: executor :: AsyncTaskExecutor ::
        new_1_args(heartbeat)); executors_size += :: core :: mem ::
        size_of_val(& executor);
        __rtic_internal_heartbeat_EXEC.set_in_main(& executor); extern "C"
        { pub static _stack_start : u32; pub static __ebss : u32; } let
        stack_start = & _stack_start as * const _ as u32; let ebss = & __ebss
        as * const _ as u32; if stack_start > ebss
        {
            if rtic :: export :: msp :: read() <= ebss
            { panic! ("Stack overflow after allocating executors"); }
        }
        __rtic_init_resources(||
        {
            let (shared_resources, local_resources) =
            init(init :: Context :: new(core.into(), executors_size));
            __rtic_internal_shared_resource_sensor_buffer.get_mut().write(core
            :: mem :: MaybeUninit :: new(shared_resources.sensor_buffer));
            __rtic_internal_shared_resource_uart.get_mut().write(core :: mem
            :: MaybeUninit :: new(shared_resources.uart));
            __rtic_internal_shared_resource_cpu_stats.get_mut().write(core ::
            mem :: MaybeUninit :: new(shared_resources.cpu_stats));
            __rtic_internal_local_resource_led.get_mut().write(core :: mem ::
            MaybeUninit :: new(local_resources.led));
            __rtic_internal_local_resource_tim2.get_mut().write(core :: mem ::
            MaybeUninit :: new(local_resources.tim2));
            __rtic_internal_local_resource_sensor_producer.get_mut().write(core
            :: mem :: MaybeUninit :: new(local_resources.sensor_producer));
            __rtic_internal_local_resource_sensor_consumer.get_mut().write(core
            :: mem :: MaybeUninit :: new(local_resources.sensor_consumer));
            rtic :: export :: interrupt :: enable();
        }); idle(idle :: Context :: new())
    }
}