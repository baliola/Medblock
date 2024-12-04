# Pull Request Documentation

## Overview
This PR implements family management features in the patient registry system, allowing patients to create and manage family groups with EMR access controls, while also enhancing admin capabilities and core patient registry functions.

## Key Implementations

### Family Management Core Features
- Family group creation and management
- Family member addition and removal
- EMR access delegation controls
- Consent management for family EMR access

### Patient Registry Enhancements
- Enhanced patient registration and info management
- Admin-specific patient search and listing
- KYC status management
- Improved patient data validation

### Build System Improvements
- Color-coded logging system with enhanced error reporting
- Optimized build process for EMR registry dependencies
- Improved error handling with detailed messaging

## New APIs

### Request Types
- CreateGroupRequest (Family Creation)
- AddGroupMemberRequest (Family Member Addition)
- LeaveGroupRequest (Family Exit)
- GrantGroupAccessRequest (EMR Access Grant)
- RevokeGroupAccessRequest (EMR Access Revocation)
- ViewGroupMemberEmrInformationRequest (EMR View)
- GetGroupDetailsRequest (Family Details)
- SearchPatientRequest (Admin Search)
- UpdateKycStatusRequest (KYC Management)

### Response Types
- CreateGroupResponse
- GetGroupDetailsResponse
- GetUserGroupsResponse
- UpdateKycStatusResponse
- PatientListAdminResponse
- SearchPatientAdminResponse

## Implemented Functions by Feature

### Create Family
- create_group
- get_group_details
- get_user_groups

### Add Family Member
- add_group_member
- bind_admin

### Leave Family
- leave_group

### Grant delegated family member EMR
- grant_group_access
- claim_consent_for_group

### Revoke delegated family member EMR
- revoke_group_access
- is_consent_claimed

### View delegated family member EMR
- get_patient_info
- construct_get_provider_batch_args
- do_call_get_provider_batch

### Admin Operations

#### get_patient_list_admin
- **Purpose**: Retrieves paginated list of all patients
- **Access**: Admin only
- **Input**: Pagination parameters
- **Returns**: List of patient information
- **Error Cases**: Fails if caller is not admin

#### search_patient_admin
- **Purpose**: Advanced patient search for admins
- **Access**: Admin only
- **Input**: Search criteria
- **Returns**: Matching patient records
- **Error Cases**: Fails if caller is not admin

#### update_kyc_status
- **Purpose**: Updates patient KYC verification status
- **Access**: Admin only
- **Input**: Patient NIK, new KYC status
- **Returns**: Update status
- **Error Cases**: 
  - Fails if caller is not admin
  - Fails if invalid NIK

### Core Patient Functions

#### register_patient
- **Purpose**: Registers new patient in the system
- **Access**: Public
- **Input**: Patient registration details
- **Returns**: Registration status
- **Error Cases**: 
  - Fails if invalid data
  - Fails if patient already exists

#### update_initial_patient_info
- **Purpose**: Updates initial patient information
- **Access**: Patient only
- **Input**: Updated patient details
- **Returns**: Update status
- **Error Cases**: Fails if unauthorized

## Testing Infrastructure
- test_group_creation_and_emr_access
- test_emr_access_permissions
- test_group_retrieval
- test_patient_registration
- test_patient_retrieval
- test_admin_patient_list
- test_search_patient_admin

## Frontend Available Functions

### Family Creation and Management

#### create_group
- **Purpose**: Creates a new family group with the caller as the group leader
- **Access**: Any registered patient
- **Input**: Group name
- **Returns**: Group ID and creation status
- **Error Cases**: Fails if caller is not a registered patient

#### get_group_details
- **Purpose**: Retrieves detailed information about a specific family group
- **Access**: Group members only
- **Input**: Group ID, pagination parameters
- **Returns**: Group name, leader, member list, and member count
- **Error Cases**: Fails if caller is not a group member

#### get_user_groups
- **Purpose**: Lists all family groups where the caller is a member
- **Access**: Any registered patient
- **Returns**: Array of group information including roles and relationships
- **Error Cases**: Fails if caller is not a registered patient

### Family Member Operations

#### add_group_member
- **Purpose**: Adds a new member to a family group
- **Access**: Group leader only
- **Input**: Member NIK, relationship type (Parent/Child/Spouse/Sibling/Other)
- **Returns**: Success status
- **Error Cases**: 
  - Fails if caller is not group leader
  - Fails if member already exists
  - Fails if invalid relationship type

#### leave_group
- **Purpose**: Removes caller from a family group
- **Access**: Any group member except leader
- **Input**: Group ID
- **Returns**: Success status
- **Error Cases**: 
  - Fails if caller is group leader
  - Fails if caller is not in group

### EMR Access Control

#### grant_group_access
- **Purpose**: Grants EMR access to a family member
- **Access**: Patient (EMR owner) only
- **Input**: Group ID, member NIK
- **Returns**: Access grant status
- **Error Cases**: 
  - Fails if caller is not EMR owner
  - Fails if target is not group member

#### revoke_group_access
- **Purpose**: Revokes previously granted EMR access
- **Access**: Patient (EMR owner) only
- **Input**: Member NIK
- **Returns**: Revocation status
- **Error Cases**: 
  - Fails if caller is not EMR owner
  - Fails if access was not previously granted

#### claim_consent_for_group
- **Purpose**: Claims consent for viewing family member's EMR
- **Access**: Group members with granted access
- **Input**: Consent code, group ID
- **Returns**: Consent claim status
- **Error Cases**: 
  - Fails if invalid consent code
  - Fails if caller not in group
  - Fails if access not granted

#### is_consent_claimed
- **Purpose**: Checks if consent has been claimed for EMR access
- **Access**: Any group member
- **Input**: Consent code
- **Returns**: Boolean claim status
- **Error Cases**: Fails if invalid consent code

### EMR Viewing

#### get_patient_info
- **Purpose**: Retrieves patient EMR information
- **Access**: 
  - Patient (own EMR)
  - Group members with claimed consent
- **Input**: Patient NIK
- **Returns**: Patient EMR data
- **Error Cases**: 
  - Fails if unauthorized
  - Fails if consent not claimed
  - Fails if invalid NIK
