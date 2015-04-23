use gcc::Config;

pub fn kit_config(config: &mut Config) -> &mut Config {
    println!("Configuring STK3700");
    super::common_config(config)
        .include("efm32-common/kits/EFM32GG_STK3700/config")

        .include("emdrv")
        .file("src/i2c1drv.c")
        .file("efm32-common/kits/common/drivers/i2cdrv.c")
        .file("efm32-common/kits/common/drivers/si7013.c")


}
