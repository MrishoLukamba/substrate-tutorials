use std::sync::Arc;
use frame_support::assert_ok;
use crate as pallet_template;
use frame_support::traits::{ConstU16, ConstU32, ConstU64};
use frame_system as system;
use frame_system::offchain::{AppCrypto,SigningTypes, CreateSignedTransaction, SendTransactionTypes};
use sp_core::H256;
use sp_runtime::{
	testing::{Header,TestSignature, TestXt},
	traits::{BlakeTwo256, IdentityLookup},
};
use sp_core::{
	offchain::{testing, OffchainWorkerExt, TransactionPoolExt},
	sr25519::Signature,
};

use sp_runtime::{
	traits::{Extrinsic, IdentifyAccount, Verify}
};
//use sp_keystore::{testing::KeyStore, KeystoreExt, CryptoStore};
pub use crate::offchain;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		TemplateModule: pallet_template,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

//Implementing CreateSignedTransaction trait and other dependency traits
type AccountId = <<TestSignature as Verify>::Signer as IdentifyAccount>::AccountId;
type TestExtrinsic = TestXt<Call,()>;

impl<T> CreateSignedTransaction<T> for Test where Call: From<T>,{
	fn create_transaction<C: AppCrypto<Self::Public, Self::Signature>>
	(call: Call,
	 _public:<TestSignature as Verify>::Signer,
	 _account: AccountId,
	 nonce: u64
	) -> Option<(Call, <TestExtrinsic as Extrinsic>::SignaturePayload)> {
		Some((call,(nonce,())))
	}
}

impl<L> SendTransactionTypes<L> for Test where Call: From<L>{
	type Extrinsic = TestExtrinsic;
	type OverarchingCall = Call;
}
impl SigningTypes for Test {
	type Public = <TestSignature as Verify>::Signer;
	type Signature = TestSignature;
}


impl pallet_template::Config for Test {
	type Event = Event;
	type AuthorityId = ocw_test::TestCrypto;
	type MaxBytes = ConstU32<16>;
}

// Build genesis storage according to the mock runtime.
// pub fn new_test_ext() -> sp_io::TestExternalities {
// 	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
//
//

//-----------------------TESTING--------------------------------------------//

use frame_support::{assert_noop};
use sp_core::offchain::testing::{TestOffchainExt, TestTransactionPoolExt};


use sp_io;
use sp_io::TestExternalities;
use crate::ocw::ocw_test;

#[test]
fn testing_call_function() {
	// Setting up testing environment
	let mut test_env = TestExternalities::default();
	test_env.execute_with(||{
		assert_ok!(TemplateModule::register_ip(Origin::signed(1),vec![20,10,30,108]));
		//Checking storage
		assert_eq!(TemplateModule::get_ip(),vec![20,10,30,108])
	});

}

#[test]
fn test_fetch_externally(){
	let mut test_env = TestExternalities::default();
	//Getting OCW environment
	let (ocw,ocw_state) = testing::TestOffchainExt::new();
	//Configuring OCW environment
	let ocw_env = OffchainWorkerExt::new(ocw);
	//Registering OCW
	test_env.register_extension(ocw_env);

	// Expecting a request and a mocking result
	ocw_state.write().expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: "https://api.ipify.org?format=json".into(),
		response: Some(br#"{"ip": "197.250.228.247"}"#.to_vec()),
		sent: true,
		..Default::default()
	});

	test_env.execute_with(||{
		let ip = TemplateModule::fetch_externally().unwrap();
		let ip_str = sp_std::str::from_utf8(&ip[..]).unwrap();
		assert_eq!(ip_str,"197.250.228.247");
	});
}


// #[test]
// pub fn test_sending_txn_to_the_pool(){
// 	let mut test_env = TestExternalities::default();
// 	let (ocw,ocw_state) = TestOffchainExt::new();
// 	let ocw_env = OffchainWorkerExt::new(ocw);
// 	test_env.register_extension(ocw_env);
// 	Getting txn_pool env
	// let (pool, pool_state) = TestTransactionPoolExt::new();
	// Getting txn_pool environment
	// let pool_env = TransactionPoolExt::new(pool);
	// registering txn_pool environment
	// test_env.register_extension(pool_env);
	// Keystore environment
	// let key_store_env = KeyStore::new();
	// key_store_env.sr25519_generate_new(
	// 	palet_template::KEY_TYPE,
	// 	None
	// ).unwrap();
	// test_env.register_extension(KeystoreExt(Arc::new(key_store_env)));
	// testing
	// TemplateModule::send_signed_txn().unwrap();
	// let mut txn = pool_state.transactions.pop();
	// let decoded_call = TestExtrinsic::decode(&mut &*txn).unwrap();
	// assert_eq!(decoded_call, Call::TemplateModule(Call::register_ip{ip:vec![]}));
//
// }
