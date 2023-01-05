// Copyright (c) Sean Lawlor
//
// This source code is licensed under both the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree.

//! Actor error types

use std::fmt::Display;

/// Spawn errors starting an actor
#[derive(Debug)]
pub enum SpawnErr {
    /// Actor panic'd during startup
    StartupPanic(String),
    /// Actor failed to startup because the startup task was cancelled
    StartupCancelled,
    /// An actor cannot be started > 1 time
    ActorAlreadyStarted,
}

impl Display for SpawnErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartupPanic(panic_msg) => {
                write!(f, "Actor panicked during startup '{}'", panic_msg)
            }
            Self::StartupCancelled => {
                write!(
                    f,
                    "Actor failed to startup due to processing task being cancelled"
                )
            }
            Self::ActorAlreadyStarted => {
                write!(f, "Actor cannot be re-started more than once")
            }
        }
    }
}

/// Actor processing loop errors
#[derive(Debug)]
pub enum ActorErr {
    /// Actor had a task cancelled internally during processing
    Cancelled,
    /// Actor had an internal panic
    Panic(String),
}

impl Display for ActorErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Panic(panic_msg) => {
                write!(f, "Actor panicked '{}'", panic_msg)
            }
            Self::Cancelled => {
                write!(f, "Actor operation cancelled")
            }
        }
    }
}

/// A messaging error has occurred
#[derive(Debug)]
pub enum MessagingErr {
    /// The channel you're trying to send a message too has been dropped/closed.
    /// If you're sending to an [crate::ActorCell] then that means the actor has died
    /// (failure or not).
    ChannelClosed,
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for MessagingErr {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::ChannelClosed
    }
}

impl<T> From<tokio::sync::broadcast::error::SendError<T>> for MessagingErr {
    fn from(_: tokio::sync::broadcast::error::SendError<T>) -> Self {
        Self::ChannelClosed
    }
}

impl<T> From<tokio::sync::watch::error::SendError<T>> for MessagingErr {
    fn from(_: tokio::sync::watch::error::SendError<T>) -> Self {
        Self::ChannelClosed
    }
}

impl<T> From<tokio::sync::mpsc::error::TrySendError<T>> for MessagingErr {
    fn from(_: tokio::sync::mpsc::error::TrySendError<T>) -> Self {
        Self::ChannelClosed
    }
}

impl Display for MessagingErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChannelClosed => {
                write!(f, "Messaging failed because channel is closed")
            }
        }
    }
}
