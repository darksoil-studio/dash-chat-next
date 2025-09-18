mod control_message;
mod store;

pub use control_message::*;
pub use store::*;

use p2panda_spaces::manager::Manager;
use p2panda_spaces::space::SpaceError;
use p2panda_spaces::traits::SpaceId;
use p2panda_spaces::types::StrongRemoveResolver;

use crate::chat::ChatId;
use crate::forge::DashForge;

pub type TestConditions = ();

impl SpaceId for ChatId {}

pub type DashSpace = p2panda_spaces::space::Space<
    ChatId,
    TestStore,
    DashForge,
    SpaceControlMessage,
    TestConditions,
    StrongRemoveResolver<TestConditions>,
>;

pub type DashGroup = p2panda_spaces::group::Group<
    ChatId,
    TestStore,
    DashForge,
    SpaceControlMessage,
    TestConditions,
    StrongRemoveResolver<TestConditions>,
>;

pub type DashManager = Manager<
    ChatId,
    TestStore,
    DashForge,
    SpaceControlMessage,
    TestConditions,
    StrongRemoveResolver<TestConditions>,
>;

pub type TestSpaceError = SpaceError<
    ChatId,
    TestStore,
    DashForge,
    SpaceControlMessage,
    TestConditions,
    StrongRemoveResolver<TestConditions>,
>;
