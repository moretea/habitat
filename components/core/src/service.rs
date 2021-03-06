// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::result;
use std::str::FromStr;

use regex::Regex;

use error::Error;

lazy_static! {
    static ref FROM_STR_RE: Regex =
        Regex::new(r"\A(?P<service>[^.]+)\.(?P<group>[^.@]+)(@(?P<organization>.+))?\z").unwrap();
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, RustcDecodable, RustcEncodable)]
pub struct ServiceGroup {
    pub service: String,
    pub group: String,
    pub organization: Option<String>,
}

impl ServiceGroup {
    pub fn new<S1, S2>(service: S1, group: S2, organization: Option<String>) -> Self
        where S1: Into<String>,
              S2: Into<String>
    {
        ServiceGroup {
            service: service.into(),
            group: group.into(),
            organization: organization,
        }
    }

    // returns ".org" if self.organization is Some, otherwise an empty string
    pub fn dotted_org_or_empty(&self) -> String {
        self.organization.as_ref().map_or("".to_string(), |s| format!(".{}", &s))
    }

    pub fn as_string(&self) -> String {
        // JW TODO: We shouldn't allocate a new string each time we call this. We should store the
        // string internally in the struct and return slices to the string.
        format!("{}", self)
    }
}

impl fmt::Display for ServiceGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.organization.as_ref() {
            Some(org) => write!(f, "{}.{}@{}", self.service, self.group, org),
            None => write!(f, "{}.{}", self.service, self.group),
        }
    }
}

impl FromStr for ServiceGroup {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let caps = match FROM_STR_RE.captures(value) {
            Some(c) => c,
            None => return Err(Error::InvalidServiceGroup(value.to_string())),
        };
        let service = match caps.name("service") {
            Some(s) => s.to_string(),
            None => return Err(Error::InvalidServiceGroup(value.to_string())),
        };
        let group = match caps.name("group") {
            Some(g) => g.to_string(),
            None => return Err(Error::InvalidServiceGroup(value.to_string())),
        };
        let organization = match caps.name("organization") {
            Some(o) => Some(o.to_string()),
            None => None,
        };

        Ok(ServiceGroup::new(service, group, organization))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::ServiceGroup;

    #[test]
    fn fmt_without_organization() {
        let sg = ServiceGroup::new("kayla", "album", None);
        assert_eq!(&sg.to_string(), "kayla.album");

        let sg = ServiceGroup::new("blue-ocean", "album-track", None);
        assert_eq!(&sg.to_string(), "blue-ocean.album-track");
    }

    #[test]
    fn fmt_with_organization() {
        let sg = ServiceGroup::new("kayla", "album", Some("flying_colors".to_string()));
        assert_eq!(&sg.to_string(), "kayla.album@flying_colors");

        let sg = ServiceGroup::new("blue-ocean", "album-track", Some("f-l_y".to_string()));
        assert_eq!(&sg.to_string(), "blue-ocean.album-track@f-l_y");
    }

    #[test]
    fn from_str_without_organization() {
        let expected = ServiceGroup::new("kayla", "album", None);
        let actual = ServiceGroup::from_str("kayla.album").unwrap();
        assert_eq!(expected, actual);

        let expected = ServiceGroup::new("blue-ocean", "track-from_album", None);
        let actual = ServiceGroup::from_str("blue-ocean.track-from_album").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_str_with_organization() {
        let expected = ServiceGroup::new("kayla", "album", Some("flying_colors".to_string()));
        let actual = ServiceGroup::from_str("kayla.album@flying_colors").unwrap();
        assert_eq!(expected, actual);

        let expected =
            ServiceGroup::new("blue-ocean", "track-from_album", Some("f-l_y".to_string()));
        let actual = ServiceGroup::from_str("blue-ocean.track-from_album@f-l_y").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic(expected = "not.allowed@")]
    fn from_str_ending_with_at() {
        ServiceGroup::from_str("not.allowed@").unwrap();
    }

    #[test]
    #[should_panic(expected = "only.one.period@allowed")]
    fn from_str_too_many_periods() {
        ServiceGroup::from_str("only.one.period@allowed").unwrap();
    }

    #[test]
    #[should_panic(expected = "oh-noes")]
    fn from_str_not_enough_periods() {
        ServiceGroup::from_str("oh-noes").unwrap();
    }

    #[test]
    fn service_groups_with_org() {
        let x = ServiceGroup::from_str("foo.bar").unwrap();
        assert!(x.service == "foo".to_string());
        assert!(x.group == "bar".to_string());
        assert!(x.organization.is_none());

        let y = ServiceGroup::from_str("foo.bar@baz").unwrap();
        assert!(y.service == "foo".to_string());
        assert!(y.group == "bar".to_string());
        assert!(y.organization.unwrap() == "baz");

        assert!(ServiceGroup::from_str("foo.bar@").is_err());
        assert!(ServiceGroup::from_str("f.oo.bar@baz").is_err());
        assert!(ServiceGroup::from_str("foo@baz").is_err());
    }

    #[test]
    fn org_or_empty() {
        assert!("" == ServiceGroup::from_str("foo.bar").unwrap().dotted_org_or_empty());
        assert!(".baz" == ServiceGroup::from_str("foo.bar@baz").unwrap().dotted_org_or_empty());
    }
}
