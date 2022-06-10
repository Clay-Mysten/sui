// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use sui_types::base_types::SuiAddress;
use sui_types::event::{Event, EventEnvelope};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize)]
pub enum EventFilter {
    ByPackage(AccountAddress),
    ByModule(Identifier),
    ByFunction(Identifier),
    ByEventType(StructTag),
    ByEventFields(BTreeMap<String, String>),
    ByAddress(SuiAddress),
    And(Box<EventFilter>, Box<EventFilter>),
}
impl EventFilter {
    fn try_matches(&self, item: &EventEnvelope) -> Result<bool, anyhow::Error> {
        Ok(match self {
            EventFilter::ByEventType(event_type) => match &item.event {
                Event::MoveEvent(event_obj) => &event_obj.type_ == event_type,
                _ => false,
            },
            EventFilter::ByEventFields(fields_filter) => {
                if let Some(json) = &item.move_struct_json_value {
                    for (pointer, value) in fields_filter {
                        if let Some(v) = json.pointer(pointer) {
                            if &v.to_string() != value {
                                return Ok(false);
                            }
                        } else {
                            return Ok(false);
                        }
                    }
                    true
                } else {
                    false
                }
            }
            // TODO: Implement the rest
            EventFilter::ByAddress(_) => true,
            EventFilter::ByPackage(_) => true,
            EventFilter::ByModule(_) => true,
            EventFilter::ByFunction(_) => true,
            EventFilter::And(filter_a, filter_b) => {
                filter_a.matches(item) && filter_b.matches(item)
            }
        })
    }

    pub fn and(self, other_filter: EventFilter) -> Self {
        Self::And(Box::new(self), Box::new(other_filter))
    }
}

impl Filter<EventEnvelope> for EventFilter {
    fn matches(&self, item: &EventEnvelope) -> bool {
        self.try_matches(item).unwrap_or_default()
    }
}

pub trait Filter<T> {
    fn matches(&self, item: &T) -> bool;
}
