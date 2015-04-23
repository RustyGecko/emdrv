extern crate gcc;

use gcc::Config;

use std::env;

#[cfg(feature = "dk3750")] use dk3750 as kit;
#[cfg(feature = "stk3700")] use stk3700 as kit;

// Kit-specific gcc configuration
#[cfg(feature = "dk3750")] mod dk3750;
#[cfg(feature = "stk3700")] mod stk3700;

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap();

    println!("The ARM embedded toolchain must be available in the PATH");
    env::set_var("CC", "arm-none-eabi-gcc");
    env::set_var("AR", "arm-none-eabi-ar");

    let mut config = Config::new();

    common_config(&mut config);
    kit::kit_config(&mut config);

    config.compile("libemdrv.a");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=emdrv");

}

fn common_config(config: &mut Config) -> &mut Config {

    config
        .define("EFM32GG990F1024", None)

        .flag("-Wall")
        .flag("-mcpu=cortex-m3")
        .flag("-mthumb")

        .include("src")
        .include("efm32-common/CMSIS/Include")
        .include("efm32-common/Device/EFM32GG/Include")
        .include("efm32-common/emlib/inc")
        .include("efm32-common/kits/common/bsp")
        .include("efm32-common/kits/common/drivers")

        .file("efm32-common/kits/common/drivers/nandflash.c")
        .file("efm32-common/kits/common/drivers/dmactrl.c")
        .file("efm32-common/kits/common/drivers/retargetio.c")

}
