# anonymous_api

All URIs are relative to *http://localhost:8082/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**anonymousRegisters**](anonymous_api.md#anonymousRegisters) | **POST** /user/anonymous-register | Anonymous registers


# **anonymousRegisters**
> models::AnonymousRegistrationResult anonymousRegisters(optional)
Anonymous registers

Anonymous registers without any data

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | **serde_json::Value**| Empty request body | 

### Return type

[**models::AnonymousRegistrationResult**](AnonymousRegistrationResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

