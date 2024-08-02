//! Benchmarking setup for pallet-multisig
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Multisig;
use frame_benchmarking::{v2::*, whitelisted_caller};
use frame_system::RawOrigin;


#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_multisig_group() {
		let caller: T::AccountId = whitelisted_caller();
		let member1: T::AccountId = account("member1", 0, 0);
		let member2: T::AccountId = account("member2", 0, 0);
		let member3: T::AccountId = account("member3", 0, 0);
		let members = vec![caller.clone(), member1.clone(), member2.clone(), member3.clone()];

		#[extrinsic_call]
		create_multisig_group(RawOrigin::Signed(caller.clone()), members);

		assert!(Multisig::<T>::members().contains(&caller));
		assert!(Multisig::<T>::members().contains(&member1));
		assert!(Multisig::<T>::members().contains(&member2));
		assert!(Multisig::<T>::members().contains(&member3));
	}

	#[benchmark]
	fn add_member() {
		let caller: T::AccountId = whitelisted_caller();
		let member1: T::AccountId = account("member1", 0, 0);
		let member2: T::AccountId = account("member2", 0, 0);
		let member3: T::AccountId = account("member3", 0, 0);
		let new_member: T::AccountId = account("new_member", 0, 0);

		#[extrinsic_call]
		Multisig::<T>::create_multisig_group(
			RawOrigin::Signed(caller.clone()),
			vec![caller.clone(), member1.clone(), member2.clone(), member3.clone()],
		);
		let _ = Multisig::<T>::add_member(RawOrigin::Signed(caller).into(), new_member.clone());

		assert!(!Multisig::<T>::members().contains(&new_member));
	}

	#[benchmark]
	fn remove_member() {
		let caller: T::AccountId = whitelisted_caller();
		let member1: T::AccountId = account("member1", 0, 0);
		let member2: T::AccountId = account("member2", 0, 0);
		let member3: T::AccountId = account("member3", 0, 0);
		let member4: T::AccountId = account("member4", 0, 0);
		#[extrinsic_call]
		Multisig::<T>::create_multisig_group(
			RawOrigin::Signed(caller.clone()),
			vec![
				caller.clone(),
				member1.clone(),
				member2.clone(),
				member3.clone(),
				member4.clone(),
			],
		);

		let _ = Multisig::<T>::remove_member(RawOrigin::Signed(caller).into(), member4.clone());

		assert!(Multisig::<T>::members().contains(&member4));
	}

	#[benchmark]
	fn approve_proposal() {
		let caller: T::AccountId = whitelisted_caller();
		let member1: T::AccountId = account("member1", 0, 0);
		let member2: T::AccountId = account("member2", 0, 0);
		#[extrinsic_call]
		Multisig::<T>::create_multisig_group(
			RawOrigin::Signed(caller.clone()),
			vec![caller.clone(), member1.clone(), member2.clone()],
		);
		let _ = Multisig::<T>::create_proposal(
			RawOrigin::Signed(caller).into(),
			3,
			2,
		);

		let _ = Multisig::<T>::approve(RawOrigin::Signed(member1.clone()).into(), 1);

		let proposal_vote = Voting::<T>::get(1).unwrap_or_else(||{
			Votes::<T> {
				index: 1,
				threshold: 2,
				ayes: vec![member1.clone()],
				nays: vec![],
			}
		});
		assert!(proposal_vote.ayes.contains(&member1));
	}

	#[benchmark]
	fn reject_proposal() {
		let caller: T::AccountId = whitelisted_caller();
		let member1: T::AccountId = account("member1", 0, 0);
		let member2: T::AccountId = account("member2", 0, 0);
		let member3: T::AccountId = account("member3", 0, 0);
		#[extrinsic_call]
		Multisig::<T>::create_multisig_group(
			RawOrigin::Signed(caller.clone()),
			vec![caller.clone(), member1.clone(), member2.clone(), member3.clone()],
		);

		let _ = Multisig::<T>::create_proposal(
			RawOrigin::Signed(caller).into(),
			3,
			2,
		);
		let _ = Multisig::<T>::approve(RawOrigin::Signed(member1).into(), 1);
		let _ = Multisig::<T>::approve(RawOrigin::Signed(member2).into(), 1);
		let _ = Multisig::<T>::reject(RawOrigin::Signed(member3.clone()).into(), 1);

		let proposal_vote = Voting::<T>::get(1).unwrap_or_else(|| {
			Votes::<T> {
				index: 1,
				threshold: 2,
				ayes: vec![],
				nays: vec![member3.clone()],
			}
		});
		assert!(proposal_vote.nays.contains(&member3));
	}

	impl_benchmark_test_suite!(Multisig, crate::mock::new_test_ext(), crate::mock::Test);
}
