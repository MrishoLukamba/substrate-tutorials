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

		// We are agreeing that these type should bound to AppCrypto trait.
		// AppCrypto provides the functionalities for signing , verifying, and types essential
		// for crypto.
		// The implementation of this trait is provided inside Crypto module.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

	}


	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_supply)]
	pub(super) type RemSupply<T: Config> = StorageValue<_ ,u64,ValueQuery>;



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
		pub fn store_supply(origin:OriginFor<T>, supply:u64) -> DispatchResult{
			let _= ensure_signed(origin)?;
			// Store the value in storage
			Ok(())
		}
	}



	impl<T: Config> Pallet<T> {

		//-------------------------------------------------------------------------------------//
		//------------------------SENDING TRANSACTION IMPLEMENTATION---------------------------//

		pub fn send_signed_txn()->Result<(),&'static str> {
			// 3. Get all the accounts that can sign the following txn.
			// Use Signer object.
			let signer = Signer::<T,T::AuthorityId>::all_accounts();


			// 4. Call fetch_externally function and saved the response to a variable


			//-----------------------------------------------------------------------//
			// 5. send a signed transaction using signer object methods

			// Note use proper error handling to notify the state of the offchain worker

			Ok(())
		}
		//-------------------------------------------------------------------------------------//
		//--------------------IMPLEMENTATION OF FETCH_EXTERNALLY METHOD------------------------//




		//You can navigate to the implementation of http by clicking "ctrl + B" when using Intellij.
		//Implement a function that fetches external data
		// Useful crates -> sp_runtime::offchain::http
		// Use lite_json crate to parse the Json body
		//      -       -
		//      -       -
		pub fn fetch_externally()->Result<u64,http::Error> {
		//make an http request to the below API
		//
		   let request = http::Request::
					get("https://api.coinstats.app/public/v1/coins?skip=0&limit=1&currency=EUR");
			//The request object has send method which returns PendingRequest object
			//1. Send the request
			//The PendingRequest object has a method for waiting for the request
			// which you can add a deadline which is optional. And this returns a HttpResult object
			//2. wait for Request to finish
			//3. check the response code if its 200
			//convert the returned body to array of bytes and from bytes obtain strings which
			//you can later turn into json
			//4. Use the parsing helper function to obtain the return value of this function

		}



		//----------------------------------------------------------------------------------//

		//parsing helper function -- you can implement as you like---
		// This helper function is based on lite_json crate
		fn parse_to_int(body: JsonValue) -> Option<u64> {
			let supply = match body {
				JsonValue::Object(object) => {
					let (_,val) = object.into_iter()
						.find(|(k,_)| k.iter().copied().eq("coins".chars()))?;
					match val {
						JsonValue::Array(array) =>
							match array.into_iter().next().unwrap() {
								JsonValue::Object(obj) => {
									let (_,val) = obj.into_iter()
										.find(|(k,_)|k.iter().copied()
											.eq("availableSupply".chars()))?;
									match val {
										JsonValue::Number(i) => Some(i),
										_ => None
									}
								},
								_ => None
							},
						_=> None
					}

				},
				_ => None
			};
			Some(supply.unwrap().integer)
		}
	}
}


