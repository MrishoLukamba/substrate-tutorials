#![cfg_attr(not(feature = "std"), no_std)]
//There are 2 modules
//Crypto module for setting up offfchain workers crypto identification keys and implementation
//of AppCrypto trait which can be later be used for configuring the pallet in the runime.
// Running this node , needs to setting a validator node manually using pre defined keys Alice and Bob
//
//I have just implement a simple ocw and a single dispatchable for testing .
//This Offchain worker runs every time a node which is Alice validating a block.
//
//Next is to set up http client and implement fecthing data and profe of receiving solution

#[cfg(test)]
mod mock;
#[cfg(test)]
mod ocw;


use sp_runtime::offchain::KeyTypeId;


pub const KEY_TYPE:KeyTypeId = KeyTypeId(*b"toff");
pub use offchain::*;
pub mod offchain {
	use frame_system::offchain::Signer;
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner
	};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct Crypto;

	//implemented for runtime
	 impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for Crypto {
	 	type RuntimeAppPublic = sr25519::AppPublic;
	 	type GenericSignature = Sr25519Signature;
	 	type GenericPublic = sp_core::sr25519::Public;
	 }
	//Implemented for testing environment

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
			Self::send_signed_txn().map_err(|_| log::info!("failed"));
		}

	}

	//---------------------THIS FUNCTION MUTATE THE ON-CHAIN STATE---------------------------//

	#[pallet::call]
	impl<T:Config> Pallet<T>{
		#[pallet::weight(100)]
		pub fn register_ip(origin:OriginFor<T>, ip:Vec<u8>) -> DispatchResult{
			let _= ensure_signed(origin)?;
			for x in ip{
				<IPValue<T>>::try_mutate(|b_vec|{
					b_vec.try_push(x)
				}).map_err(|_| Error::<T>::MaxLenReached)?
			};
			Ok(())
		}
	}



	impl<T: Config> Pallet<T> {

		//-------------------------------------------------------------------------------------//
		//------------------------SENDING TRANSACTION IMPLEMENTATION---------------------------//

		pub fn send_signed_txn()->Result<(),&'static str> {
			//getting all the accounts that can sign the following txn.
			let signer = Signer::<T,T::AuthorityId>::all_accounts();

			//converting to array of bytes as our Call takes that as a parameter_type
			let ip_bytes= Self::fetch_externally().map_err(|_| "failed to fetch")?;

			//-----------------------------------------------------------------------//
			let result = signer.send_signed_transaction(|account|{
				Call::register_ip{ip:ip_bytes.clone()}
			});
			for (acc, res) in result{
				match res{
					Ok(()) => log::info!("success submitted by {:?}",acc.id),
					Err(()) => log::info!("failed submitted by {:?}",acc.id)
				}
			}
			Ok(())
		}
		//-------------------------------------------------------------------------------------//
		//--------------------IMPLEMENTATION OF FETCH_EXTERNALLY METHOD------------------------//




		//You can navigate to the implementation of http by clicking "ctr + B" when using Intellij
		//implement a function that fetches external data
		pub fn fetch_externally()->Result<Vec<u8>,http::Error> {
			//make an http request to "https://api.ipify.org?format=json" to get your personal IP address
			let request = http::Request::get("https://api.ipify.org?format=json");
			//you can add headers its optional
			//sending the request which returns a PendingRequest object with an Id parameter
			// The Request struct has methods for sending the request which returns PendingRequest object
			let pending = request.send().map_err(|_|http::Error::Unknown)?;
			//The PendingRequest object has a method for waiting for the request
			// which you can add a deadline which is optional. And this returns a HttpResult
			// type alias with a Response object.
			let result = pending.wait();

			let response = result.map_err(|_| http::Error::IoError)?;
			//the returned Response object has status code parameter which we may check if its
			//success before proceeding
			if response.code != 200 {
				log::info!("bad code");
				return Err(http::Error::Unknown)
			}

			log::info!("fetched success");
			//convert the returned body to array of bytes and from bytes obtain strings which
			//you can later turn into json
			let body = response.body().collect::<Vec<u8>>();
			let body_str = sp_std::str::from_utf8(&body[..])
				.map_err(|_|http::Error::Unknown)?;
			let body_json = lite_json::json_parser::
			parse_json(body_str).map_err(|_| http::Error::Unknown)?;
			log::info!("{}",body_str);

			let ip = Self::parse_to_bytes(body_json).unwrap();
			// let ip_str = sp_std::str::from_utf8(&ip[..]).map_err(|_| http::Error::Unknown)?;
			// log::info!("{:?}",ip_str);
			Ok(ip)
		}



		//----------------------------------------------------------------------------------//

		//passing helper function -- you can implement as you like---
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


