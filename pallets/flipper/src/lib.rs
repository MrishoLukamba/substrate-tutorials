#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn value)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Value<T> = StorageValue<_, bool>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Event documentation should end with an array that provides descriptive names for event
		/// A value has been stored [value, who]
		ValueStored(bool, T::AccountId),
		/// The stored value has been flipped [new_value, who]
		ValueFlipped(bool, T::AccountId),
		/// The stored value has been removed [who]
		ValueRemoved(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Nothing is present in the storage
		NoneValue,
		/// Something is already present in the storage
		AlreadySet,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// This function set a value to the Value storage and emit an event, it should be used once,
		/// if something is already present in the storage, it return an error.
		#[pallet::weight(0)]
		pub fn set_value(origin: OriginFor<T>, value: bool) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Check that there is nothing in the storage.
			match <Value<T>>::get() {
				// Return an error if the value has already been set.
				Some(_) => Err(Error::<T>::AlreadySet)?,
				None => {
					// Update storage.
					<Value<T>>::put(value);

					// Emit an event.
					Self::deposit_event(Event::ValueStored(value, who));

					// Return a successful DispatchResultWithPostInfo.
					Ok(())
				},
			}
		}

		/// This function flip the value and emit an event, if there is no value in the storage then
		/// it return an error.
		#[pallet::weight(0)]
		pub fn flip_value(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check that there is something stored.
			match <Value<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Flip the value.
					let new = !old;
					// Set the value in the storage.
					<Value<T>>::put(new);

					// Emit an event.
					Self::deposit_event(Event::ValueFlipped(new, who));

					Ok(())
				},
			}
		}

		// TODO :
	}
}
