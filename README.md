# SigmaOS-access

Access control and permission primitives for SigmaOS.

## LDAP integration status

`LdapLightweightDirectoryEngine` currently stores in-memory fixture entries; it
does not connect to or authenticate against an LDAP server. Credential binds
therefore return an explicit unavailable error and do not expose fixture
records as authenticated directory results. Do not use this type as an
authentication provider until a real, TLS-protected LDAP backend is added.

## Development

```sh
cargo check
```
