#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::log;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// 写入OffChain的数据类型
    pub type OffChainDataType = BoundedVec<u8, ConstU32<4>>;

    const ON_CHAIN_T0_OFF_CHAIN_INDEX: &[u8] = b"kictto:data_index";

    #[derive(Debug, Encode, Decode, Default)]
    struct OffChainData(OffChainDataType);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // #[pallet::storage]
    // #[pallet::getter(fn something)]
    // pub type Something<T> = StorageValue<_, u32>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        OffChainDataStored { data: OffChainDataType, who: T::AccountId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn save_data_to_off_chain(origin: OriginFor<T>, data: OffChainDataType) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let to_store_data = OffChainData(data.clone());
            sp_io::offchain_index::set(&ON_CHAIN_T0_OFF_CHAIN_INDEX, &to_store_data.encode());
            log::info!("data set:{:?}",&to_store_data);
            Self::deposit_event(Event::OffChainDataStored { data, who });
            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("OffChainWorker ==> RUN");
            if let Some(data_stored) =
                sp_runtime::offchain::storage::StorageValue::persistent(ON_CHAIN_T0_OFF_CHAIN_INDEX)
                    .get::<OffChainData>()
                    .unwrap_or_else(|_| {
                        log::info!("OffChainWorker ==> 无法从OffChainStorage读取数据");
                        None
                    }) {
                log::info!("OffChainWorker ==> 读取成功:{:?}",data_stored.0)
            }
        }
    }
}
