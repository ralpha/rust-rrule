use super::{datetime::DateTime, rrule::RRule};
use crate::{parser::build_rruleset, RRuleError};
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct RRuleSet {
    pub rrule: Vec<RRule>,
    pub rdate: Vec<DateTime>,
    pub exrule: Vec<RRule>,
    pub exdate: Vec<DateTime>,
    pub dt_start: Option<DateTime>,
}

impl RRuleSet {
    pub fn rrule(&mut self, rrule: RRule) {
        self.rrule.push(rrule);
    }

    pub fn exrule(&mut self, rrule: RRule) {
        self.exrule.push(rrule);
    }

    pub fn rdate(&mut self, rdate: DateTime) {
        self.rdate.push(rdate);
    }

    pub fn exdate(&mut self, exdate: DateTime) {
        self.exdate.push(exdate);
    }

    /// Returns all the recurrences of the rruleset.
    /// Limit must be set in order to prevent infinite loops.
    /// The max limit is `65535`. If you need more please use `into_iter` directly.
    pub fn all(&self, limit: u16) -> Vec<DateTime> {
        self.into_iter().take(limit as usize).collect()
    }

    // /// TODO: **Work in progress**
    // /// Returns all the recurrences of the rrule.
    // /// Limit must be set in order to prevent infinite loops.
    // /// The max limit is `65535`. If you need more please use `into_iter` directly.
    // ///
    // /// In case where the iterator ended with an errors the error will be included,
    // /// otherwise the second value of the return tuple will be `None`.
    // pub fn all_with_error(&self, _limit: u16) -> (Vec<DateTime>, Option<RRuleError>) {
    //     let mut iterator = self.into_iter();
    //     let mut list = vec![];
    //     let err = None;
    //     for _i in 0..limit {
    //         let next = iterator.next();
    //         match next {
    //             Some(value) => list.push(value),
    //             None => {
    //                 // TODO add error handling in RRuleSetIter
    //                 // if iterator.has_err() {
    //                 //     err = iterator.get_err().clone();
    //                 // }
    //                 break;
    //             }
    //         }
    //     }
    //     (list, err)
    // }

    /// Returns the last recurrence before the given datetime instance.
    /// The inc keyword defines what happens if dt is an recurrence.
    /// With inc == true, if dt itself is an recurrence, it will be returned.
    pub fn before(&self, dt: DateTime, inc: bool) -> Option<DateTime> {
        self.into_iter()
            .take_while(|d| if inc { *d <= dt } else { *d < dt })
            .last()
    }

    /// Returns the last recurrence after the given datetime instance.
    /// The inc keyword defines what happens if dt is an recurrence.
    /// With inc == true, if dt itself is an recurrence, it will be returned.
    pub fn after(&self, dt: DateTime, inc: bool) -> Option<DateTime> {
        self.into_iter()
            .find(|d| !(if inc { *d <= dt } else { *d < dt }))
    }

    /// Returns all the recurrences of the rrule between after and before.
    /// The inc keyword defines what happens if after and/or before are
    /// themselves recurrences. With inc == true, they will be included in the
    /// list, if they are found in the recurrence set.
    pub fn between(&self, after: DateTime, before: DateTime, inc: bool) -> Vec<DateTime> {
        self.into_iter()
            .skip_while(|d| if inc { *d <= after } else { *d < after })
            .take_while(|d| if inc { *d <= before } else { *d < before })
            .collect()
    }
}

impl FromStr for RRuleSet {
    type Err = RRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        build_rruleset(s)
    }
}
