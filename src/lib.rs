//! SigmaOS-access: access control model.
//!
//! Real implementation: users, permissions, and validation.
//! Replaces the 6-line stub.

use std::collections::BTreeMap;
use std::fmt;

/// Access permission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPermission {
    pub id: String,
    pub name: String,
    pub resource: String,
    pub level: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessError {
    EmptyName,
    DuplicatePermission(String),
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "permission name must not be empty"),
            Self::DuplicatePermission(id) => write!(f, "duplicate permission {:?}", id),
        }
    }
}

impl std::error::Error for AccessError {}

/// Access control manager.
pub struct AccessControl {
    pub name: String,
    permissions: BTreeMap<String, AccessPermission>,
}

impl AccessControl {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            permissions: BTreeMap::new(),
        }
    }

    pub fn add_permission(&mut self, permission: AccessPermission) -> Result<(), AccessError> {
        if permission.name.trim().is_empty() { return Err(AccessError::EmptyName); }
        let id = permission.id.clone();
        if self.permissions.contains_key(&id) {
            return Err(AccessError::DuplicatePermission(id.clone()));
        }
        self.permissions.insert(id, permission);
        Ok(())
    }

    pub fn permission_count(&self) -> usize { self.permissions.len() }

    pub fn get_permission(&self, id: &str) -> Option<&AccessPermission> { self.permissions.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_create_aaa() {
        let a = AccessControl::new("My Access");
        assert_eq!(a.name, "My Access");
        assert_eq!(a.permission_count(), 0);
    }

    #[test]
    fn test_add_permission_aaa() {
        let mut a = AccessControl::new("My Access");
        let p = AccessPermission { id: "p1".into(), name: "read".into(), resource: "file".into(), level: "user".into() };
        a.add_permission(p).unwrap();
        assert_eq!(a.permission_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut a = AccessControl::new("My Access");
        let p = AccessPermission { id: "x".into(), name: "".into(), resource: "a".into(), level: "b".into() };
        assert!(a.add_permission(p).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = AccessControl::new("My Access");
        let p = AccessPermission { id: "x".into(), name: "X".into(), resource: "a".into(), level: "b".into() };
        a.add_permission(p).unwrap();
        let p2 = AccessPermission { id: "x".into(), name: "X".into(), resource: "a".into(), level: "b".into() };
        assert!(a.add_permission(p2).is_err());
    }
}
