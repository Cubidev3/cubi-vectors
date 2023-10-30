pub mod vector3;
pub mod vector2;

#[cfg(test)]
mod vector3_tests {
    use crate::vector3::Vector3;

    use super::*;

    #[test]
    fn test_vector3_addition() {
        let v1 = Vector3::xyz(1.0, 2.0, 3.0);
        let v2 = Vector3::xyz(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result, Vector3::xyz(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vector3_subtraction() {
        let v1 = Vector3::xyz(4.0, 5.0, 6.0);
        let v2 = Vector3::xyz(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result, Vector3::xyz(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_vector3_scalar_multiplication() {
        let v = Vector3::xyz(2.0, 3.0, 4.0);
        let scalar = 2.0;
        let result = v * scalar;
        assert_eq!(result, Vector3::xyz(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_vector3_scalar_division() {
        let v = Vector3::xyz(4.0, 6.0, 8.0);
        let scalar = 2.0;
        let result = v / scalar;
        assert_eq!(result, Vector3::xyz(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_vector3_length() {
        let v = Vector3::xyz(3.0, 4.0, 0.0);
        let length = v.len();
        assert_eq!(length, 5.0);
    }

    #[test]
    fn test_vector3_dot_product() {
        let v1 = Vector3::xyz(1.0, 2.0, 3.0);
        let v2 = Vector3::xyz(4.0, 5.0, 6.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_vector3_cross_product() {
        let v1 = Vector3::xyz(1.0, 0.0, 0.0);
        let v2 = Vector3::xyz(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        assert_eq!(result, Vector3::xyz(0.0, 0.0, 1.0));
    }
}

#[cfg(test)]
mod vector2_tests {
    use crate::vector2::Vector2;

    use super::*;

    #[test]
    fn test_vector2_addition() {
        let v1 = Vector2::xy(1.0, 2.0);
        let v2 = Vector2::xy(3.0, 4.0);
        let result = v1 + v2;
        assert_eq!(result, Vector2::xy(4.0, 6.0));
    }

#[test]
    fn test_vector2_subtraction() {
        let v1 = Vector2::xy(4.0, 5.0);
        let v2 = Vector2::xy(1.0, 2.0);
        let result = v1 - v2;
        assert_eq!(result, Vector2::xy(3.0, 3.0));
    }

#[test]
    fn test_vector2_scalar_multiplication() {
        let v = Vector2::xy(2.0, 3.0);
        let scalar = 2.0;
        let result = v * scalar;
        assert_eq!(result, Vector2::xy(4.0, 6.0));
    }

#[test]
    fn test_vector2_scalar_division() {
        let v = Vector2::xy(4.0, 6.0);
        let scalar = 2.0;
        let result = v / scalar;
        assert_eq!(result, Vector2::xy(2.0, 3.0));
    }

#[test]
    fn test_vector2_length() {
        let v = Vector2::xy(3.0, 4.0);
        let length = v.len();
        assert_eq!(length, 5.0);
    }

#[test]
    fn test_vector2_dot_product() {
        let v1 = Vector2::xy(1.0, 2.0);
        let v2 = Vector2::xy(3.0, 4.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 11.0);
    }

// Add more tests for other methods like projection, rejection, angle_between, etc.
}