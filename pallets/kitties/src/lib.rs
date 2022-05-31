#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::{DispatchResult, DispatchResultWithPostInfo},
		pallet_prelude::*,
		sp_runtime::traits::{Hash, Zero},
		traits::{Currency, ExistenceRequirement, Randomness},
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;

	// TODO: Struct for holding kitty information.

	// TODO: Enum and implementation to handle Gender type in Kitty struct.

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types it depends on.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits event, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency handler for the kitties pallet.
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}
