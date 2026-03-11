# Data Model: BMI Calculator Web Service

**Feature**: 001-bmi-calculator | **Date**: 2026-03-11

## Entities

### BmiCategory (Domain Layer)

Represents the WHO classification of a BMI value.

```rust
// src/domain.rs
pub enum BmiCategory {
    Underweight,  // BMI < 18.5
    Normal,       // 18.5 <= BMI <= 24.9
    Overweight,   // 25.0 <= BMI <= 29.9
    Obese,        // BMI >= 30.0
}
```

**Display representation**: Each variant maps to a human-readable string — `"Underweight"`, `"Normal"`, `"Overweight"`, `"Obese"`. Implemented via `std::fmt::Display`.

**No Serde**: This type lives in the domain layer and MUST NOT derive `Serialize`/`Deserialize`. Conversion to string happens at the API layer boundary.

### BmiResult (Domain Layer)

Represents the outcome of a successful BMI calculation.

```rust
// src/domain.rs
pub struct BmiResult {
    pub bmi: f64,              // Rounded to 1 decimal place
    pub category: BmiCategory, // WHO classification
}
```

**Invariants**:
- `bmi` is always positive (guaranteed by input validation)
- `bmi` is always rounded to exactly 1 decimal place
- `category` always matches the WHO threshold for the `bmi` value

**No Serde**: Domain type. Serialization is the API layer's responsibility.

### BmiError (Domain Layer)

Represents domain-level validation failures.

```rust
// src/domain.rs — derived with thiserror
pub enum BmiError {
    InvalidWeight,  // weight_kg <= 0.0
    InvalidHeight,  // height_m <= 0.0
}
```

**Display messages**:
- `InvalidWeight` → `"weight_kg must be positive"`
- `InvalidHeight` → `"height_m must be positive"`

### BmiRequest (API Layer)

Represents an incoming JSON request for BMI calculation.

```rust
// src/api.rs
#[derive(Deserialize)]
pub struct BmiRequest {
    pub weight_kg: f64,
    pub height_m: f64,
}
```

**Validation**: None at this level. Values are passed to the domain function which performs validation.

### BmiResponse (API Layer)

Represents a successful JSON response.

```rust
// src/api.rs
#[derive(Serialize)]
pub struct BmiResponse {
    pub bmi: f64,
    pub category: String,
}
```

**Mapping**: Constructed from `BmiResult` — `bmi` copied directly, `category` via `BmiCategory::to_string()`.

### ErrorResponse (API Layer)

Represents a JSON error response.

```rust
// src/api.rs
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
```

**HTTP Status**: Always returned with HTTP 422 (Unprocessable Entity).

## Domain Function

```rust
// src/domain.rs
pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> Result<BmiResult, BmiError>
```

**Algorithm**:
1. Validate `weight_kg > 0.0`, else return `Err(BmiError::InvalidWeight)`
2. Validate `height_m > 0.0`, else return `Err(BmiError::InvalidHeight)`
3. Compute `bmi = weight_kg / (height_m * height_m)`
4. Round to 1 decimal: `bmi = (bmi * 10.0).round() / 10.0`
5. Classify using WHO thresholds:
   - `bmi < 18.5` → `Underweight`
   - `bmi < 25.0` → `Normal`
   - `bmi < 30.0` → `Overweight`
   - `bmi >= 30.0` → `Obese`
6. Return `Ok(BmiResult { bmi, category })`

## State Transitions

N/A — the application is fully stateless. No entity lifecycle or state machine.

## Relationships

```text
BmiRequest (API) ──extracts──→ (weight_kg, height_m) ──calls──→ calculate_bmi (Domain)
                                                                       │
                                                              ┌────────┴────────┐
                                                              │                 │
                                                         Ok(BmiResult)    Err(BmiError)
                                                              │                 │
                                                              ▼                 ▼
                                                        BmiResponse       ErrorResponse
                                                        (200 OK)          (422 Unprocessable)
```
