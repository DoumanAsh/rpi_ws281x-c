#![no_std]

use core::{fmt, slice, mem, ptr};

pub use rpi_ws281x_sys as bindings;
pub use rpi_ws281x_sys::ws2811_return_t as ErrorCode;

#[repr(transparent)]
///Raspberry PI information
pub struct PiInfo(&'static bindings::rpi_hw_t);

impl PiInfo {
    #[inline]
    ///Checks running Hardware, returning if it is supported raspberry pi.
    pub fn detect() -> Option<Self> {
        //This is static data of C library so safe to cast with 'static lifetime
        unsafe {
            bindings::rpi_hw_detect().as_ref().map(PiInfo)
        }
    }

    #[inline(always)]
    ///Returns hardware revision
    pub const fn revision(&self) -> u32 {
        self.0.hwver
    }

    #[inline(always)]
    ///Peripheral base address
    pub const fn periph_base(&self) -> u32 {
        self.0.periph_base
    }

    #[inline(always)]
    ///Video core base address
    pub const fn video_core_base(&self) -> u32 {
        self.0.videocore_base
    }

    #[inline(always)]
    ///Returns descriptive name
    ///
    ///Empty string on utf-8 error.
    pub fn description(&self) -> &'static str {
        let bytes = unsafe {
            let len = libc::strlen(self.0.desc);
            core::slice::from_raw_parts(self.0.desc as *const u8, len as usize)
        };

        match core::str::from_utf8(bytes) {
            Ok(result) => result,
            Err(_) => "",
        }
    }
}

impl fmt::Debug for PiInfo {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PiInfo")
           .field("revision", &self.revision())
           .field("description", &self.description())
           .field("peripheral base", &self.periph_base())
           .field("video core base", &self.video_core_base())
           .finish()
    }
}

#[repr(u32)]
///PWM pins
pub enum PwmPin {
    ///PWM0 on GPIO 12
    Pwm0_1 = 12,
    ///PWM0 on GPIO 18
    Pwm0_2 = 18,
    ///PWM1 on GPIO 13
    Pwm1_1 = 13,
    ///PWM1 on GPIO 19
    Pwm1_2 = 19,
}

#[repr(u32)]
///SPI pins
pub enum SpiPin {
    ///SPI0 available on GPIO 10
    Spi0 = 10,
}

#[repr(u32)]
///PCM pins
pub enum PcmPin {
    ///PCM_DOUT is only available on GPIO 21.
    Board = 21,
    ///P5 header provides GPIO 31 with PCM_DOUT function, not available for all boards.
    P5 = 31,
}

#[repr(transparent)]
///Driver's channel
pub struct Channel(bindings::ws2811_channel_t);

impl Channel {
    ///Creates unused channel.
    pub const fn disabled() -> Self {
        Self(bindings::ws2811_channel_t {
            gpionum: 0,
            invert: 0,
            count: 0,
            strip_type: bindings::WS2811_STRIP_RGB as _,
            leds: ptr::null_mut(),
            brightness: 0,
            wshift: 0,
            rshift: 0,
            gshift: 0,
            bshift: 0,
            gamma: ptr::null_mut(),
        })
    }

    #[inline]
    ///Creates PWM channel
    pub const fn set_pwm(pin: PwmPin) -> Self {
        let mut ch = Channel::disabled();
        ch.0.gpionum = pin as _;
        ch
    }

    #[inline]
    ///Creates PCM channel
    pub const fn set_pcm(pin: PcmPin) -> Self {
        let mut ch = Channel::disabled();
        ch.0.gpionum = pin as _;
        ch
    }

    #[inline]
    ///Creates SPI channel
    pub const fn set_spi(pin: SpiPin) -> Self {
        let mut ch = Channel::disabled();
        ch.0.gpionum = pin as _;
        ch
    }

    #[inline]
    ///Sets brightness.
    pub const fn set_led_count(mut self, led_count: u16) -> Self {
        debug_assert!(led_count > 0);
        self.0.count = led_count as _;
        self
    }

