use gcc::Config;

pub fn kit_config(config: &mut Config) -> &mut Config {
    println!("Configuring DK3750");
    super::common_config(config)
        .include("efm32-common/kits/EFM32GG_DK3750/config")

        .include("efm32-common/reptile/glib")

        .file("efm32-common/reptile/glib/dmd/ssd2119/dmd_ssd2119_direct.c")
        .file("efm32-common/kits/common/drivers/tftdirect.c")
        .file("efm32-common/kits/common/drivers/tftspi.c")

}
