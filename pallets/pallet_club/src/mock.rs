use crate as pallet_club;
use frame_support::{
	PalletId,
	derive_impl, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, ConstU128},
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

parameter_types! {
	pub const ClubPallet: PalletId = PalletId(*b"ClubMngr");
}

type Block = frame_system::mocking::MockBlock<Test>;
pub type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		Balances: pallet_balances,
		Timestamp: pallet_timestamp,
		Club: pallet_club,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxLocks = ConstU32<10>;
	type MaxReserves = ConstU32<10>;
	type MaxFreezes = ConstU32<10>;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1>;
	type WeightInfo = ();
}

impl crate::weights::WeightInfo for () {
	fn create_club() -> frame_support::weights::Weight {
		frame_support::weights::Weight::from_parts(10_000_000, 0)
	}
	fn transfer_ownership() -> frame_support::weights::Weight {
		frame_support::weights::Weight::from_parts(8_000_000, 0)
	}
	fn set_annual_fee() -> frame_support::weights::Weight {
		frame_support::weights::Weight::from_parts(8_000_000, 0)
	}
	fn join_club() -> frame_support::weights::Weight {
		frame_support::weights::Weight::from_parts(15_000_000, 0)
	}
}

impl pallet_club::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CreationFee = ConstU128<100>;
	type MaxYears = ConstU32<100>;
	type YearDuration = ConstU64<31_536_000_000>; // 1 year in milliseconds
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap()
		.into()
}