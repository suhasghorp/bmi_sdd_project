# Tasks: BMI Calculator Web Service

**Input**: Design documents from `/specs/001-bmi-calculator/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are included following the Test-First Development principle from the project constitution.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

Single project structure at repository root:
- `src/` — source code
- `tests/` — test files
- `Cargo.toml` — dependencies and project config
- `Procfile` — Heroku deployment config

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create project structure with src/ and tests/ directories
- [X] T002 Initialize Cargo.toml with dependencies (axum, tokio, serde, serde_json, thiserror, anyhow, tracing, tracing-subscriber, clap) and dev dependencies (reqwest)
- [X] T003 [P] Configure Rust edition 2024 in Cargo.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core domain layer that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Domain Layer Tests (Test-First)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T004 [P] Create domain unit test module in src/domain.rs with #[cfg(test)] mod tests
- [X] T005 [P] Write unit test for BmiCategory classification at boundary 18.5 in src/domain.rs
- [X] T006 [P] Write unit test for BmiCategory classification at boundary 25.0 in src/domain.rs
- [X] T007 [P] Write unit test for BmiCategory classification at boundary 30.0 in src/domain.rs
- [X] T008 [P] Write unit test for calculate_bmi with valid inputs (70.0 kg, 1.75 m) in src/domain.rs
- [X] T009 [P] Write unit test for zero weight validation error in src/domain.rs
- [X] T010 [P] Write unit test for negative weight validation error in src/domain.rs
- [X] T011 [P] Write unit test for zero height validation error in src/domain.rs
- [X] T012 [P] Write unit test for negative height validation error in src/domain.rs

### Domain Layer Implementation

- [X] T013 [P] Implement BmiCategory enum with four variants in src/domain.rs
- [X] T014 [P] Implement Display trait for BmiCategory in src/domain.rs
- [X] T015 [P] Implement BmiError enum with thiserror derives in src/domain.rs
- [X] T016 [P] Implement BmiResult struct in src/domain.rs
- [X] T017 Implement calculate_bmi function with validation and WHO classification in src/domain.rs
- [X] T018 Verify all domain unit tests pass with cargo test

**Checkpoint**: Foundation ready - domain logic complete and tested. User story implementation can now begin.

---

## Phase 3: User Story 2 - Calculate BMI via API (Priority: P1)

**Goal**: Provide a JSON API endpoint that accepts weight and height, validates inputs, calculates BMI, and returns BMI value with category

**Independent Test**: Send a JSON POST request with weight and height and verify the JSON response contains correct BMI and category

**Why this comes first**: User Story 1 (web form) depends on this API endpoint via JavaScript fetch()

### Integration Tests for API (Test-First)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T019 [P] [US2] Create integration test file tests/api_integration.rs with test server setup
- [X] T020 [P] [US2] Write integration test for POST /api/bmi with valid inputs in tests/api_integration.rs
- [X] T021 [P] [US2] Write integration test for POST /api/bmi with zero weight returning 422 in tests/api_integration.rs
- [X] T022 [P] [US2] Write integration test for POST /api/bmi with negative height returning 422 in tests/api_integration.rs
- [X] T023 [P] [US2] Write integration test for GET /health returning 200 in tests/api_integration.rs
- [X] T024 [P] [US2] Write integration test for malformed JSON request in tests/api_integration.rs

### API Layer Implementation

- [X] T025 [P] [US2] Create BmiRequest struct with Deserialize in src/api.rs
- [X] T026 [P] [US2] Create BmiResponse struct with Serialize in src/api.rs
- [X] T027 [P] [US2] Create ErrorResponse struct with Serialize in src/api.rs
- [X] T028 [US2] Implement IntoResponse trait for BmiError in src/api.rs
- [X] T029 [US2] Implement POST /api/bmi handler with Json extractor in src/api.rs
- [X] T030 [P] [US2] Implement GET /health handler returning 200 OK in src/api.rs
- [X] T031 [US2] Create basic main.rs with Axum server setup and routing for API endpoints
- [X] T032 [US2] Verify all integration tests pass with cargo test

**Checkpoint**: User Story 2 is complete - API endpoint fully functional and tested independently

---

## Phase 4: User Story 1 - Calculate BMI via Web Form (Priority: P1)

**Goal**: Serve an embedded HTML page with a Bootstrap-styled form that allows users to enter weight and height, submits to the API, and displays BMI result with category

**Independent Test**: Open the web page, enter weight and height values, submit form, and verify correct BMI and category are displayed

**Depends on**: User Story 2 (API endpoint) must be complete

### UI Layer Implementation

- [X] T033 [P] [US1] Create embedded HTML constant with Bootstrap 5 CDN in src/ui.rs
- [X] T034 [P] [US1] Add JavaScript fetch() to POST form data to /api/bmi in src/ui.rs HTML
- [X] T035 [P] [US1] Add result display area for BMI value and category in src/ui.rs HTML
- [X] T036 [P] [US1] Add error display for validation failures in src/ui.rs HTML
- [X] T037 [US1] Implement GET / handler returning Html response in src/ui.rs
- [X] T038 [US1] Update main.rs routing to include GET / endpoint

**Checkpoint**: User Story 1 is complete - Web form fully functional and calls API independently

---

## Phase 5: User Story 3 - Input Validation with Clear Error Messages (Priority: P2)

**Goal**: Ensure all invalid inputs (zero/negative weight or height) are rejected with clear, descriptive error messages at domain level, with consistent behavior across API and web UI

**Independent Test**: Submit invalid values (zero weight, negative height) via API and verify appropriate 422 error responses are returned

**Note**: Domain-level validation is already implemented in Phase 2 (Foundational). This phase ensures comprehensive test coverage and error message quality.

### Additional Validation Tests

- [X] T039 [P] [US3] Add integration test for multiple validation scenarios in tests/api_integration.rs
- [X] T040 [P] [US3] Add unit test for edge case: very large weight values in src/domain.rs
- [X] T041 [P] [US3] Add unit test for edge case: very large height values in src/domain.rs
- [X] T042 [US3] Verify error messages are descriptive and match contract in contracts/api.md
- [X] T043 [US3] Run cargo test and verify 100% of invalid inputs are rejected per SC-003

**Checkpoint**: User Story 3 is complete - Validation comprehensively tested and error messages verified

---

## Phase 6: User Story 4 - Server Startup via CLI (Priority: P3)

**Goal**: Allow operators to start the server from command line with configurable address/port, environment variable support (PORT), and configurable logging verbosity

**Independent Test**: Start the server from command line and verify it binds to expected address and responds to requests

### CLI and Observability Implementation

- [X] T044 [P] [US4] Implement Clap CLI struct with port and address arguments in main.rs
- [X] T045 [US4] Implement PORT environment variable handling with precedence over CLI flag in main.rs
- [X] T046 [P] [US4] Setup tracing-subscriber with env_filter in main.rs
- [X] T047 [P] [US4] Add structured logging for server startup in main.rs
- [X] T048 [P] [US4] Add tracing spans for request handling in src/api.rs handlers
- [X] T049 [US4] Test server startup with custom port via CLI argument
- [X] T050 [US4] Test server startup with PORT environment variable

**Checkpoint**: User Story 4 is complete - CLI configuration and observability fully functional

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Deployment configuration, code quality, and final validation

- [X] T051 [P] Create Procfile for Heroku deployment with web: ./target/release/bmi_sdd
- [X] T052 [P] Run cargo clippy and address all warnings
- [X] T053 [P] Run cargo fmt to format all code
- [X] T054 Run full test suite with cargo test and verify all tests pass
- [X] T055 Validate quickstart.md commands (build, run, test, API curl examples)
- [X] T056 Verify SC-001: BMI calculation completes in under 1 second
- [X] T057 Verify SC-005: Server starts and is ready within 3 seconds

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 2 (Phase 3)**: Depends on Foundational completion - P1 priority, implements API
- **User Story 1 (Phase 4)**: Depends on User Story 2 completion (web form calls API) - P1 priority
- **User Story 3 (Phase 5)**: Depends on Foundational and User Story 2 completion - P2 priority, validation already in domain
- **User Story 4 (Phase 6)**: Depends on User Story 2 completion - P3 priority, enhances server startup
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 1 (P1)**: Depends on User Story 2 - Web form POSTs to /api/bmi endpoint
- **User Story 3 (P2)**: Validation is in Foundational layer, additional tests depend on API being complete
- **User Story 4 (P3)**: Can start after User Story 2 - Enhances main.rs with CLI/tracing

### Within Each Phase

- Tasks marked [P] can run in parallel (different files or independent modules)
- Tests should be written BEFORE implementation (test-first)
- Run cargo test after each implementation phase to verify tests pass
- Integration tests require complete API implementation

### Parallel Opportunities

**Setup Phase**: T001, T002, T003 can run sequentially (T003 depends on T002 for Cargo.toml)

**Foundational Phase**:
- Tests T004-T012 can all run in parallel (independent test functions)
- Implementation T013-T016 can run in parallel (independent types)
- T017 depends on T013-T016 (uses BmiCategory, BmiError, BmiResult)

**User Story 2**:
- Tests T019-T024 can run in parallel (independent test functions)
- Types T025-T027 can run in parallel (independent struct definitions)
- Handlers T029-T030 can be implemented in parallel (different routes)

**User Story 1**:
- HTML constant T033-T036 are all editing the same constant (sequential)
- T037 and T038 are sequential (T038 adds route for T037)

**User Story 3**:
- Tests T039-T041 can run in parallel

**User Story 4**:
- T044, T046, T047 can run in parallel (independent features in main.rs)
- T048 is in api.rs (parallel with main.rs work)

**Polish Phase**:
- T051, T052, T053 can run in parallel (different files/concerns)

---

## Parallel Example: Foundational Phase (Domain Tests)

```bash
# Launch all domain unit tests together:
Task: "Write unit test for BmiCategory classification at boundary 18.5 in src/domain.rs"
Task: "Write unit test for BmiCategory classification at boundary 25.0 in src/domain.rs"
Task: "Write unit test for BmiCategory classification at boundary 30.0 in src/domain.rs"
Task: "Write unit test for calculate_bmi with valid inputs in src/domain.rs"
Task: "Write unit test for zero weight validation error in src/domain.rs"
Task: "Write unit test for negative weight validation error in src/domain.rs"
Task: "Write unit test for zero height validation error in src/domain.rs"
Task: "Write unit test for negative height validation error in src/domain.rs"

