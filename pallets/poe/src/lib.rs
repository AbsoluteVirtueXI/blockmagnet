#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use core::ops::Bound;

	use frame_support::{pallet_prelude::*, BoundedVec};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/* Placeholder for defining custom types. */

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// For constraining the maximum bytes of a hash used for any proof
		type MaxBytesInHash: Get<u32>;
	}

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive names for parameters.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a proof has been claimed. [who, claim]
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
		/// Event emitted when a claim is revoked by the owner. [who, claim]
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	// TODO: add #[pallet::storage] block

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}
