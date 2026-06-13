# Dependency Warning Analysis

## Issue

The following warning appears during builds:

```
warning: the following packages contain code that will be rejected by a future version of Rust: num-bigint-dig v0.8.4
```

## Root Cause Analysis

### Dependency Chain

```
mykobo-rs v0.1.0
└── jsonwebtoken v10.2.0
    └── rsa v0.9.8
        └── num-bigint-dig v0.8.4  ⚠️ (PROBLEMATIC VERSION)
```

### Specific Issue

The `num-bigint-dig v0.8.4` package contains code that uses the `vec!` macro in a way that will become a hard error in future Rust versions. Specifically:

- **Issue**: `macro 'vec' is private`
- **Rust Issue**: https://github.com/rust-lang/rust/issues/120192
- **Status**: Being phased out as a breaking change

### Affected Code Locations

The problematic code is in `num-bigint-dig v0.8.4`:
- `src/biguint.rs:490` - `BigUint::new(vec![1])`
- `src/biguint.rs:2005` - `vec![0]`
- `src/biguint.rs:2027` - `return vec![b'0']`
- `src/biguint.rs:2313` - `vec![0]`
- `src/prime.rs:138` - `vec![BigUint::zero(); prime_limit]`
- `src/bigrand.rs:319` - `vec![0u8; bytes_len]`

## Available Versions

| Package | Current | Latest Stable | Latest (including RC) |
|---------|---------|---------------|----------------------|
| `jsonwebtoken` | 10.2.0 | 10.2.0 | 10.2.0 |
| `rsa` | 0.9.8 | 0.9.8 | 0.10.0-rc.10 |
| `num-bigint-dig` | 0.8.4 | 0.9.1 | 0.9.1 |

## Solution Options

### Option 1: Wait for Upstream Update (Recommended)

The issue will be resolved when one of the following happens:

1. **`rsa` crate updates** to use `num-bigint-dig v0.9.1+`
   - Current: `rsa v0.9.8` depends on `num-bigint-dig v0.8.4`
   - Possible: `rsa v0.10.0` (currently RC) may already use a newer version
   - Repository: https://github.com/RustCrypto/RSA

2. **`jsonwebtoken` crate updates** to use newer `rsa` version
   - Current: `jsonwebtoken v10.2.0` depends on `rsa v0.9.8`
   - Repository: https://github.com/Keats/jsonwebtoken

**Action Items:**
- Monitor `jsonwebtoken` releases for updates
- Check if the issue is already resolved in `rsa v0.10.0-rc.10`

### Option 2: Use Cargo Patch (Temporary Workaround)

Force the use of a newer `num-bigint-dig` version using Cargo's `[patch]` section:

**Add to `Cargo.toml`:**
```toml
[patch.crates-io]
num-bigint-dig = "0.9.1"
```

**Pros:**
- Immediate resolution of the warning
- Simple configuration change

**Cons:**
- May cause compatibility issues if `rsa v0.9.8` isn't compatible with `num-bigint-dig v0.9.1`
- Requires testing to ensure no breaking changes
- Overrides dependency resolution for the entire project

### Option 3: Check rsa v0.10.0-rc.10

Investigate if using the release candidate of `rsa` resolves the issue:

```bash
# Check what num-bigint-dig version rsa v0.10.0-rc.10 uses
cargo info rsa@0.10.0-rc.10
```

If `rsa v0.10.0-rc.10` uses a newer `num-bigint-dig`, you could potentially:

1. Wait for `rsa v0.10.0` stable release
2. Wait for `jsonwebtoken` to update to `rsa v0.10.0`

## Current Impact

**Severity:** Low (Warning only, not an error)

**Current Status:**
- The code compiles and runs correctly
- This is a future compatibility warning
- No immediate action required for functionality
- Will become a compilation error in a future Rust version

**Timeline:**
- This warning was introduced in recent Rust versions
- Will become a hard error in a future Rust release (TBD)
- No specific deadline announced yet

## Recommendations

### Short Term (Current)
1. **Accept the warning** - It's only a warning and doesn't affect functionality
2. **Monitor upstream** - Watch for updates to `jsonwebtoken` and `rsa` crates
3. **Document the issue** - Keep this analysis for reference

### Medium Term (When available)
1. **Update `jsonwebtoken`** when a new version is released that uses updated dependencies
2. **Run tests** after any updates to ensure compatibility

### Long Term Prevention
1. **Regular dependency updates** - Run `cargo update` periodically
2. **Monitor security advisories** - Use tools like `cargo audit`
3. **Track upstream issues** - Subscribe to repositories for important dependencies

## Testing the Patch Option

If you want to test Option 2 (Cargo Patch), follow these steps:

1. Add the patch to `Cargo.toml`:
   ```toml
   [patch.crates-io]
   num-bigint-dig = "0.9.1"
   ```

2. Update dependencies:
   ```bash
   cargo update
   ```

3. Run all tests:
   ```bash
   cargo test --all
   ```

4. Check for any compatibility issues

5. If tests pass, the patch is safe to use temporarily

## Useful Commands

```bash
# Check dependency tree for num-bigint-dig
cargo tree -i num-bigint-dig

# Get future incompatibility details
cargo report future-incompatibilities --id 1

# Check for newer versions of dependencies
cargo update --dry-run

# Audit dependencies for security issues
cargo audit
```

## Related Links

- Rust Issue: https://github.com/rust-lang/rust/issues/120192
- num-bigint-dig Repository: https://github.com/dignifiedquire/num-bigint
- rsa Repository: https://github.com/RustCrypto/RSA
- jsonwebtoken Repository: https://github.com/Keats/jsonwebtoken

---

**Status:** Open - Waiting for upstream dependency updates
**Last Updated:** 2025-11-16
**Affected Version:** mykobo-rs v0.1.0
