use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::department_model::Department;

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct AddDepartmentRequestDto {
    #[validate(
        length(min = 1, message = "Department name is required"),
        length(max = 20, message = "Department name is too long"),
    )]
    pub department_name: String,

    #[validate(length(min = 1, message = "Club name is required"))]
    pub club_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddDepartmentResponseDto {
    pub status: String,
    pub department: Department
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Validate)]
pub struct GetClubDepartmentsRequestDto {
    #[validate(length(min = 1, message = "Club name is required"))]
    pub club_name: String,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct GetClubDepartmentsResponseDto {
    pub status: String,
    pub departments: Vec<Department>
}