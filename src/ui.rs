// UI layer: Embedded HTML page with Bootstrap and JavaScript
use axum::response::{Html, IntoResponse};

// T033-T036: Embedded HTML with Bootstrap, form, fetch(), result display, error display
const HTML_PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BMI Calculator</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
</head>
<body>
    <div class="container mt-5">
        <div class="row justify-content-center">
            <div class="col-md-6">
                <div class="card">
                    <div class="card-header bg-primary text-white">
                        <h3 class="text-center mb-0">BMI Calculator</h3>
                    </div>
                    <div class="card-body">
                        <form id="bmiForm">
                            <div class="mb-3">
                                <label for="weight" class="form-label">Weight (kg)</label>
                                <input type="number" step="0.1" class="form-control" id="weight"
                                       placeholder="e.g., 70.0" required>
                            </div>
                            <div class="mb-3">
                                <label for="height" class="form-label">Height (m)</label>
                                <input type="number" step="0.01" class="form-control" id="height"
                                       placeholder="e.g., 1.75" required>
                            </div>
                            <button type="submit" class="btn btn-primary w-100">Calculate BMI</button>
                        </form>

                        <!-- T035: Result display area -->
                        <div id="result" class="mt-4 alert alert-success" style="display: none;">
                            <h5>Your BMI: <span id="bmiValue"></span></h5>
                            <p class="mb-0">Category: <strong id="bmiCategory"></strong></p>
                        </div>

                        <!-- T036: Error display area -->
                        <div id="error" class="mt-4 alert alert-danger" style="display: none;">
                            <strong>Error:</strong> <span id="errorMessage"></span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js"></script>
    <script>
        // T034: JavaScript fetch() to POST form data to /api/bmi
        document.getElementById('bmiForm').addEventListener('submit', async (e) => {
            e.preventDefault();

            // Hide previous results/errors
            document.getElementById('result').style.display = 'none';
            document.getElementById('error').style.display = 'none';

            // Get form values
            const weight = parseFloat(document.getElementById('weight').value);
            const height = parseFloat(document.getElementById('height').value);

            try {
                // POST to API endpoint
                const response = await fetch('/api/bmi', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        weight_kg: weight,
                        height_m: height
                    })
                });

                const data = await response.json();

                if (response.ok) {
                    // Display success result
                    document.getElementById('bmiValue').textContent = data.bmi;
                    document.getElementById('bmiCategory').textContent = data.category;
                    document.getElementById('result').style.display = 'block';
                } else {
                    // Display validation error
                    document.getElementById('errorMessage').textContent = data.error || 'An error occurred';
                    document.getElementById('error').style.display = 'block';
                }
            } catch (err) {
                // Display network error
                document.getElementById('errorMessage').textContent = 'Network error: ' + err.message;
                document.getElementById('error').style.display = 'block';
            }
        });
    </script>
</body>
</html>
"#;

// T037: GET / handler returning Html response
pub async fn serve_homepage() -> impl IntoResponse {
    Html(HTML_PAGE)
}
