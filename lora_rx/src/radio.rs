use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi::{Async, Spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Delay;
use lora_phy::iv::GenericSx127xInterfaceVariant;
use lora_phy::{sx127x, LoRa, RxMode};
use lora_phy::mod_params::{Bandwidth, CodingRate, PacketParams, PacketStatus, RadioError, SpreadingFactor};
use lora_phy::sx127x::{Sx1276, Sx127x};
use crate::common::Spi1Bus;

const LORA_FREQUENCY: u32 = 915_000_000;
const PREAMBLE_LENGTH: u16 = 4;
const IMPLICIT_HEADER: bool = false;
const CRC_ON: bool = true;
const IQ_INVERTED: bool = false;
const OUTPUT_POWER: i32 = 20;
const SPREADING_FACTOR: SpreadingFactor = SpreadingFactor::_10;
const BANDWIDTH: Bandwidth = Bandwidth::_250KHz;
const CODING_RATE: CodingRate = CodingRate::_4_8;

pub struct Radio {
    lora: LoRa<Sx127x<SpiDevice<'static, NoopRawMutex, Spi<'static, SPI1, Async>, Output<'static>>, GenericSx127xInterfaceVariant<Output<'static>, Input<'static>>, Sx1276>, Delay>
}

impl Radio {
    pub(crate) async fn new(
        spi_bus: &'static Spi1Bus,
        chip_select: Output<'static>,
        reset: Output<'static>,
        dio0: Input<'static>
    ) -> Self {
        let spi_device = SpiDevice::new(spi_bus, chip_select);
        let config = sx127x::Config {
            chip: Sx1276,
            tcxo_used: false,
            tx_boost: false,
            rx_boost: false,
        };
        let iv = GenericSx127xInterfaceVariant::new(reset, dio0, None, None).unwrap();
        let lora = LoRa::new(Sx127x::new(spi_device, iv, config), true, Delay).await.unwrap();
        Radio { lora }
    }

    pub async fn init_rx(&mut self, buffer_len: u8) -> Result<(PacketParams), RadioError> {
        let mod_params = self.lora.create_modulation_params(SPREADING_FACTOR, BANDWIDTH, CODING_RATE, LORA_FREQUENCY)?;
        let packet_params = self.lora.create_rx_packet_params(PREAMBLE_LENGTH, IMPLICIT_HEADER, buffer_len, CRC_ON, IQ_INVERTED, &mod_params)?;
        self.lora.prepare_for_rx(RxMode::Continuous, &mod_params, &packet_params).await?;
        Ok(packet_params)
    }

    pub async fn rx(
        &mut self,
        packet_params: &PacketParams,
        buffer: &mut [u8]
    ) -> Result<(u8, PacketStatus), RadioError> {
        self.lora.rx(&packet_params, buffer).await
    }
}