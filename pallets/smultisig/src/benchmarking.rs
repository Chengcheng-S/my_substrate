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

		assert!(MultisigMembers::<T>::contains_key(&caller));
		assert!(MultisigMembers::<T>::contains_key(&member1));
		assert!(MultisigMembers::<T>::contains_key(&member2));
		assert!(MultisigMembers::<T>::contains_key(&member3));
	}

	#[benchmark]
	fn add_member() {
		let caller: T::AccountId = whitelisted_caller();
		let member1: T::AccountId = account("member1", 0, 0);
		let member2: T::AccountId = account("member2", 0, 0);
		let member3: T::AccountId = account("member3", 0, 0);
		let new_member: T::AccountId = account("new_member", 0, 0);
		Multisig::<T>::create_multisig_group(
			RawOrigin::Signed(caller.clone()).into(),
			vec![caller.clone(), member1.clone(), member2.clone(), member3.clone()],
		)?;

		#[extrinsic_call]
		add_member(RawOrigin::Signed(caller.clone()), new_member.clone());

		assert!(MultisigMembers::<T>::contains_key(&new_member));
	}

	#[benchmark]
	fn remove_member() {
		let caller: T::AccountId = whitelisted_caller();
		let member1: T::AccountId = account("member1", 0, 0);
		let member2: T::AccountId = account("member2", 0, 0);
		let member3: T::AccountId = account("member3", 0, 0);
		let member4: T::AccountId = account("member4", 0, 0);
		Multisig::<T>::create_multisig_group(
			RawOrigin::Signed(caller.clone()).into(),
			vec![
				caller.clone(),
				member1.clone(),
				member2.clone(),
				member3.clone(),
				member4.clone(),
			],
		)?;

		#[extrinsic_call]
		remove_member(RawOrigin::Signed(caller.clone()), member4.clone());

		assert!(!MultisigMembers::<T>::contains_key(&member4));
	}

	#[benchmark]
	fn approve_proposal() {
		let caller: T::AccountId = whitelisted_caller();
		let member1: T::AccountId = account("member1", 0, 0);
		let member2: T::AccountId = account("member2", 0, 0);
		Multisig::<T>::create_multisig_group(
			RawOrigin::Signed(caller.clone()).into(),
			vec![caller.clone(), member1.clone(), member2.clone()],
		)?;
		Multisig::<T>::create_proposal(
			RawOrigin::Signed(caller.clone()).into(),
			Proposal::dummy(),
			2,
		)?;

		#[extrinsic_call]
		approve(RawOrigin::Signed(member1.clone()), 1);

		let proposal_vote = Voting::<T>::get(1).unwrap();
		assert!(proposal_vote.ayes.contains(&member1));
	}

	#[benchmark]
	fn reject_proposal() {
		let caller: T::AccountId = whitelisted_caller();
		let member1: T::AccountId = account("member1", 0, 0);
		let member2: T::AccountId = account("member2", 0, 0);
		let member3: T::AccountId = account("member3", 0, 0);
		Multisig::<T>::create_multisig_group(
			RawOrigin::Signed(caller.clone()).into(),
			vec![caller.clone(), member1.clone(), member2.clone(), member3.clone()],
		)?;
		Multisig::<T>::create_proposal(
			RawOrigin::Signed(caller.clone()).into(),
			Proposal::dummy(),
			2,
		)?;
		Multisig::<T>::approve(RawOrigin::Signed(member1.clone()).into(), 1)?;
		Multisig::<T>::approve(RawOrigin::Signed(member2.clone()).into(), 1)?;

		#[extrinsic_call]
		reject(RawOrigin::Signed(member3.clone()), 1);

		let proposal_vote = Voting::<T>::get(1).unwrap();
		assert!(proposal_vote.nays.contains(&member3));
	}

	impl_benchmark_test_suite!(Multisig, crate::mock::new_test_ext(), crate::mock::Test);
}
