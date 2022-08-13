#![cfg_attr(not(feature = "std"), no_std)]

// This tutorial contains instructions and
// commented code for which can be commented out and complete the task
// Solution is in tutorial.rs
// dont forget to implement also in Runtime/src
#[cfg(test)]
mod mock;
#[cfg(test)]
mod ocw;


use sp_runtime::offchain::KeyTypeId;


pub const KEY_TYPE:KeyTypeId = KeyTypeId(*b"toff");
pub use offchain::*;
pub mod offchain {

	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner
	};

	// Setting up keystore with a unique id which will be used to fetch keys of type
	// sr25519 by offchain worker.
	app_crypto!(sr25519, KEY_TYPE);

	pub struct Crypto;

	//The AppCrypto trait is used for signing transaction by Offchain worker.
	// 1. Implement it on struct Crypto
	//  impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for Crypto {
	//  	type RuntimeAppPublic = // A crypto type that implements RuntimeAppPublic trait;
	//  	type GenericSignature = // A crypto primitive signature type;
	//  	type GenericPublic = // A crypto public key type;
	//  }

	//Note : Crypto primitve types are at sp_core crate
	//Useful crates -> sp-runtime::app_crypto.

	// This implementation will be used when implementing pallet_template on Runtime.

}


pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,

	};
	use sp_std::vec::Vec;
	use frame_support::storage::bounded_vec::BoundedVec;
	use frame_system::pallet_prelude::*;
	use frame_system::offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	};
	use sp_runtime::RuntimeDebug;
	use log;
	use sp_runtime::offchain::{http, Duration};
	use lite_json::json::JsonValue;
	use lite_json::parse_json;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	// CreateSignedTransaction trait must be a super trait as it has functionalities for creating
	// a signed transaction.
	// Later will be implemented on runtime
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// We are agreeing that these type should bound to AppCrypto trait.
		// AppCrypto provides the functionalities for signing , verifying, and types essential
		// for crypto.
		// The implementation of this trait is provided inside Crypto module.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		type MaxBytes: Get<u32>;
	}


	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_ip)]
	pub(super) type IPValue<T: Config> = StorageValue<_,BoundedVec<u8,T::MaxBytes> , ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {

	}
	#[pallet::error]
	pub enum Error<T>{
		MaxLenReached
	}



	#[pallet::hooks]
	impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn offchain_worker(block_number: T::BlockNumber) {
			// 7. Call send_signed_txn function
			//Use proper error handling
			// the offchain worker will run and submit the transaction into txn_pool.
			}
		}



	//---------------------THIS FUNCTION MUTATE THE ON-CHAIN STATE---------------------------//
	//Offchain workers use transactions to change state on-chain.
	// A pallet call is used to submit data into on-chain storage

	#[pallet::call]
	impl<T:Config> Pallet<T>{
		#[pallet::weight(100)]
		pub fn register_ip(origin:OriginFor<T>, ip:Vec<u8>) -> DispatchResult{
			let _= ensure_signed(origin)?;
			// 6. Store the array of bytes on chain storage IPValue
			Ok(())
		}
	}



	impl<T: Config> Pallet<T> {

		//-------------------------------------------------------------------------------------//
		//------------------------SENDING TRANSACTION IMPLEMENTATION---------------------------//

		pub fn send_signed_txn()->Result<(),&'static str> {
			// 3. Get all the accounts that can sign the following txn.
			// Use Signer object.


			// 4. Call fetch_externally function and saved the response to a variable


			//-----------------------------------------------------------------------//
			// 5. send a signed transaction using signer object methods

			// Note use proper error handling to notify the state of the offchain worker

			Ok(())
		}
		//-------------------------------------------------------------------------------------//
		//--------------------IMPLEMENTATION OF FETCH_EXTERNALLY METHOD------------------------//




		//You can navigate to the implementation of http by clicking "ctrl + B" when using Intellij.
		// 2 .Implement a function that fetches external data
		// Useful crates -> sp_runtime::offchain::http
		// Use lite_json crate to parse the Json body
		// Comment out the below function
		//      -       -
		//      -       -
		//pub fn fetch_externally()->Result<Vec<u8>,http::Error> {
			//make an http request to "https://api.ipify.org?format=json"
			// to get your personal IP address
			//
			//
			// The below return value is just to avoid errors when compiling, please remove it when
			// implementing the fucntion.

		//}



		//----------------------------------------------------------------------------------//

		//parsing helper function -- you can implement as you like---
		// This helper function is based on lite_json crate
		fn parse_to_bytes(body: JsonValue) -> Option<Vec<u8>> {
			let val = match body {
				JsonValue::Object(obj) => {
					let (_,v) = obj.into_iter()
						.find(|(k,_)|k.iter().copied().eq("ip".chars()))?;
					match v {
						JsonValue::String(n) => Some(n),
						_ => None
					}
				}
				_=> None
			};
			let bytes_from_chars:Vec<u8> = val.unwrap().iter().map(|ch| *ch as u8).collect();
			Some(bytes_from_chars)
		}
	}
}


