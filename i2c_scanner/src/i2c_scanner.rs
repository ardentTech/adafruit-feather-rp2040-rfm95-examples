use embedded_hal::i2c::SevenBitAddress;
use embedded_hal_async::i2c::I2c;

pub struct I2cScanner<I2C> {
    i2c: I2C
}

impl<I2C: I2c<SevenBitAddress>> I2cScanner<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub async fn check(&mut self, addr: SevenBitAddress) -> bool {
        self.i2c.read(addr, &mut [0]).await.is_ok()
    }

    pub async fn scan(&mut self) -> [u8; 128] {
        let mut addrs = [0u8; 128];

        for i in 0..addrs.len() {
            match self.i2c.read(i as SevenBitAddress, &mut [0]).await {
                Ok(_) => addrs[i] = 1,
                Err(_) => {}
            }
        }

        addrs
    }
}

#[cfg(test)]
mod tests {

}