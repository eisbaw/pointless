# ADR-003: Main Function Decomposition

**Date**: 2025-09-02  
**Status**: Implemented  
**Context**: MPED Architecture Review - Monolithic Function Concerns  

## Problem

The `main()` function was 176 lines and handled multiple responsibilities:
1. Configuration parsing and validation
2. Logging initialization  
3. X11 client setup and testing
4. Component initialization (input handler, renderer, grid manager)
5. Signal handling setup
6. Main event loop processing
7. Cleanup on shutdown

This violated MPED principles of single responsibility and made the code harder to:
- Test individual components
- Understand the application flow
- Debug initialization issues
- Maintain over time

## Decision

**Decomposed the monolithic main function into focused, single-responsibility functions with clear data flow.**

### Architecture Changes:

```rust
// Before: 176-line monolithic function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 176 lines of mixed concerns...
}

// After: Focused functions with clear responsibilities
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    initialize_logging(&config);
    initialize_and_run_application(config)
}

fn initialize_logging(config: &Config) { /* ... */ }
fn initialize_and_run_application(config: Config) -> Result<...> {
    let app_state = initialize_application_components(config)?;
    run_event_loop(app_state)
}
fn initialize_application_components(config: Config) -> Result<ApplicationState, ...> { /* ... */ }
fn run_event_loop(mut app_state: ApplicationState) -> Result<...> { /* ... */ }
fn handle_input_event(input_event: InputEvent, app_state: &mut ApplicationState) -> Result<...> { /* ... */ }
fn cleanup_application(app_state: &mut ApplicationState) { /* ... */ }
```

## Rationale

1. **MPED Single Responsibility**: Each function has one clear purpose
2. **Testability**: Individual components can be tested in isolation
3. **Maintainability**: Easier to locate and fix issues in specific phases
4. **Readability**: Application flow is clear from function names
5. **Error Handling**: Focused error contexts for each phase

## Implementation Details

### ApplicationState Struct
Created a struct to hold all initialized components, avoiding parameter passing:

```rust
struct ApplicationState {
    config: Config,
    x11_client: X11Client,
    input_handler: InputHandler,
    renderer: Renderer,  
    grid_manager: GridManager,
    screen_info: ScreenInfo,
}
```

### Function Responsibilities:

1. **`initialize_logging`**: Debug/info logging setup based on config
2. **`initialize_application_components`**: X11 client, input handler, renderer, grid manager setup
3. **`test_cursor_control`**: Debug mode cursor movement testing (extracted)
4. **`run_event_loop`**: Signal handling and main event processing loop
5. **`setup_signal_handling`**: SIGINT/SIGTERM graceful shutdown (extracted)
6. **`handle_input_event`**: Process individual input events (extracted)
7. **`cleanup_application`**: Resource cleanup and shutdown (extracted)

## Consequences

### Positive:
- **Cleaner Architecture**: Each function has single responsibility
- **Better Error Context**: Errors occur in focused contexts
- **Improved Testability**: Individual phases can be unit tested
- **Easier Debugging**: Stack traces point to specific functionality
- **Better Code Navigation**: IDE can jump to specific functionality

### Negative:
- **More Functions**: 7 functions instead of 1 (but each is focused)
- **State Management**: ApplicationState struct needed for component passing
- **Slightly More Lines**: Total LOC increased but individual functions are smaller

## Verification

- All tests pass (20/20)
- Application builds and runs correctly
- Event loop functions identically to before
- Signal handling works properly
- Cleanup occurs on shutdown
- Debug mode cursor testing works

## Future Benefits

This refactoring enables:
- Individual component testing
- Easier addition of new initialization steps
- Better error handling per phase
- Cleaner integration testing
- Potential extraction to application library crate