    #[inline]
    ///Sets brightness.
    pub const fn set_brightness(mut self, brightness: u8) -> Self {
        self.0.brightness = brightness;
        self
    }

    #[inline]
    ///Get channel's brightness.
    pub fn get_brightness(&self) -> u8 {
        self.0.brightness
    }

    #[inline]
    ///Changes channel's brightness.
    pub fn change_brightness(&mut self, brightness: u8) {
        self.0.brightness = brightness;
    }

    #[inline]
    const fn set_strip(mut self, strip: u32) -> Self {
        self.0.strip_type = strip as _;
        self
    }

    #[inline]
    ///Sets strip type to `WS2811_STRIP_RGB`
    pub const fn set_strip_rgb(self) -> Self {
        self.set_strip(bindings::WS2811_STRIP_RGB)
    }

    #[inline]
    ///Sets strip type to `WS2811_STRIP_RBG`
    pub const fn set_strip_rbg(self) -> Self {
        self.set_strip(bindings::WS2811_STRIP_RBG)
    }

    #[inline]
    ///Sets strip type to `WS2811_STRIP_GRB`
    pub const fn set_strip_grb(self) -> Self {
        self.set_strip(bindings::WS2811_STRIP_GRB)
    }

    #[inline]
    ///Sets strip type to `WS2811_STRIP_GBR`
    pub const fn set_strip_gbr(self) -> Self {
        self.set_strip(bindings::WS2811_STRIP_GBR)
    }

    #[inline]
    ///Sets strip type to `WS2811_STRIP_BGR`
    pub const fn set_strip_bgr(self) -> Self {
        self.set_strip(bindings::WS2811_STRIP_BGR)
    }

    #[inline]
    ///Sets strip type to `WS2811_STRIP_BRG`
    pub const fn set_strip_brg(self) -> Self {
        self.set_strip(bindings::WS2811_STRIP_BRG)
    }

    #[inline]
    ///Access LEDs array.
    pub fn leds(&self) -> &[bindings::ws2811_led_t] {
        debug_assert!(!self.0.leds.is_null());

        unsafe {
            slice::from_raw_parts(self.0.leds, self.0.count as usize)
        }
    }

    #[inline]
    ///Mutable access LEDs array.
    pub fn leds_mut(&mut self) -> &mut [bindings::ws2811_led_t] {
        debug_assert!(!self.0.leds.is_null());

        unsafe {
            slice::from_raw_parts_mut(self.0.leds, self.0.count as usize)
        }
    }
}

#[derive(Copy, Clone)]
///LED driver builder
///
///Default values:
///
///- `freq` - `WS2811_TARGET_FREQ` constant (800000);
///- `strip` - `WS2811_STRIP_RGB`
///- `dma_channel` - 10;
pub struct DriverBuilder {
    render_wait_time: u64,
    freq: u32,
    dma_channel: u8,
    channel: [bindings::ws2811_channel_t; 2],
}

impl DriverBuilder {
    #[inline]
    ///Creates default configuration:
    pub const fn new() -> Self {
        Self {
            render_wait_time: 0,
            freq: bindings::WS2811_TARGET_FREQ,
            dma_channel: 10,
            channel: [Channel::disabled().0; 2],
        }
    }

    #[inline]
    ///Sets waiting time for rendering function.
    pub const fn render_wait_time(mut self, render_wait_time: u64) -> Self {
        self.render_wait_time = render_wait_time;
        self
    }

    #[inline]
    ///Sets frequency.
    pub const fn freq(mut self, freq: u32) -> Self {
        debug_assert!(freq > 0);
        self.freq = freq;
        self
    }

    #[inline]
    ///Sets DMA channel number to use
    pub const fn dma(mut self, dma_num: u8) -> Self {
        self.dma_channel = dma_num;
        self
    }


