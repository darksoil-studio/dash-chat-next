use p2panda_core::PrivateKey;
use p2panda_encryption::{
    crypto::x25519::SecretKey, key_bundle::Lifetime, key_manager::KeyManager, Rng,
};
use p2panda_spaces::{auth::orderer::AuthOrderer, types::AuthGroupState, ActorId};

use super::*;

pub type TestStore =
    p2panda_spaces::test_utils::MemoryStore<ChatId, GroupControlMessage, TestConditions>;

pub fn create_test_store(private_key: PrivateKey) -> TestStore {
    let rng = Rng::default();

    let my_id: ActorId = private_key.public_key().into();

    let key_manager_y = {
        let identity_secret = SecretKey::from_bytes(rng.random_array().unwrap());
        KeyManager::init(&identity_secret, Lifetime::default(), &rng).unwrap()
    };

    let orderer_y = AuthOrderer::init();
    let auth_y = AuthGroupState::new(orderer_y);
    let store = TestStore::new(my_id, key_manager_y, auth_y);

    store
}
