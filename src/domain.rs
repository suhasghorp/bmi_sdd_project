// Domain layer: Pure BMI calculation and WHO category classification
// No I/O, no Serde, no framework dependencies

use std::fmt;

// T013: BmiCategory enum with four variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BmiCategory {
    Underweight, // BMI < 18.5
    Normal,      // 18.5 <= BMI <= 24.9
    Overweight,  // 25.0 <= BMI <= 29.9
    Obese,       // BMI >= 30.0
}

// T014: Display trait for BmiCategory
impl fmt::Display for BmiCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BmiCategory::Underweight => "Underweight",
            BmiCategory::Normal => "Normal",
            BmiCategory::Overweight => "Overweight",
            BmiCategory::Obese => "Obese",
        };
        write!(f, "{}", s)
    }
}

// T015: BmiError enum with thiserror derives
#[derive(Debug, thiserror::Error)]
pub enum BmiError {
    #[error("weight_kg must be positive")]
    InvalidWeight,
    #[error("height_m must be positive")]
    InvalidHeight,
}

// T016: BmiResult struct
#[derive(Debug, Clone, PartialEq)]
pub struct BmiResult {
    pub bmi: f64,              // Rounded to 1 decimal place
    pub category: BmiCategory, // WHO classification
}

// T017: calculate_bmi function with validation and WHO classification
pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> Result<BmiResult, BmiError> {
    // Validate weight
    if weight_kg <= 0.0 {
        return Err(BmiError::InvalidWeight);
    }

    // Validate height
    if height_m <= 0.0 {
        return Err(BmiError::InvalidHeight);
    }

    // Calculate BMI
    let bmi = weight_kg / (height_m * height_m);

    // Round to 1 decimal place
    let bmi = (bmi * 10.0).round() / 10.0;

    // Classify using WHO thresholds
    let category = if bmi < 18.5 {
        BmiCategory::Underweight
    } else if bmi < 25.0 {
        BmiCategory::Normal
    } else if bmi < 30.0 {
        BmiCategory::Overweight
    } else {
        BmiCategory::Obese
    };

    Ok(BmiResult { bmi, category })
}

#[cfg(test)]
mod tests {
    use super::*;

    // T005: Boundary test at 18.5 (Underweight/Normal threshold)
    #[test]
    fn test_category_boundary_18_5() {
        // BMI = 18.5 should be Normal (inclusive lower bound)
        let result = calculate_bmi(51.975, 1.68).unwrap();
        assert_eq!(result.bmi, 18.4);
        assert!(matches!(result.category, BmiCategory::Underweight));

        let result = calculate_bmi(52.22, 1.68).unwrap();
        assert_eq!(result.bmi, 18.5);
        assert!(matches!(result.category, BmiCategory::Normal));
    }

    // T006: Boundary test at 25.0 (Normal/Overweight threshold)
    #[test]
    fn test_category_boundary_25_0() {
        // BMI = 24.9 should be Normal
        let result = calculate_bmi(70.27, 1.68).unwrap();
        assert_eq!(result.bmi, 24.9);
        assert!(matches!(result.category, BmiCategory::Normal));

        // BMI = 25.0 should be Overweight
        let result = calculate_bmi(70.56, 1.68).unwrap();
        assert_eq!(result.bmi, 25.0);
        assert!(matches!(result.category, BmiCategory::Overweight));
    }

    // T007: Boundary test at 30.0 (Overweight/Obese threshold)
    #[test]
    fn test_category_boundary_30_0() {
        // BMI = 29.9 should be Overweight
        let result = calculate_bmi(84.39, 1.68).unwrap();
        assert_eq!(result.bmi, 29.9);
        assert!(matches!(result.category, BmiCategory::Overweight));

        // BMI = 30.0 should be Obese
        let result = calculate_bmi(84.67, 1.68).unwrap();
        assert_eq!(result.bmi, 30.0);
        assert!(matches!(result.category, BmiCategory::Obese));
    }

    // T008: Valid input test
    #[test]
    fn test_calculate_bmi_valid_inputs() {
        let result = calculate_bmi(70.0, 1.75).unwrap();
        assert_eq!(result.bmi, 22.9);
        assert!(matches!(result.category, BmiCategory::Normal));
    }

    // T009: Zero weight validation
    #[test]
    fn test_zero_weight_error() {
        let result = calculate_bmi(0.0, 1.75);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BmiError::InvalidWeight));
    }

    // T010: Negative weight validation
    #[test]
    fn test_negative_weight_error() {
        let result = calculate_bmi(-5.0, 1.75);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BmiError::InvalidWeight));
    }

    // T011: Zero height validation
    #[test]
    fn test_zero_height_error() {
        let result = calculate_bmi(70.0, 0.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BmiError::InvalidHeight));
    }

    // T012: Negative height validation
    #[test]
    fn test_negative_height_error() {
        let result = calculate_bmi(70.0, -1.75);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BmiError::InvalidHeight));
    }

    // T040: Edge case - very large weight values
    #[test]
    fn test_very_large_weight() {
        let result = calculate_bmi(500.0, 2.0).unwrap();
        assert_eq!(result.bmi, 125.0);
        assert!(matches!(result.category, BmiCategory::Obese));
    }

    // T041: Edge case - very large height values
    #[test]
    fn test_very_large_height() {
        let result = calculate_bmi(70.0, 3.0).unwrap();
        assert_eq!(result.bmi, 7.8);
        assert!(matches!(result.category, BmiCategory::Underweight));
    }
}
