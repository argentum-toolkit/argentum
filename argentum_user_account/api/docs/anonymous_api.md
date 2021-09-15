# anonymous_api

All URIs are relative to *http://localhost:8080/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**anonymousRegisters**](anonymous_api.md#anonymousRegisters) | **POST** /user/anonymous-register | Anonymous registers


# **anonymousRegisters**
> models::AnonymousRegistrationResult anonymousRegisters(ctx, optional)
Anonymous registers

Anonymous registers without any data

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | **serde_json::Value**| Empty request body | 

### Return type

[**models::AnonymousRegistrationResult**](AnonymousRegistrationResult.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

