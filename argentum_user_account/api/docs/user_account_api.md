# user_account_api

All URIs are relative to *http://localhost:8080/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**changePasswordWithToken**](user_account_api.md#changePasswordWithToken) | **POST** /user/restore-password/change-password | User with token changes his password
**loginWithPassword**](user_account_api.md#loginWithPassword) | **POST** /user/password-login | Login as an user
**registerWithPassword**](user_account_api.md#registerWithPassword) | **POST** /user/register | User registers with password
**requestRestoreToken**](user_account_api.md#requestRestoreToken) | **POST** /user/restore-password/token-request | Anonymous requests restore password token


# **changePasswordWithToken**
> serde_json::Value changePasswordWithToken(ctx, change_password_schema)
User with token changes his password

Final step of restoring password process

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **change_password_schema** | [**ChangePasswordSchema**](ChangePasswordSchema.md)| Required data to change password with token | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **loginWithPassword**
> models::LoginResult loginWithPassword(ctx, login_with_password_schema)
Login as an user

Login as an user

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **login_with_password_schema** | [**LoginWithPasswordSchema**](LoginWithPasswordSchema.md)| Required data to login with email and password | 

### Return type

[**models::LoginResult**](LoginResult.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **registerWithPassword**
> models::RegistrationWithPasswordResult registerWithPassword(ctx, registration_with_password_schema)
User registers with password

User registers with email and password

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **registration_with_password_schema** | [**RegistrationWithPasswordSchema**](RegistrationWithPasswordSchema.md)| Data required to register with email and password | 

### Return type

[**models::RegistrationWithPasswordResult**](RegistrationWithPasswordResult.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **requestRestoreToken**
> serde_json::Value requestRestoreToken(ctx, request_restore_token_schema)
Anonymous requests restore password token

Requested token and link will be sent to email

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request_restore_token_schema** | [**RequestRestoreTokenSchema**](RequestRestoreTokenSchema.md)| Required data to get restore password token | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

