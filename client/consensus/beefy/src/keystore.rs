// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use sp_application_crypto::RuntimeAppPublic;
use sp_core::keccak_256;
use sp_keystore::KeystorePtr;

use log::warn;

use sp_consensus_beefy::{
	crypto::{Public, Signature},
	BeefyAuthorityId, KEY_TYPE,
};

use crate::{error, LOG_TARGET};

/// Hasher used for BEEFY signatures.
pub(crate) type BeefySignatureHasher = sp_runtime::traits::Keccak256;

/// A BEEFY specific keystore implemented as a `Newtype`. This is basically a
/// wrapper around [`sp_keystore::Keystore`] and allows to customize
/// common cryptographic functionality.
pub(crate) struct BeefyKeystore(Option<KeystorePtr>);

impl BeefyKeystore {
	/// Check if the keystore contains a private key for one of the public keys
	/// contained in `keys`. A public key with a matching private key is known
	/// as a local authority id.
	///
	/// Return the public key for which we also do have a private key. If no
	/// matching private key is found, `None` will be returned.
	pub fn authority_id(&self, keys: &[Public]) -> Option<Public> {
		let store = self.0.clone()?;

		// we do check for multiple private keys as a key store sanity check.
		let public: Vec<Public> = keys
			.iter()
			.filter(|k| store.has_keys(&[(k.to_raw_vec(), KEY_TYPE)]))
			.cloned()
			.collect();

		if public.len() > 1 {
			warn!(
				target: LOG_TARGET,
				"🥩 Multiple private keys found for: {:?} ({})",
				public,
				public.len()
			);
		}

		public.get(0).cloned()
	}

	/// Sign `message` with the `public` key.
	///
	/// Note that `message` usually will be pre-hashed before being signed.
	///
	/// Return the message signature or an error in case of failure.
	pub fn sign(&self, public: &Public, message: &[u8]) -> Result<Signature, error::Error> {
		let store = self.0.clone().ok_or_else(|| error::Error::Keystore("no Keystore".into()))?;

		let msg = keccak_256(message);
		let public = public.as_ref();

		let sig = store
			.ecdsa_sign_prehashed(KEY_TYPE, public, &msg)
			.map_err(|e| error::Error::Keystore(e.to_string()))?
			.ok_or_else(|| error::Error::Signature("ecdsa_sign_prehashed() failed".to_string()))?;

		// check that `sig` has the expected result type
		let sig = sig.clone().try_into().map_err(|_| {
			error::Error::Signature(format!("invalid signature {:?} for key {:?}", sig, public))
		})?;

		Ok(sig)
	}

	/// Returns a vector of [`sp_consensus_beefy::crypto::Public`] keys which are currently
	/// supported (i.e. found in the keystore).
	pub fn public_keys(&self) -> Result<Vec<Public>, error::Error> {
		let store = self.0.clone().ok_or_else(|| error::Error::Keystore("no Keystore".into()))?;

		let pk: Vec<Public> =
			store.ecdsa_public_keys(KEY_TYPE).drain(..).map(Public::from).collect();

		Ok(pk)
	}

	/// Use the `public` key to verify that `sig` is a valid signature for `message`.
	///
	/// Return `true` if the signature is authentic, `false` otherwise.
	pub fn verify(public: &Public, sig: &Signature, message: &[u8]) -> bool {
		BeefyAuthorityId::<BeefySignatureHasher>::verify(public, sig, message)
	}
}

impl From<Option<KeystorePtr>> for BeefyKeystore {
	fn from(store: Option<KeystorePtr>) -> BeefyKeystore {
		BeefyKeystore(store)
	}
}

#[cfg(test)]
pub mod tests {
	use sp_consensus_beefy::{crypto, Keyring};
	use sp_core::{ecdsa, Pair};
	use sp_keystore::testing::MemoryKeystore;

	use super::*;
	use crate::error::Error;

	fn keystore() -> KeystorePtr {
		MemoryKeystore::new().into()
	}

	#[test]
	fn verify_should_work() {
		let msg = keccak_256(b"I am Alice!");
		let sig = Keyring::Alice.sign(b"I am Alice!");

		assert!(ecdsa::Pair::verify_prehashed(
			&sig.clone().into(),
			&msg,
			&Keyring::Alice.public().into(),
		));

		// different public key -> fail
		assert!(!ecdsa::Pair::verify_prehashed(
			&sig.clone().into(),
			&msg,
			&Keyring::Bob.public().into(),
		));

		let msg = keccak_256(b"I am not Alice!");

		// different msg -> fail
		assert!(
			!ecdsa::Pair::verify_prehashed(&sig.into(), &msg, &Keyring::Alice.public().into(),)
		);
	}

