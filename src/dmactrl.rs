use emlib::dma;
use core::intrinsics::transmute;

pub fn dma_control_block() -> &'static dma::Descriptor {
    unsafe { transmute(GET_DMA_CONTROL_BLOCK()) }
}

extern {
    fn GET_DMA_CONTROL_BLOCK() -> &'static dma::Descriptor;
}
