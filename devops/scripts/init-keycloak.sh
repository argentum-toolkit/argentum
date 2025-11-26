#!/usr/bin/env bash

# set -ex

# Get admin access token
# This token is required for all admin API requests
echo "Requesting admin access token..."
TOKEN_RESPONSE=$(curl -X POST \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id=admin-cli" \
  -d "username=admin" \
  -d "password=password" \
  -d "grant_type=password" \
  http://localhost:8099/realms/master/protocol/openid-connect/token)

# Extract access token from the response
TOKEN=$(echo "$TOKEN_RESPONSE" | jq -r '.access_token')

# Check if token was received
if [ -z "$TOKEN" ] || [ "$TOKEN" == "null" ]; then
  echo "Failed to get access token. Response: $TOKEN_RESPONSE"
  exit 1
fi

echo "Access token received successfully."


REALM="argentum-realm-dev"

# Check if realm already exists
echo "Checking if realm '$REALM' already exists..."
REALM_EXISTS=$(curl -s -X GET \
  -H "Authorization: Bearer $TOKEN" \
  http://localhost:8099/admin/realms | jq -r '.[] | select(.realm == "'$REALM'")')

if [ -n "$REALM_EXISTS" ]; then
  echo "Realm '$REALM' already exists. Skipping creation."
else
  # Create realm
  echo "Creating realm '$REALM'..."
  CREATE_REALM_RESPONSE=$(curl -s -i -w "\n%{http_code}" -X POST \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"realm": "'$REALM'", "enabled": true}' \
    http://localhost:8099/admin/realms)

  HTTP_CODE=$(echo "$CREATE_REALM_RESPONSE" | tail -n1)
  if [ "$HTTP_CODE" -ne 201 ]; then
    echo "Failed to create realm. HTTP Code: $HTTP_CODE"
    exit 1
  fi
  echo "Realm '$REALM' created successfully."
fi


echo "Enabling 'Email as username' for realm '$REALM'..."
ENABLE_EMAIL_AS_USERNAME_RESPONSE=$(curl -s -i -w "\n%{http_code}" -X PUT \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "loginWithEmailAllowed": true, 
    "duplicateEmailsAllowed": false,
    "registrationEmailAsUsername": true,
    "smtpServer": {
        "host": "mailhog",
        "port": "1025",
        "from": "no-reply@example.com",
        "auth": "false",
        "ssl": "false",
        "starttls": "false"
    },
    "attributes": {
        "frontendUrl": "http://localhost:8099"
    }
  }' \
  http://localhost:8099/admin/realms/$REALM)

HTTP_CODE=$(echo "$ENABLE_EMAIL_AS_USERNAME_RESPONSE" | tail -n1)
if [ "$HTTP_CODE" -ne 204 ]; then
  echo "Failed to enable 'Email as username'. HTTP Code: $HTTP_CODE"
  exit 1
fi

echo "'Email as username' enabled successfully for realm '$REALM'."


CUSTOM_THEME="argentum"

echo "Applying custom theme '$CUSTOM_THEME' to realm '$REALM'..."
curl -s -X PUT \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "loginTheme": "'$CUSTOM_THEME'",
    "emailTheme": "'$CUSTOM_THEME'"
  }' \
  http://localhost:8099/admin/realms/$REALM
echo "Custom theme '$CUSTOM_THEME' applied."





CLIENT="argentum-be-client-dev"
CLIENT_SECRET="dj9cobGEQptoEk5gMvqadYHd6emeUQ2Y"

# Check if client already exists
echo "Checking if client '$CLIENT' already exists in realm '$REALM'..."
CLIENT_EXISTS=$(curl -s -X GET \
  -H "Authorization: Bearer $TOKEN" \
  http://localhost:8099/admin/realms/$REALM/clients | jq -r '.[] | select(.clientId == "'$CLIENT'")')

if [ -n "$CLIENT_EXISTS" ]; then
  echo "Client '$CLIENT' already exists in realm '$REALM'. Skipping creation."
else
  # Create client in realm
  echo "Creating client '$CLIENT' in realm '$REALM'..."
  CLIENT_PAYLOAD='{
    "clientId": "'$CLIENT'",
    "enabled": true,
    "protocol": "openid-connect",
    "publicClient": false,
    "standardFlowEnabled": false,
    "directAccessGrantsEnabled": true,
    "serviceAccountsEnabled": true,
    "secret": "'$CLIENT_SECRET'"
  }'

  CREATE_CLIENT_RESPONSE=$(curl -s -v -X POST \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "$CLIENT_PAYLOAD" \
    http://localhost:8099/admin/realms/$REALM/clients 2>&1)


  # Check HTTP code
  HTTP_CODE=$(echo "$CREATE_CLIENT_RESPONSE" | grep -Eo 'HTTP/[0-9.]+ [0-9]{3}' | tail -n1 | awk '{print $2}')
  if [ "$HTTP_CODE" != "201" ]; then
    echo "Failed to create client. HTTP Code: $HTTP_CODE"
    echo "Full response: $CREATE_CLIENT_RESPONSE"
    exit 1
  fi
  echo "Client '$CLIENT' created successfully."
fi






# ADD ROLE  view-users

CLIENT_UUID=$(curl -s -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8099/admin/realms/$REALM/clients?clientId=$CLIENT" \
  | jq -r '.[0].id')


SERVICE_USER_ID=$(curl -s -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8099/admin/realms/$REALM/clients/$CLIENT_UUID/service-account-user" \
  | jq -r '.id')


REALM_MGMT_CLIENT_ID=$(curl -s -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8099/admin/realms/$REALM/clients?clientId=realm-management" \
  | jq -r '.[0].id')

ROLE_PAYLOAD=$(curl -s -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8099/admin/realms/$REALM/clients/$REALM_MGMT_CLIENT_ID/roles/view-users")
ROLE_ID=$(echo "$ROLE_PAYLOAD" | jq -r '.id')


ASSIGN_ROLE_PAYLOAD="[{\"id\": \"$ROLE_ID\", \"name\": \"view-users\"}]"

curl -s -X POST \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "$ASSIGN_ROLE_PAYLOAD" \
    "http://localhost:8099/admin/realms/$REALM/users/$SERVICE_USER_ID/role-mappings/clients/$REALM_MGMT_CLIENT_ID"



ROLE_PAYLOAD=$(curl -s -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8099/admin/realms/$REALM/clients/$REALM_MGMT_CLIENT_ID/roles/manage-users")
ROLE_ID=$(echo "$ROLE_PAYLOAD" | jq -r '.id')


ASSIGN_ROLE_PAYLOAD="[{\"id\": \"$ROLE_ID\", \"name\": \"manage-users\"}]"

curl -s -X POST \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "$ASSIGN_ROLE_PAYLOAD" \
    "http://localhost:8099/admin/realms/$REALM/users/$SERVICE_USER_ID/role-mappings/clients/$REALM_MGMT_CLIENT_ID"





FRONTEND_CLIENT="argentum-fe-client-dev"
FRONTEND_REDIRECT="http://localhost:8080/oidc/auth/callback"

echo "Checking if frontend client '$FRONTEND_CLIENT' exists in realm '$REALM'..."
CLIENT_EXISTS=$(curl -s -X GET \
  -H "Authorization: Bearer $TOKEN" \
  http://localhost:8099/admin/realms/$REALM/clients | jq -r '.[] | select(.clientId == "'$FRONTEND_CLIENT'")')

if [ -n "$CLIENT_EXISTS" ]; then
  echo "Frontend client '$FRONTEND_CLIENT' already exists. Skipping creation."
else
  echo "Creating frontend client '$FRONTEND_CLIENT'..."
  CLIENT_PAYLOAD='{
    "clientId": "'$FRONTEND_CLIENT'",
    "enabled": true,
    "protocol": "openid-connect",
    "publicClient": true,
    "standardFlowEnabled": true,
    "directAccessGrantsEnabled": false,
    "serviceAccountsEnabled": false,
    "redirectUris": ["'$FRONTEND_REDIRECT'"]
  }'

  CREATE_CLIENT_RESPONSE=$(curl -s -v -X POST \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d "$CLIENT_PAYLOAD" \
    http://localhost:8099/admin/realms/$REALM/clients 2>&1)

  HTTP_CODE=$(echo "$CREATE_CLIENT_RESPONSE" | grep -Eo 'HTTP/[0-9.]+ [0-9]{3}' | tail -n1 | awk '{print $2}')
  if [ "$HTTP_CODE" != "201" ]; then
    echo "Failed to create frontend client. HTTP Code: $HTTP_CODE"
    echo "Full response: $CREATE_CLIENT_RESPONSE"
    exit 1
  fi
  echo "Frontend client '$FRONTEND_CLIENT' created successfully."
fi
