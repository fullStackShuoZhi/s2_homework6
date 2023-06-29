#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use frame_support::pallet_prelude::*;
pub use frame_system::pallet_prelude::*;
pub use pallet::*;
pub use weights::WeightInfo;


pub mod weights;


#[frame_support::pallet]
pub mod pallet {
    use super::*;
    pub use frame_support::inherent::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// 定义存证的最大数据长度
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// 权重信息
        type WeightInfo: WeightInfo;
    }

    // Pallets use events to inform users when important changes are made.
    // Event documentation should end with an array that provides descriptive names for parameters.
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// 存证创建成功
        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        /// 存证被所有者撤销
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        /// 存证被所有者转移
        ClaimTransferred(T::AccountId, BoundedVec<u8, T::MaxClaimLength>, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// 存证已经存在
        ClaimAlreadyExisted,
        /// 存在不存在
        NoSuchClaim,
        /// 非存证所有者
        NotClaimOwner,
        /// 创建存证失败
        FailToCreateClaim,
        /// 不能转移给自己
        CanNotTransferToSelf,
    }

    /// 定义存证的存储结构
    #[pallet::storage]
    #[pallet::getter(fn claims)]
    pub(super) type Claims<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>, // key类型
        (T::AccountId, T::BlockNumber) // value类型
    >;

    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 创建存证
        #[pallet::call_index(1)]
        // #[pallet::weight(0)]
        #[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))]
        pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            let sender = ensure_signed(origin)?;

            // 验证存证是否存在
            ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::ClaimAlreadyExisted);

            // Get the block number from the FRAME System pallet.
            let current_block = <frame_system::Pallet<T>>::block_number();

            // Store the claim with the sender and block number.
            Claims::<T>::insert(&claim, (&sender, current_block));

            // Emit an event that the claim was created.
            Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(())
        }
        /// 撤销存证
        #[pallet::call_index(2)]
        // #[pallet::weight(0)]
        #[pallet::weight(T::WeightInfo::revoke_claim(claim.len() as u32))]
        pub fn revoke_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            let sender = ensure_signed(origin)?;

            // Get owner of the claim, if none return an error.
            let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

            // 确定发起人是凭证的拥有者
            ensure!(sender == owner, Error::<T>::NotClaimOwner);

            // Remove claim from storage.
            Claims::<T>::remove(&claim);

            // Emit an event that the claim was erased.
            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(())
        }
        /// 转移存证
        #[pallet::call_index(3)]
        // #[pallet::weight(0)]
        #[pallet::weight(T::WeightInfo::transfer_claim(claim.len() as u32))]
        pub fn transfer_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>, receiver: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Get owner of the claim, if none return an error.
            let (owner, block_content) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

            // 确定发起人是凭证的拥有者
            ensure!(sender == owner, Error::<T>::NotClaimOwner);
            // 确定接受人与持有人不是同一个人
            ensure!(receiver != owner,Error::<T>::CanNotTransferToSelf);

            Claims::<T>::insert(&claim, (receiver.clone(), block_content));

            Self::deposit_event(Event::ClaimTransferred(sender, claim, receiver));

            Ok(())
        }
    }
}