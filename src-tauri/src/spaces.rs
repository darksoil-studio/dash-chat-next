pub mod store;

use p2panda_auth::traits::Conditions;
use p2panda_spaces::manager::Manager;
use p2panda_spaces::space::SpaceError;
use p2panda_spaces::traits::SpaceId;
use p2panda_spaces::types::StrongRemoveResolver;

use crate::chat::ChatId;
use crate::forge::DashForge;
use crate::operation::GroupControlMessage;
use crate::spaces::store::TestStore;

pub type TestConditions = ();

impl SpaceId for ChatId {}

pub type DashManager = Manager<
    ChatId,
    TestStore,
    DashForge,
    GroupControlMessage,
    TestConditions,
    StrongRemoveResolver<TestConditions>,
>;

pub type TestSpaceError = SpaceError<
    ChatId,
    TestStore,
    DashForge,
    GroupControlMessage,
    TestConditions,
    StrongRemoveResolver<TestConditions>,
>;
