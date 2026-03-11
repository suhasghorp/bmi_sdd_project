# Feature Specification: BMI Calculator Web Service

**Feature Branch**: `001-bmi-calculator`
**Created**: 2026-03-11
**Status**: Draft
**Input**: User description: "BMI calculator web service with CLI-configured server, domain-layer validation, REST API, embedded HTML UI, and comprehensive test coverage"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Calculate BMI via Web Form (Priority: P1)

A user opens the BMI calculator in their browser. They see a simple HTML form where they enter their weight (in kilograms) and height (in meters). After submitting the form, the system calculates their BMI, rounds it to one decimal place, and returns the numeric BMI value along with a human-readable category (Underweight, Normal, Overweight, or Obese).

**Why this priority**: This is the core value proposition — users need a fast, accessible way to calculate their BMI and understand what it means.

**Independent Test**: Can be fully tested by opening the web page, entering weight and height values, and verifying that the correct BMI and category are returned.

**Acceptance Scenarios**:

1. **Given** the BMI calculator page is loaded, **When** a user submits weight 70.0 kg and height 1.75 m, **Then** the system returns BMI 22.9 with category "Normal"
2. **Given** the BMI calculator page is loaded, **When** a user submits weight 50.0 kg and height 1.80 m, **Then** the system returns BMI 15.4 with category "Underweight"
3. **Given** the BMI calculator page is loaded, **When** a user submits weight 90.0 kg and height 1.70 m, **Then** the system returns BMI 31.1 with category "Obese"

---

### User Story 2 - Calculate BMI via API (Priority: P1)

A developer or external system sends a JSON request to the BMI calculation endpoint with weight and height values. The system validates the inputs, calculates BMI, and returns a JSON response containing the BMI value (rounded to 1 decimal place) and the corresponding category.

**Why this priority**: The API endpoint is the primary programmatic interface and enables integration with other systems. It shares P1 with the web form as both are essential delivery mechanisms.

**Independent Test**: Can be fully tested by sending a JSON POST request with weight and height and verifying the JSON response contains the correct BMI and category.

**Acceptance Scenarios**:

1. **Given** the API is running, **When** a client sends a valid JSON request with weight 70.0 and height 1.75, **Then** the system responds with JSON containing bmi: 22.9 and category: "Normal"
2. **Given** the API is running, **When** a client sends a valid JSON request with weight 110.0 and height 1.65, **Then** the system responds with JSON containing the correct BMI and category "Obese"

---

### User Story 3 - Input Validation with Clear Error Messages (Priority: P2)

When a user submits invalid data (such as zero or negative weight, or zero or negative height), the system rejects the request with a clear error message explaining what went wrong. This validation occurs at the domain level, ensuring consistent behavior regardless of whether the request comes from the web form or the API.

**Why this priority**: Input validation is essential for data integrity and user experience, but the system cannot deliver value without the core calculation (P1) being in place first.

**Independent Test**: Can be fully tested by submitting invalid values (zero weight, negative height) and verifying appropriate error responses are returned.

**Acceptance Scenarios**:

1. **Given** the API is running, **When** a client sends weight 0.0 and height 1.75, **Then** the system responds with an error indicating the weight is invalid
2. **Given** the API is running, **When** a client sends weight 70.0 and height 0.0, **Then** the system responds with an error indicating the height is invalid
3. **Given** the API is running, **When** a client sends weight -5.0 and height 1.75, **Then** the system responds with an error indicating the weight is invalid
4. **Given** the API is running, **When** a client sends weight 70.0 and height -1.0, **Then** the system responds with an error indicating the height is invalid

---

### User Story 4 - Server Startup via CLI (Priority: P3)

An operator starts the BMI calculator service from the command line. The server binds to a configurable address and port, logs startup information, and begins accepting requests. The operator can control log verbosity through environment configuration.

**Why this priority**: Server startup and CLI configuration are operational concerns. The system must work correctly first (P1, P2) before operational configuration matters.