    ///Sets first channel
    pub const fn channel1(mut self, channel: Channel) -> Self {
        //PWM1 must be used as channel 2 only
        debug_assert!(channel.0.gpionum != PwmPin::Pwm1_1 as i32);
        debug_assert!(channel.0.gpionum != PwmPin::Pwm1_2 as i32);

        self.channel[0] = channel.0;
        self
    }

    ///Sets second channel
    pub const fn channel2(mut self, channel: Channel) -> Self {
        //PWM1 must be used as channel 2 only
        debug_assert!(channel.0.gpionum == PwmPin::Pwm1_1 as i32);
        debug_assert!(channel.0.gpionum == PwmPin::Pwm1_2 as i32);

        self.channel[1] = channel.0;
        self
    }

    #[inline]
    pub fn build(self) -> Result<Driver, ErrorCode> {
        let inner = bindings::ws2811_t {
            render_wait_time: 0,
            device: core::ptr::null_mut(),
            rpi_hw: core::ptr::null(),
            freq: self.freq,
            dmanum: self.dma_channel as _,
            channel: self.channel,
        };

        Ok(Driver {
            inner,
        })
    }
}

#[repr(transparent)]
///rpi_ws281x driver wrapper.
pub struct Driver {
    inner: bindings::ws2811_t,
}

impl Driver {
    #[inline]
    ///Starts building driver.
    pub const fn builder() -> DriverBuilder {
        DriverBuilder::new()
    }

    #[inline]
    ///Accesses first channel
    pub const fn channel1(&self) -> &'_ Channel {
        unsafe {
            mem::transmute(&self.inner.channel[0])
        }
    }

    #[inline]
    ///Accesses first channel
    pub fn channel1_mut(&mut self) -> &'_ mut Channel {
        unsafe {
            mem::transmute(&mut self.inner.channel[0])
        }
    }

    #[inline]
    ///Accesses second channel
    ///
    ///Only usable when PWM1 is set
    pub const fn channel2(&self) -> &'_ Channel {
        unsafe {
            mem::transmute(&self.inner.channel[1])
        }
    }

    #[inline]
    ///Accesses second channel
    pub fn channel2_mut(&mut self) -> &'_ mut Channel {
        unsafe {
            mem::transmute(&mut self.inner.channel[1])
        }
    }

    #[inline]
    ///Renders LEDs, awaiting previous render completion and performing necessary data transfer.
    pub fn render(&mut self) -> Result<(), ErrorCode> {
        let result = unsafe {
            bindings::ws2811_render(&mut self.inner)
        };

        match result {
            ErrorCode::WS2811_SUCCESS => Ok(()),
            error => Err(error),
        }
    }

    #[inline]
    ///Waits for ongoing data transfer to complete
    ///
    ///Waiting is not needed for SPI channel.
    pub fn wait(&mut self) -> Result<(), ErrorCode> {
        let result = unsafe {
            bindings::ws2811_wait(&mut self.inner)
        };

        match result {
            ErrorCode::WS2811_SUCCESS => Ok(()),
            error => Err(error),
        }
    }

    #[inline]
    ///Sets custom gamma correction using factor value.
    pub fn set_gamma_factor(&mut self, factor: f64) {
        unsafe {
            bindings::ws2811_set_custom_gamma_factor(&mut self.inner, factor)
        }
    }

    #[inline]
    ///Shuts down HW, cleans up memory.
    ///
    ///After that driver can no longer be used.
    ///
    ///It is recommended to always do proper shutdown by dropping this struct or calling `stop`.
    ///In order to guarantee that one can install signal handler that will make sure to call
    ///`stop`.
    pub fn stop(&mut self) {
        if !self.inner.device.is_null() {
            //all dynamic memory is freed and NULLed after this
            //so just don't call it again if it is already null.
            unsafe {
                bindings::ws2811_fini(&mut self.inner)
            }
        }
    }
}

impl Drop for Driver {
    #[inline]
    fn drop(&mut self) {
        self.stop()
    }
}
