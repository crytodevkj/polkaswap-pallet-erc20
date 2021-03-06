use crate::{Module, Trait};
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

impl_outer_origin! {
    pub enum Origin for Test where system = frame_system {}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Trait for Test {
    type Version = ();
    type PalletInfo = ();
    type OnNewAccount = ();
    type SystemWeightInfo = ();
    type AccountData = ();
    type OnKilledAccount = ();
    type MaximumBlockWeight = MaximumBlockWeight;
    type BlockExecutionWeight = ();
    type DbWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type BaseCallFilter = ();
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Origin = Origin;
    type Call = ();
    type AccountId = u64;
    type BlockNumber = u64;
    type Index = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type AvailableBlockRatio = AvailableBlockRatio;
    type MaximumBlockLength = MaximumBlockLength;

}

impl Trait for Test {
    type Event = ();
    type TokenBalance = u64;
    type AssetId = u32;
}

pub type Assets = Module<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
