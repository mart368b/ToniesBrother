use super::MvpState;
use std::convert::{AsRef, AsMut};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerState {
    MvpState(MvpState)
}

macro_rules! convert {
    ($($name:ident),*) => {$(
        impl AsRef<$name> for ServerState {
            #[allow(unreachable_patterns)]
            fn as_ref(&self) -> &$name {
                match self {
                    ServerState::$name(value) => &value,
                    _ => panic!("WRONG GETTER")
                }
            }
        }

        impl AsMut<$name> for ServerState {
            #[allow(unreachable_patterns)]
            fn as_mut(&mut self) -> &mut $name {
                match self {
                    ServerState::$name(ref mut value) => value,
                    _ => panic!("WRONG GETTER")
                }
            }
        }
        
        impl From<$name> for ServerState {
            fn from(value: $name) -> ServerState {
                ServerState::$name(value)
            }
        }
    )*};
}

convert!{
    MvpState
}