use embedded_sdmmc::{self, BlockDevice, TimeSource, Timestamp};

use super::{kernel_call_a, ReadBlockArgs};

pub struct SdCard{

}

impl BlockDevice for &SdCard{
    type Error = u32;

    fn read(
        &self,
        blocks: &mut [embedded_sdmmc::Block],
        start_block_idx: embedded_sdmmc::BlockIdx,
        reason: &str,
    ) -> Result<(), Self::Error> {
        let mut args: ReadBlockArgs = ReadBlockArgs { blocks: blocks, start_block: start_block_idx, reason: reason };
        unsafe{
            let mut ptr = &mut args;
            let ptr2 = &mut ptr as *mut &mut ReadBlockArgs;
            kernel_call_a(super::KernelFunction::ReadBlock, ptr2 as u64);
        }
        Ok(())
    }

    fn write(&self, blocks: &[embedded_sdmmc::Block], start_block_idx: embedded_sdmmc::BlockIdx) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn num_blocks(&self) -> Result<embedded_sdmmc::BlockCount, Self::Error> {
        unimplemented!()
    }
}

pub struct TestClock{}

impl TimeSource for TestClock{
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        Timestamp { year_since_1970: 0, zero_indexed_month: 0, zero_indexed_day: 0, hours: 0, minutes: 0, seconds: 0 }
    }
}


pub static SDCARD: SdCard = SdCard{};

