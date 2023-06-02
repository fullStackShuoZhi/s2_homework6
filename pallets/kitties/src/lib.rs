#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
//
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use sp_io::hashing::blake2_128;
    use frame_support::traits::Randomness;

    /// ID
    pub type KittyId = u32;

    /// 数据存储的类型和长度
    #[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
    pub struct Kitty(pub [u8; 16]);

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
    }

    /// 存储KittyId
    #[pallet::storage]
    #[pallet::getter(fn next_kitty_id)]
    pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery>;     // 此处给定了第三个参数，该参数用于给定默认值，对于u32类型的KittyId来说，它就是0

    /// 存储Kitty的数据内容
    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;
    /// 存储Kitty的Owner
    #[pallet::storage]
    #[pallet::getter(fn kitty_owner)]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;
    /// 存储Kitty的继承关系
    #[pallet::storage]
    #[pallet::getter(fn kitty_parents)]
    pub type KittyParents<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId), OptionQuery>;


    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Kitty创建成功
        KittyCreated { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
        /// Kitty breed成功
        KittyBred { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },
        /// Kitty 转移成功
        KittyTransferred { who: T::AccountId, recipient: T::AccountId, kitty_id: KittyId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// KittyId创建失败
        InvalidKittyId,
        /// KittyId相同
        SameKittyId,
        /// 非Owner
        NotOwner,
        /// 转移给自己
        CanNotTransferToSelf,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 创建Kitty
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let kitty_id = Self::get_next_id()?;
            let kitty = Kitty(Self::random_value(&who));
            Kitties::<T>::insert(kitty_id, &kitty);
            KittyOwner::<T>::insert(kitty_id, &who);


            // 发布创建成功事件
            Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        /// 两个kitty，生成一个子kitty
        #[pallet::call_index(1)]
        #[pallet::weight(10_001 + T::DbWeight::get().writes(1).ref_time())]
        pub fn breed(origin: OriginFor<T>, kitty_id_1: KittyId, kitty_id_2: KittyId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(kitty_id_1 != kitty_id_2,Error::<T>::SameKittyId);
            ensure!(Kitties::<T>::contains_key(kitty_id_1),Error::<T>::InvalidKittyId);
            ensure!(Kitties::<T>::contains_key(kitty_id_2),Error::<T>::InvalidKittyId);

            let kitty_id = Self::get_next_id()?;

            let kitty_1_result = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId);
            let kitty_1 = kitty_1_result?.0;
            // let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId);
            let kitty_2_result = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId);
            let kitty_2 = kitty_2_result?.0;
            // let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId);

            let selector = Self::random_value(&who);
            let mut data = [0u8; 16];
            for i in 0..kitty_1.len() {
                data[i] = (kitty_1[i] & selector[i]) | (kitty_2[i] & !selector[i])
            }
            let kitty = Kitty(data);

            Kitties::<T>::insert(kitty_id, &kitty);
            KittyOwner::<T>::insert(kitty_id, &who);
            KittyParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));

            // 发布创建成功事件
            Self::deposit_event(Event::KittyBred { who, kitty_id, kitty });

            Ok(())
        }

        /// 转移kitty
        #[pallet::call_index(2)]
        #[pallet::weight(10_002 + T::DbWeight::get().writes(1).ref_time())]
        pub fn transfer(origin: OriginFor<T>, recipient: T::AccountId, kitty_id: KittyId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(Kitties::<T>::contains_key(kitty_id),Error::<T>::InvalidKittyId);

            ensure!( Self::kitty_owner(kitty_id) == Some(who.clone()),Error::<T>::NotOwner);

            ensure!(recipient == who,Error::<T>::CanNotTransferToSelf);

            KittyOwner::<T>::insert(kitty_id, &recipient);

            Self::deposit_event(Event::KittyTransferred { who, recipient, kitty_id });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 返回一个kittyId，并+1后保存为下一个kittyId
        fn get_next_id() -> Result<KittyId, DispatchError> {
            NextKittyId::<T>::try_mutate(|next_id| -> Result<KittyId, DispatchError> {
                // 读取当前的 此时完成了copy
                let current_id = *next_id;
                // 更新下一个id，可能超出u32的范围，溢出则抛出Error
                *next_id = next_id.checked_add(1).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
                Ok(current_id)
            })
        }
        /// 生成一个随机数
        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            // 多个参数，确保payload唯一
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            // 用blake2_128确保长度match
            payload.using_encoded(blake2_128)
        }

    }
}