**Independent Test**: Can be fully tested by starting the server from the command line and verifying it binds to the expected address and responds to requests.

**Acceptance Scenarios**:

1. **Given** the application binary is available, **When** an operator starts the server, **Then** the server binds to the configured address and begins accepting HTTP requests
2. **Given** the server is starting, **When** the operator has configured logging, **Then** startup information is logged at the appropriate level

---

### Edge Cases

- What happens when weight is zero? System returns a validation error (InvalidWeight)
- What happens when height is zero? System returns a validation error (InvalidHeight)
- What happens when weight is negative? System returns a validation error (InvalidWeight)
- What happens when height is negative? System returns a validation error (InvalidHeight)
- What happens with extremely large weight or height values? System calculates BMI normally (no upper-bound restriction assumed)
- What happens when the request body is missing or malformed JSON? System returns an appropriate error response

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST calculate BMI using the standard formula (weight in kg divided by height in meters squared)
- **FR-002**: System MUST round the calculated BMI to exactly 1 decimal place
- **FR-003**: System MUST classify BMI into categories: Underweight (below 18.5), Normal (18.5 to 24.9), Overweight (25.0 to 29.9), Obese (30.0 and above)
- **FR-004**: System MUST reject weight values that are zero or negative, returning a clear "invalid weight" error
- **FR-005**: System MUST reject height values that are zero or negative, returning a clear "invalid height" error
- **FR-006**: System MUST perform input validation at the domain level, ensuring consistent behavior across all interfaces (API and web UI)
- **FR-007**: System MUST expose a JSON API endpoint that accepts weight and height and returns the BMI value and category
- **FR-008**: System MUST return validation errors with a 422 (Unprocessable Entity) status code
- **FR-009**: System MUST serve an embedded HTML page that provides a web form for BMI calculation
- **FR-010**: System MUST accept PORT environment variable (takes precedence when set) and --port CLI flag for server configuration, following Heroku deployment conventions
- **FR-011**: System MUST log server activity using tracing-subscriber with log filtering controlled via RUST_LOG environment variable (standard Rust tracing convention)
- **FR-012**: System MUST expose a health check endpoint (GET /health) that returns 200 OK status with no body, enabling load balancers and monitoring tools to verify service availability

### Key Entities

- **BmiInput**: Represents a BMI calculation request containing weight (kg) and height (meters)
- **BmiResult**: Represents the outcome of a BMI calculation containing the numeric BMI value and its category
- **BmiCategory**: Represents the classification of a BMI value — one of Underweight, Normal, Overweight, or Obese. Has a human-readable text representation used in API responses

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can calculate their BMI and receive a result in under 1 second from form submission
- **SC-002**: All four BMI categories (Underweight, Normal, Overweight, Obese) are correctly assigned based on standard WHO thresholds
- **SC-003**: 100% of invalid inputs (zero/negative weight or height) are rejected with descriptive error messages before any calculation is attempted
- **SC-004**: The web interface is accessible via a single URL with no external dependencies (all assets embedded)
- **SC-005**: The service starts and is ready to accept requests within 3 seconds of launch
- **SC-006**: All expected test cases pass: calculate_bmi(70.0, 1.75) = 22.9/Normal, calculate_bmi(50.0, 1.80) = 15.4/Underweight, calculate_bmi(90.0, 1.70) = 31.1/Obese, calculate_bmi(0.0, 1.75) = Error

## Assumptions

- BMI is calculated using the standard formula: weight (kg) / height (m)^2
- Weight is provided in kilograms, height in meters
- BMI category thresholds follow WHO standards: <18.5 Underweight, 18.5-24.9 Normal, 25.0-29.9 Overweight, >=30.0 Obese
- No upper-bound validation is enforced on weight or height (only zero/negative values are rejected)
- The HTML page is a self-contained, single-page interface with Bootstrap CSS/JS loaded via CDN (no local asset files to serve)
- The server defaults to binding on localhost if no CLI arguments are provided
- Logging uses environment-based filter configuration (e.g., RUST_LOG)