	#[test]
	fn pair_works() {
		let want = crypto::Pair::from_string("//Alice", None).expect("Pair failed").to_raw_vec();
		let got = Keyring::Alice.pair().to_raw_vec();
		assert_eq!(want, got);

		let want = crypto::Pair::from_string("//Bob", None).expect("Pair failed").to_raw_vec();
		let got = Keyring::Bob.pair().to_raw_vec();
		assert_eq!(want, got);

		let want = crypto::Pair::from_string("//Charlie", None).expect("Pair failed").to_raw_vec();
		let got = Keyring::Charlie.pair().to_raw_vec();
		assert_eq!(want, got);

		let want = crypto::Pair::from_string("//Dave", None).expect("Pair failed").to_raw_vec();
		let got = Keyring::Dave.pair().to_raw_vec();
		assert_eq!(want, got);

		let want = crypto::Pair::from_string("//Eve", None).expect("Pair failed").to_raw_vec();
		let got = Keyring::Eve.pair().to_raw_vec();
		assert_eq!(want, got);

		let want = crypto::Pair::from_string("//Ferdie", None).expect("Pair failed").to_raw_vec();
		let got = Keyring::Ferdie.pair().to_raw_vec();
		assert_eq!(want, got);

		let want = crypto::Pair::from_string("//One", None).expect("Pair failed").to_raw_vec();
		let got = Keyring::One.pair().to_raw_vec();
		assert_eq!(want, got);

		let want = crypto::Pair::from_string("//Two", None).expect("Pair failed").to_raw_vec();
		let got = Keyring::Two.pair().to_raw_vec();
		assert_eq!(want, got);
	}

	#[test]
	fn authority_id_works() {
		let store = keystore();

		let alice: crypto::Public = store
			.ecdsa_generate_new(KEY_TYPE, Some(&Keyring::Alice.to_seed()))
			.ok()
			.unwrap()
			.into();

		let bob = Keyring::Bob.public();
		let charlie = Keyring::Charlie.public();

		let store: BeefyKeystore = Some(store).into();

		let mut keys = vec![bob, charlie];

		let id = store.authority_id(keys.as_slice());
		assert!(id.is_none());

		keys.push(alice.clone());

		let id = store.authority_id(keys.as_slice()).unwrap();
		assert_eq!(id, alice);
	}

	#[test]
	fn sign_works() {
		let store = keystore();

		let alice: crypto::Public = store
			.ecdsa_generate_new(KEY_TYPE, Some(&Keyring::Alice.to_seed()))
			.ok()
			.unwrap()
			.into();

		let store: BeefyKeystore = Some(store).into();

		let msg = b"are you involved or committed?";

		let sig1 = store.sign(&alice, msg).unwrap();
		let sig2 = Keyring::Alice.sign(msg);

		assert_eq!(sig1, sig2);
	}

	#[test]
	fn sign_error() {
		let store = keystore();

		store.ecdsa_generate_new(KEY_TYPE, Some(&Keyring::Bob.to_seed())).ok().unwrap();

		let store: BeefyKeystore = Some(store).into();

		let alice = Keyring::Alice.public();

		let msg = b"are you involved or committed?";
		let sig = store.sign(&alice, msg).err().unwrap();
		let err = Error::Signature("ecdsa_sign_prehashed() failed".to_string());

		assert_eq!(sig, err);
	}

	#[test]
	fn sign_no_keystore() {
		let store: BeefyKeystore = None.into();

		let alice = Keyring::Alice.public();
		let msg = b"are you involved or committed";

		let sig = store.sign(&alice, msg).err().unwrap();
		let err = Error::Keystore("no Keystore".to_string());
		assert_eq!(sig, err);
	}

	#[test]
	fn verify_works() {
		let store = keystore();

		let alice: crypto::Public = store
			.ecdsa_generate_new(KEY_TYPE, Some(&Keyring::Alice.to_seed()))
			.ok()
			.unwrap()
			.into();

		let store: BeefyKeystore = Some(store).into();

		// `msg` and `sig` match
		let msg = b"are you involved or committed?";
		let sig = store.sign(&alice, msg).unwrap();
		assert!(BeefyKeystore::verify(&alice, &sig, msg));

		// `msg and `sig` don't match
		let msg = b"you are just involved";
		assert!(!BeefyKeystore::verify(&alice, &sig, msg));
	}

	// Note that we use keys with and without a seed for this test.
	#[test]
	fn public_keys_works() {
		const TEST_TYPE: sp_application_crypto::KeyTypeId =
			sp_application_crypto::KeyTypeId(*b"test");

		let store = keystore();

		let add_key =
			|key_type, seed: Option<&str>| store.ecdsa_generate_new(key_type, seed).unwrap();

		// test keys
		let _ = add_key(TEST_TYPE, Some(Keyring::Alice.to_seed().as_str()));
		let _ = add_key(TEST_TYPE, Some(Keyring::Bob.to_seed().as_str()));

		let _ = add_key(TEST_TYPE, None);
		let _ = add_key(TEST_TYPE, None);

		// BEEFY keys
		let _ = add_key(KEY_TYPE, Some(Keyring::Dave.to_seed().as_str()));
		let _ = add_key(KEY_TYPE, Some(Keyring::Eve.to_seed().as_str()));

		let key1: crypto::Public = add_key(KEY_TYPE, None).into();
		let key2: crypto::Public = add_key(KEY_TYPE, None).into();

		let store: BeefyKeystore = Some(store).into();

		let keys = store.public_keys().ok().unwrap();

		assert!(keys.len() == 4);
		assert!(keys.contains(&Keyring::Dave.public()));
		assert!(keys.contains(&Keyring::Eve.public()));
		assert!(keys.contains(&key1));
		assert!(keys.contains(&key2));
	}
}
