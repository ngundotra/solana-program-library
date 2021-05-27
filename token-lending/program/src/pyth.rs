#![allow(missing_docs)]

use crate::*;
use bytemuck::{cast_slice_mut, from_bytes_mut, try_cast_slice_mut, Pod, Zeroable};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use std::cell::RefMut;

pub const MAGIC: u32 = 0xa1b2c3d4;
pub const VERSION_1: u32 = 1;
pub const VERSION: u32 = VERSION_1;
pub const MAP_TABLE_SIZE: usize = 640;
pub const PROD_ACCT_SIZE: usize = 512;
pub const PROD_HDR_SIZE: usize = 48;
pub const PROD_ATTR_SIZE: usize = PROD_ACCT_SIZE - PROD_HDR_SIZE;

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct AccKey {
    pub val: [u8; 32],
}

#[derive(Default, Copy, Clone)]
struct AccKeyU64
{
  pub val: [u64;4]
}

pub fn cast<T>(d: &[u8]) -> &T {
    let (_, pxa, _) = unsafe { d.align_to::<T>() };
    &pxa[0]
}

impl AccKey {
    pub fn is_valid(&self) -> bool {
        // @FIXME: unsafe
        let k8 = cast::<AccKeyU64>(&self.val);
        k8.val[0] != 0 || k8.val[1] != 0 || k8.val[2] != 0 || k8.val[3] != 0
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum AccountType {
    Unknown,
    Mapping,
    Product,
    Price,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum PriceStatus {
    Unknown,
    Trading,
    Halted,
    Auction,
}

impl Default for PriceStatus {
    fn default() -> Self {
        PriceStatus::Trading
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum CorpAction {
    NoCorpAct,
}

impl Default for CorpAction {
    fn default() -> Self {
        CorpAction::NoCorpAct
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct PriceInfo {
    pub price: i64,
    pub conf: u64,
    pub status: PriceStatus,
    pub corp_act: CorpAction,
    pub pub_slot: u64,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct PriceComp {
    publisher: AccKey,
    agg: PriceInfo,
    latest: PriceInfo,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum PriceType {
    Unknown,
    Price,
    #[allow(clippy::upper_case_acronyms)]
    TWAP,
    Volatility,
}

impl Default for PriceType {
    fn default() -> Self {
        PriceType::Price
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Price {
    pub magic: u32,       // Pyth magic number.
    pub ver: u32,         // Program version.
    pub atype: u32,       // Account type.
    pub size: u32,        // Price account size.
    pub ptype: PriceType, // Price or calculation type.
    pub expo: i32,        // Price exponent.
    pub num: u32,         // Number of component prices.
    pub unused: u32,
    pub curr_slot: u64,  // Currently accumulating price slot.
    pub valid_slot: u64, // Valid slot-time of agg price.
    pub prod: AccKey,
    pub next: AccKey,
    pub agg_pub: AccKey,
    pub agg: PriceInfo,
    pub comp: [PriceComp; 16],
}

impl Price {
    #[inline]
    pub fn load<'a>(account_info: &'a AccountInfo) -> Result<RefMut<'a, Price>, ProgramError> {
        let account_data: RefMut<'a, [u8]>;
        let state: RefMut<'a, Self>;

        account_data = RefMut::map(account_info.try_borrow_mut_data().unwrap(), |data| *data);

        state = RefMut::map(account_data, |data| {
            from_bytes_mut(cast_slice_mut::<u8, u8>(try_cast_slice_mut(data).unwrap()))
        });
        Ok(state)
    }
}

#[cfg(target_endian = "little")]
unsafe impl Zeroable for Price {}

#[cfg(target_endian = "little")]
unsafe impl Pod for Price {}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Product {
    pub magic: u32,                 // pyth magic number
    pub ver: u32,                   // program version
    pub atype: u32,                 // account type
    pub size: u32,                  // price account size
    pub px_acc: AccKey,             // first price account in list
    pub attr: [u8; PROD_ATTR_SIZE], // key/value pairs of reference attr.
}

impl Product {
    #[inline]
    pub fn load<'a>(account_info: &'a AccountInfo) -> Result<RefMut<'a, Product>, ProgramError> {
        let account_data: RefMut<'a, [u8]>;
        let state: RefMut<'a, Self>;

        account_data = RefMut::map(account_info.try_borrow_mut_data().unwrap(), |data| *data);

        state = RefMut::map(account_data, |data| {
            from_bytes_mut(cast_slice_mut::<u8, u8>(try_cast_slice_mut(data).unwrap()))
        });
        Ok(state)
    }
}

#[cfg(target_endian = "little")]
unsafe impl Zeroable for Product {}

#[cfg(target_endian = "little")]
unsafe impl Pod for Product {}
