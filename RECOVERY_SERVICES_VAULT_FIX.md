# Recovery Services Vault List Fix

## Issue
The `recovery_services_list_vaults` tool is returning zero vaults even though vaults exist in the subscription:
```json
{
  "vaults": [],
  "total_count": 0
}
```

## Root Cause Analysis

After examining the code, I've identified several potential issues:

1. **Permissions**: The API call requires subscription-level access with the appropriate RBAC role (Reader or Contributor).

2. **API Version**: The API version used (2021-12-01) might not be compatible with your Azure environment.

3. **Token Scope**: The OAuth token might not have the correct scope for listing vaults at the subscription level.

4. **Subscription ID**: The subscription ID in the configuration might not be correct or might not contain any Recovery Services vaults.

5. **Response Parsing**: There might be an issue with parsing the response from the API.

## Solution

Here's how to fix the issue:

1. **Check Permissions**: Ensure your Azure account has the appropriate permissions (Reader or Contributor) at the subscription level.

2. **Update API Version**: Try using a different API version:

```diff
- .query(&[("api-version", "2021-12-01")])
+ .query(&[("api-version", "2022-10-01")])
```

3. **Add Debug Logging**: Add more detailed logging to see the exact API response:

```diff
let json_response = self.handle_response(response).await?;
+ tracing::debug!("Vault list response: {:?}", json_response);
```

4. **Verify Subscription ID**: Double-check that the subscription ID in your config is correct:

```bash
az account show --query id -o tsv
```

5. **Test with Azure CLI**: Verify that vaults exist using the Azure CLI:

```bash
az backup vault list --subscription <your-subscription-id>
```

6. **Clear Auth Cache**: Try clearing the authentication cache:

```bash
codex "clear recovery services auth cache"
```

## Implementation

To fix this issue, update the `list_vaults` method in `codex-rs/core/src/recovery_services/client.rs`:

```rust
pub async fn list_vaults(&self) -> Result<Vec<VaultInfo>> {
    let url = format!(
        "https://management.azure.com/subscriptions/{}/providers/Microsoft.RecoveryServices/vaults",
        self.subscription_id
    );

    tracing::info!("Listing vaults with URL: {}", url);

    let response = self
        .client
        .get(&url)
        .header("Authorization", format!("Bearer {}", self.access_token))
        .header("Content-Type", "application/json")
        .query(&[("api-version", "2022-10-01")])
        .send()
        .await
        .map_err(|e| CodexErr::Other(format!("Failed to list vaults: {}", e)))?;

    let json_response = self.handle_response(response).await?;
    tracing::debug!("Vault list response: {:?}", json_response);
    
    if let Some(vaults_array) = json_response.get("value").and_then(|v| v.as_array()) {
        let mut vaults = Vec::new();
        for vault_json in vaults_array {
            if let Ok(vault) = serde_json::from_value::<VaultInfo>(vault_json.clone()) {
                vaults.push(vault);
            } else {
                tracing::warn!("Failed to parse vault info: {:?}", vault_json);
            }
        }
        Ok(vaults)
    } else {
        tracing::warn!("No 'value' array found in response: {:?}", json_response);
        Ok(Vec::new())
    }
}
```

After making these changes, rebuild the server and test again.