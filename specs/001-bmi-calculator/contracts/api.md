# API Contract: BMI Calculator Web Service

**Feature**: 001-bmi-calculator | **Date**: 2026-03-11

## Base URL

`http://{host}:{port}`

Default: `http://127.0.0.1:3000`

---

## Endpoints

### POST /api/bmi

Calculate BMI from weight and height.

**Request**:

```
Content-Type: application/json
```

```json
{
  "weight_kg": 70.0,
  "height_m": 1.75
}
```

| Field       | Type  | Required | Constraints              |
|-------------|-------|----------|--------------------------|
| `weight_kg` | `f64` | Yes      | Must be positive (> 0.0) |
| `height_m`  | `f64` | Yes      | Must be positive (> 0.0) |

**Success Response (200 OK)**:

```json
{
  "bmi": 22.9,
  "category": "Normal"
}
```

| Field      | Type     | Description                                          |
|------------|----------|------------------------------------------------------|
| `bmi`      | `f64`    | BMI value rounded to 1 decimal place                 |
| `category` | `string` | One of: `"Underweight"`, `"Normal"`, `"Overweight"`, `"Obese"` |

**Validation Error Response (422 Unprocessable Entity)**:

```json
{
  "error": "weight_kg must be positive"
}
```

| Field   | Type     | Description                          |
|---------|----------|--------------------------------------|
| `error` | `string` | Human-readable validation error message |

**Error messages**:
- `"weight_kg must be positive"` — when `weight_kg <= 0.0`
- `"height_m must be positive"` — when `height_m <= 0.0`

**Malformed request (422 Unprocessable Entity)**:

When the request body is missing, not valid JSON, or missing required fields, Axum's built-in `Json` extractor rejection returns a 422 with a descriptive error.

---

### GET /health

Health check endpoint.

**Response (200 OK)**:

Empty body, status 200.

---

### GET /

Serves the embedded HTML UI page with Bootstrap styling and a BMI calculation form.

**Response (200 OK)**:

```
Content-Type: text/html
```

The HTML page contains:
- A form with weight (kg) and height (m) inputs
- JavaScript using `fetch()` to POST to `/api/bmi`
- Result display area showing BMI value and category
- Error display for validation failures
- Bootstrap 5 via CDN for styling

---

## Category Thresholds (WHO Standard)

| Category      | BMI Range          |
|---------------|--------------------|
| Underweight   | < 18.5             |
| Normal        | 18.5 – 24.9       |
| Overweight    | 25.0 – 29.9       |
| Obese         | >= 30.0            |

## Example Scenarios

| Weight (kg) | Height (m) | BMI  | Category    | HTTP Status |
|-------------|------------|------|-------------|-------------|
| 70.0        | 1.75       | 22.9 | Normal      | 200         |
| 50.0        | 1.80       | 15.4 | Underweight | 200         |
| 90.0        | 1.70       | 31.1 | Obese       | 200         |
| 110.0       | 1.65       | 40.4 | Obese       | 200         |
| 0.0         | 1.75       | —    | —           | 422         |
| 70.0        | 0.0        | —    | —           | 422         |
| -5.0        | 1.75       | —    | —           | 422         |
