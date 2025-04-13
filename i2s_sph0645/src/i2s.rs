use embassy_rp::dma::{AnyChannel, Channel, Transfer};
use embassy_rp::{into_ref, Peripheral, PeripheralRef};
use embassy_rp::pio::{Common, Config, Direction, FifoJoin, Instance, LoadedProgram, PioPin, ShiftConfig, ShiftDirection, StateMachine};
use embassy_rp::pio::program::pio_asm;
use fixed_macro::__fixed::FixedU32;
use fixed_macro::__fixed::prelude::ToFixed;
use fixed_macro::__fixed::types::extra::U8;

// 18 bits of data, 6 bits of 0s, 8 bits of ?
const BITS_PER_SAMPLE: u32 = 32;
const CHANNELS: u32 = 2;
const CLOCK_CYCLES_PER_BIT: u32 = 2;
const SAMPLE_RATE_HZ: u32 = 44_100;

// This struct represents an i2s output driver program
pub struct PioI2sInProgram<'a, PIO: Instance> {
    prg: LoadedProgram<'a, PIO>,
}

impl<'a, PIO: Instance> PioI2sInProgram<'a, PIO> {
    pub fn new(common: &mut Common<'a, PIO>) -> Self {
        let prg = pio_asm!(
            ".side_set 2",
            ".wrap_target",
            "    set x, 30             side 0b01", // side 0bWC - W = WS, C = SCK
            "left_channel:", // active mic (mono)
            "    in pins, 1            side 0b00",
            "    jmp x-- left_channel  side 0b01", // jump if nonzero and decrement
            "    in pins, 1            side 0b10",
            "    set x, 30             side 0b11",
            "right_channel:", // if using two mics (stereo), adjust this block
            "    nop                   side 0b10",
            "    jmp x-- right_channel side 0b11",
            "    nop                   side 0b00",
            ".wrap"
        );
        Self { prg: common.load_program(&prg.program) }
    }
}

// PIO backed I2s input driver
pub struct PioI2sIn<'a, P: Instance, const S: usize> {
    dma: PeripheralRef<'a, AnyChannel>,
    sm: StateMachine<'a, P, S>,
}

impl<'a, P: Instance, const S: usize> PioI2sIn<'a, P, S> {
    // configure a state machine to input I2s
    pub fn new(
        common: &mut Common<'a, P>,
        mut sm: StateMachine<'a, P, S>,
        dma: impl Peripheral<P = impl Channel> + 'a,
        sd_pin: impl PioPin,
        sck_pin: impl PioPin,
        ws_pin: impl PioPin,
        program: &PioI2sInProgram<'a, P>,
    ) -> Self {
        into_ref!(dma);

        let sd_pin = common.make_pio_pin(sd_pin);
        let sck_pin = common.make_pio_pin(sck_pin);
        let ws_pin = common.make_pio_pin(ws_pin);

        let cfg = {
            let mut cfg = Config::default();
            cfg.use_program(&program.prg, &[&sck_pin, &ws_pin]);
            cfg.set_in_pins(&[&sd_pin]);
            cfg.clock_divider = Self::pio_clock_divider();
            cfg.shift_in = ShiftConfig {
                threshold: 32,
                direction: ShiftDirection::Left,
                auto_fill: true,
            };
            cfg.fifo_join = FifoJoin::RxOnly;
            cfg
        };
        sm.set_config(&cfg);
        sm.set_pin_dirs(Direction::In, &[&sd_pin]);
        sm.set_pin_dirs(Direction::Out, &[&sck_pin, &ws_pin]);
        sm.set_enable(true);

        Self { dma: dma.map_into(), sm }
    }

    fn pio_clock_divider() -> FixedU32<U8> {
        (embassy_rp::clocks::clk_sys_freq() as f64 / Self::pio_clock_frequency()).to_fixed()
    }

    fn pio_clock_frequency() -> f64 {
        // 2.048 MHz <= x <= 4.096 MHz
        (SAMPLE_RATE_HZ * CHANNELS * CLOCK_CYCLES_PER_BIT * BITS_PER_SAMPLE) as f64
    }

    pub fn read<'b>(&'b mut self, buff: &'b mut [u32]) -> Transfer<'b, AnyChannel> {
        self.sm.rx().dma_pull(self.dma.reborrow(), buff, false)
    }
}