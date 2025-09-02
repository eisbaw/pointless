# ADR-001: Interface Consistency in GridLayout Trait

**Date**: 2025-09-02  
**Status**: Implemented  
**Context**: MPED Architecture Review  

## Problem

The `GridLayout` trait had two methods for processing keys:
- `process_key(key, current_level)` - basic version
- `process_key_with_screen(key, current_level, screen, density)` - extended version with screen context

This created a leaky abstraction where:
1. Two different interfaces did similar things
2. Grid implementations had to implement both methods
3. GridManager had to know about internal implementation details
4. Interface inconsistency violated MPED principles of clean abstractions

## Decision

**Unified the interface by removing the dual methods and making `process_key` always receive screen information.**

### Changes Made:
- Removed `process_key_with_screen` method from trait
- Updated `process_key` signature to always include `screen: &ScreenInfo` and `density: u32`
- Updated all grid implementations to use the unified interface
- Updated all test calls to use the new interface
- GridManager now calls the simplified, consistent interface

## Rationale

1. **MPED Compliance**: Single, clear interface with no leaky abstractions
2. **Consistency**: All grids receive the same information, enabling better grid logic
3. **Future-Proofing**: Screen-aware grids can make better decisions with full context
4. **Simplicity**: Removes the need to understand two different interfaces

## Implementation Details

```rust
// Before (confusing dual interface)
trait GridLayout {
    fn process_key(&mut self, key: KeyCode, level: u32) -> Option<GridCoord>;
    fn process_key_with_screen(&mut self, key: KeyCode, level: u32, 
                              screen: &ScreenInfo, density: u32) -> Option<GridCoord>;
}

// After (clean single interface)  
trait GridLayout {
    fn process_key(&mut self, key: KeyCode, level: u32, 
                   screen: &ScreenInfo, density: u32) -> Option<GridCoord>;
}
```

## Consequences

### Positive:
- Cleaner, more consistent interface
- All grids have access to screen context for better decisions
- Removes architectural debt identified in MPED review
- Simpler mental model for developers

### Negative:
- Breaking change (but internal to the crate)
- All grid implementations required updates
- All tests required updates

## Verification

- All unit tests pass (20/20)
- All grid implementations successfully updated
- Interface is now consistent across all grids
- GridManager code is simpler and cleaner