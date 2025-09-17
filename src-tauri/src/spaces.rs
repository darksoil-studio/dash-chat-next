use std::convert::Infallible;
use std::hash::Hash as StdHash;
use std::marker::PhantomData;

use p2panda_auth::group::GroupMember;
use p2panda_auth::traits::Conditions;
use p2panda_auth::Access;
use p2panda_core::{Hash, PrivateKey, PublicKey};
use p2panda_encryption::crypto::x25519::SecretKey;
use p2panda_encryption::data_scheme::DirectMessage;
use p2panda_encryption::key_bundle::Lifetime;
use p2panda_encryption::key_manager::KeyManager;
use p2panda_encryption::Rng;

use p2panda_spaces::auth::orderer::AuthOrderer;
use p2panda_spaces::event::Event;
use p2panda_spaces::forge::Forge;
use p2panda_spaces::manager::Manager;
use p2panda_spaces::message::SpacesArgs;
use p2panda_spaces::space::SpaceError;
use p2panda_spaces::store::{AuthStore, SpaceStore};
use p2panda_spaces::traits::SpaceId;
use p2panda_spaces::types::{
    ActorId, AuthControlMessage, AuthGroupAction, AuthGroupState, EncryptionControlMessage,
    OperationId, StrongRemoveResolver,
};

use crate::forge::DashForge;
use crate::operation::GroupControlMessage;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct TestConditions {}

impl Conditions for TestConditions {}

// TODO: implement
// (empty enum is a way to make an uninhabitable type)
struct TestStore<ID> {
    _phantom: PhantomData<ID>,
}

type TestManager<ID> = Manager<
    ID,
    TestStore<ID>,
    DashForge,
    GroupControlMessage,
    TestConditions,
    StrongRemoveResolver<TestConditions>,
>;

type TestSpaceError<ID> = SpaceError<
    ID,
    TestStore<ID>,
    DashForge,
    GroupControlMessage,
    TestConditions,
    StrongRemoveResolver<TestConditions>,
>;