# Then launch all domain implementations together:
Task: "Implement BmiCategory enum with four variants in src/domain.rs"
Task: "Implement Display trait for BmiCategory in src/domain.rs"
Task: "Implement BmiError enum with thiserror derives in src/domain.rs"
Task: "Implement BmiResult struct in src/domain.rs"
```

---

## Parallel Example: User Story 2 (API Integration Tests)

```bash
# Launch all API integration tests together:
Task: "Write integration test for POST /api/bmi with valid inputs in tests/api_integration.rs"
Task: "Write integration test for POST /api/bmi with zero weight returning 422 in tests/api_integration.rs"
Task: "Write integration test for POST /api/bmi with negative height returning 422 in tests/api_integration.rs"
Task: "Write integration test for GET /health returning 200 in tests/api_integration.rs"
Task: "Write integration test for malformed JSON request in tests/api_integration.rs"

# Then launch all API type definitions together:
Task: "Create BmiRequest struct with Deserialize in src/api.rs"
Task: "Create BmiResponse struct with Serialize in src/api.rs"
Task: "Create ErrorResponse struct with Serialize in src/api.rs"
```

---

## Implementation Strategy

### MVP First (User Stories 2 + 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 2 (API endpoint)
4. Complete Phase 4: User Story 1 (Web form)
5. **STOP and VALIDATE**: Test both stories independently
6. Deploy/demo if ready (full BMI calculator with API + web UI)

### Incremental Delivery

1. **Foundation**: Setup (Phase 1) + Foundational (Phase 2) → Domain logic complete and tested
2. **API MVP**: Add User Story 2 → Test independently → API endpoint working
3. **Web UI**: Add User Story 1 → Test independently → Complete web experience
4. **Validation Polish**: Add User Story 3 → Comprehensive validation testing
5. **Production Ready**: Add User Story 4 → CLI configuration and observability
6. **Deploy**: Complete Polish phase → Ready for Heroku deployment

Each increment adds value without breaking previous functionality.

### Single Developer Strategy

Follow phases sequentially:
1. Setup → Foundational (write tests, implement domain, verify)
2. User Story 2 (write integration tests, implement API, verify)
3. User Story 1 (implement UI, verify with manual testing)
4. User Story 3 (add validation tests, verify error handling)
5. User Story 4 (add CLI and tracing, verify startup)
6. Polish (Heroku config, code quality, final validation)

Stop at any checkpoint to validate independently before proceeding.

---

## Notes

- [P] tasks = different files or independent modules, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- **Test-first**: Write tests before implementation, verify they fail, then implement, verify they pass
- Run `cargo test` after each implementation phase
- Run `cargo clippy` and `cargo fmt` regularly
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Constitution principles: Domain Purity, Clean Layer Separation, Test-First Development, Stateless Design, Observability, Simplicity

---

## Task Summary

- **Total Tasks**: 57
- **Phase 1 (Setup)**: 3 tasks
- **Phase 2 (Foundational)**: 15 tasks (9 tests + 6 implementation)
- **Phase 3 (User Story 2 - API)**: 14 tasks (6 tests + 8 implementation)
- **Phase 4 (User Story 1 - Web UI)**: 6 tasks
- **Phase 5 (User Story 3 - Validation)**: 5 tasks
- **Phase 6 (User Story 4 - CLI)**: 7 tasks
- **Phase 7 (Polish)**: 7 tasks
- **Parallel Opportunities**: 35 tasks marked [P]
- **MVP Scope**: Phases 1, 2, 3, 4 (38 tasks) - Delivers functional API + web UI

---

## Success Criteria Mapping

- **SC-001** (Response < 1s): Verified in T056
- **SC-002** (All categories correct): Tested in T005-T007 (boundary tests)
- **SC-003** (100% invalid inputs rejected): Tested in T009-T012, T021-T022, T039-T041
- **SC-004** (Web interface accessible): Implemented in T033-T038
- **SC-005** (Server ready < 3s): Verified in T057
- **SC-006** (Expected test cases pass): Covered in T008, T020, and domain unit tests
