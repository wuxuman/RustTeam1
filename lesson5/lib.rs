#![cfg_attr(not(feature = "std"), no_std)]
//使用std或者no_std标签编译
pub use pallet::*;

// a module for proof of existence
//使用pallet宏，定义pallet模块
#[frame_support::pallet]
pub mod pallet {
    //引入相关依赖
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    //定义模块配置接口
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    //定义Pallet结构体承载功能模块
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    //定义存储单元
    #[pallet::storage]
    pub(super) type Proofs<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

    //定义Event枚举类型
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
    }

    //定义error 枚举来包含error信息
    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyClaimed,
        NoSuchProof,
        NotProofOwner,
    }

    //例如，区块开头执行的函数，区块结尾时执行的函数，定义在hooks里
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    //定义可调用函数
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]

        pub(super) fn create_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofAlreadyClaimed,
            );
            Proofs::<T>::insert(&claim, (&sender, <frame_system::Module<T>>::block_number()));
            Self::deposit_event(Event::ClaimCreated(sender, claim));
            Ok(().into())
        }

        //吊销存证
        #[pallet::weight(0)]
        fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::NoSuchProof);

            let (owner, _) = Proofs::<T>::get(&claim);

            ensure!(sender == owner, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(sender, claim));

            Ok(().into())
        }
    }
}
