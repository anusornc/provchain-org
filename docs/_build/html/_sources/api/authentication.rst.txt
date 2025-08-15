Authentication
=============

Secure access to ProvChainOrg APIs through multiple authentication methods designed for different use cases and security requirements.

.. raw:: html

   <div class="hero-section">
     <div class="hero-content">
       <h1>API Authentication</h1>
       <p class="hero-subtitle">Secure access to ProvChainOrg APIs with multiple authentication methods</p>
       <div class="hero-badges">
         <span class="badge badge-api">API</span>
         <span class="badge badge-security">Security</span>
         <span class="badge badge-authentication">Authentication</span>
         <span class="badge badge-integration">Integration</span>
       </div>
     </div>
   </div>

Overview
--------

ProvChainOrg provides multiple authentication methods to secure API access while maintaining flexibility for different integration scenarios:

**Authentication Methods:**
- **API Keys**: Simple token-based authentication for applications
- **JWT Tokens**: Session-based authentication for users
- **OAuth 2.0**: Third-party application integration
- **Certificate Authentication**: Mutual TLS for high-security environments
- **HMAC Signatures**: Message authentication for API requests

**Security Features:**
- **Role-Based Access Control**: Fine-grained permissions by user role
- **Rate Limiting**: Prevent abuse through request limiting
- **Audit Logging**: Comprehensive logging of all API access
- **Encryption**: TLS 1.3 for all communications

Authentication Methods
----------------------

API Keys
~~~~~~~~

API keys are the simplest authentication method, suitable for server-to-server communication and applications that don't require user context.

**Generating API Keys**
.. code-block:: bash

   # Generate a new API key via CLI
   cargo run -- generate-api-key --name "My Application" --permissions "read,write"

   # Generate via REST API (requires admin privileges)
   curl -X POST https://api.provchain-org.com/v1/api-keys \
     -H "Authorization: Bearer ADMIN_JWT_TOKEN" \
     -H "Content-Type: application/json" \
     -d '{
       "name": "My Application",
       "permissions": ["read", "write"],
       "expires_in": 86400
     }'

**Using API Keys**
.. code-block:: http

   GET /api/v1/status
   Authorization: Bearer pk_1234567890abcdef1234567890abcdef

.. code-block:: javascript
   // JavaScript example
   const response = await fetch('https://api.provchain-org.com/api/v1/status', {
     headers: {
       'Authorization': 'Bearer pk_1234567890abcdef1234567890abcdef'
     }
   });

.. code-block:: python
   # Python example
   import requests
   
   response = requests.get(
       'https://api.provchain-org.com/api/v1/status',
       headers={'Authorization': 'Bearer pk_1234567890abcdef1234567890abcdef'}
   )

JWT Tokens
~~~~~~~~~~

JWT (JSON Web Token) authentication is used for user-based access where individual user context is important.

**Obtaining JWT Tokens**
.. code-block:: http

   POST /auth/login
   Content-Type: application/json

   {
     "username": "user@example.com",
     "password": "secure-password"
   }

**Response:**
.. code-block:: json

   {
     "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
     "token_type": "Bearer",
     "expires_in": 3600,
     "refresh_token": "refresh_1234567890abcdef"
   }

**Using JWT Tokens**
.. code-block:: http

   GET /api/v1/user/profile
   Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

OAuth 2.0
~~~~~~~~~

OAuth 2.0 enables third-party applications to access ProvChainOrg APIs on behalf of users without exposing credentials.

**Authorization Code Flow**
.. code-block:: http

   # Redirect user to authorization endpoint
   GET /oauth/authorize?
     response_type=code&
     client_id=CLIENT_ID&
     redirect_uri=https://your-app.com/callback&
     scope=read write&
     state=xyz

**Token Exchange**
.. code-block:: http

   POST /oauth/token
   Content-Type: application/x-www-form-urlencoded
   Authorization: Basic BASE64_ENCODED_CLIENT_CREDENTIALS

   grant_type=authorization_code&
   code=AUTHORIZATION_CODE&
   redirect_uri=https://your-app.com/callback

Certificate Authentication
~~~~~~~~~~~~~~~~~~~~~~~~~~

Certificate-based authentication provides the highest level of security through mutual TLS authentication.

**Client Certificate Setup**
.. code-block:: bash

   # Generate client certificate
   openssl req -new -newkey rsa:2048 -nodes -keyout client.key -out client.csr

   # Submit CSR to ProvChainOrg for signing
   curl -X POST https://api.provchain-org.com/v1/certificates \
     -H "Authorization: Bearer ADMIN_API_KEY" \
     -F "csr=@client.csr" \
     -F "name=My Application"

**Using Client Certificates**
.. code-block:: bash

   # Use certificate with curl
   curl --cert client.crt --key client.key \
     https://api.provchain-org.com/api/v1/status

HMAC Signatures
~~~~~~~~~~~~~~~

HMAC signatures provide message authentication for API requests, ensuring both authenticity and integrity.

**HMAC Signature Generation**
.. code-block:: python

   import hmac
   import hashlib
   import base64
   import time
   
   def generate_hmac_signature(api_key, secret_key, method, url, body=''):
       # Create signature string
       timestamp = str(int(time.time()))
       signature_string = f"{method}\n{url}\n{body}\n{timestamp}"
       
       # Generate HMAC signature
       signature = hmac.new(
           secret_key.encode(),
           signature_string.encode(),
           hashlib.sha256
       ).digest()
       
       signature_b64 = base64.b64encode(signature).decode()
       
       return signature_b64, timestamp

**Using HMAC Signatures**
.. code-block:: http

   POST /api/v1/data
   Content-Type: text/turtle
   X-API-Key: pk_1234567890abcdef
   X-Timestamp: 1640995200
   X-Signature: Base64EncodedHMACSignature

Role-Based Access Control
------------------------

ProvChainOrg implements fine-grained access control through roles and permissions.

**User Roles**
.. list-table::
   :header-rows: 1
   :widths: 20 40 40

   * - Role
     - Description
     - Permissions
   * - **Viewer**
     - Read-only access to public data
     - read_public_data
   * - **User**
     - Standard user with read/write access to their data
     - read_data, write_data, query_data
   * - **Manager**
     - Business user with extended permissions
     - user_permissions + manage_batches, generate_reports
   * - **Administrator**
     - System administrator with full access
     - all_permissions + user_management, system_config
   * - **Auditor**
     - Compliance auditor with read-only access
     - read_all_data, audit_logs, compliance_reports

**Resource-Level Permissions**
Permissions can be granted at the resource level for fine-grained control:

.. code-block:: json

   {
     "user_id": "user_123",
     "permissions": {
       "organization:acme": {
         "read": true,
         "write": true,
         "delete": false
       },
       "organization:competitor": {
         "read": false,
         "write": false,
         "delete": false
       }
     }
   }

Rate Limiting
------------

API endpoints are rate limited to prevent abuse and ensure fair usage.

**Rate Limits by Authentication Method**
.. list-table::
   :header-rows: 1
   :widths: 25 25 25 25

   * - Authentication Method
     - Read Operations
     - Write Operations
     - Special Operations
   * - **API Key**
     - 1000 requests/minute
     - 100 requests/minute
     - 10 requests/minute
   * - **JWT Token**
     - 500 requests/minute
     - 50 requests/minute
     - 5 requests/minute
   * - **OAuth 2.0**
     - 200 requests/minute
     - 20 requests/minute
     - 2 requests/minute
   * - **Certificate Auth**
     - Unlimited (with monitoring)
     - 1000 requests/minute
     - 100 requests/minute

**Rate Limit Headers**
All API responses include rate limit information:

.. code-block:: http

   HTTP/1.1 200 OK
   X-RateLimit-Limit: 1000
   X-RateLimit-Remaining: 999
   X-RateLimit-Reset: 1640995260
   X-RateLimit-Policy: api_key_read

**Handling Rate Limits**
When a rate limit is exceeded, the API returns a 429 status:

.. code-block:: http

   HTTP/1.1 429 Too Many Requests
   X-RateLimit-Limit: 1000
   X-RateLimit-Remaining: 0
   X-RateLimit-Reset: 1640995260
   Retry-After: 60

   {
     "error": {
       "code": "RATE_LIMIT_EXCEEDED",
       "message": "Rate limit exceeded. Try again in 60 seconds."
     }
   }

Security Best Practices
----------------------

API Key Security
~~~~~~~~~~~~~~~

1. **Storage**: Store API keys securely, never in source code
2. **Rotation**: Regularly rotate API keys
3. **Scope**: Use least privilege principle
4. **Monitoring**: Monitor API key usage for anomalies

.. code-block:: python
   # ❌ Bad: Hardcoded API key
   API_KEY = "pk_1234567890abcdef1234567890abcdef"
   
   # ✅ Good: Environment variable
   import os
   API_KEY = os.getenv('PROVCHAIN_API_KEY')

JWT Security
~~~~~~~~~~~~

1. **Storage**: Store JWT tokens securely (HttpOnly cookies)
2. **Expiration**: Use short-lived access tokens with refresh tokens
3. **Validation**: Always validate JWT signatures and claims
4. **Revocation**: Implement token revocation for compromised tokens

.. code-block:: javascript
   // ✅ Good: Secure JWT handling
   const token = await fetch('/auth/login', {
     method: 'POST',
     body: JSON.stringify({username, password}),
     credentials: 'include' // Store in HttpOnly cookie
   });

Certificate Security
~~~~~~~~~~~~~~~~~~~~

1. **Storage**: Store private keys securely with proper permissions
2. **Rotation**: Regularly rotate certificates before expiration
3. **Revocation**: Implement certificate revocation checking
4. **Validation**: Validate certificate chains properly

HMAC Security
~~~~~~~~~~~~~

1. **Secret Storage**: Store HMAC secrets securely
2. **Timestamp Validation**: Validate request timestamps to prevent replay attacks
3. **Signature Verification**: Always verify signatures before processing requests
4. **Nonce Usage**: Use nonces to prevent replay attacks for critical operations

Audit Logging
-------------

All API access is logged for security monitoring and compliance.

**Log Information**
.. code-block:: json

   {
     "timestamp": "2025-01-15T10:30:00Z",
     "request_id": "req_1234567890abcdef",
     "user_id": "user_123",
     "ip_address": "192.168.1.100",
     "user_agent": "Mozilla/5.0 (compatible; MyApp/1.0)",
     "method": "POST",
     "path": "/api/v1/data",
     "status_code": 201,
     "response_time_ms": 45,
     "authentication_method": "api_key",
     "rate_limit_remaining": 99
   }

**Compliance Features**
- **Immutable Logs**: Logs cannot be modified or deleted
- **Chain of Custody**: Complete audit trail for compliance
- **Export Options**: Export logs for external analysis
- **Retention Policies**: Configurable log retention periods

Troubleshooting
--------------

Common Authentication Issues
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**Invalid API Key**
- **Error**: 401 Unauthorized with "Invalid API key"
- **Solution**: Verify API key format and validity

**Expired JWT Token**
- **Error**: 401 Unauthorized with "Token expired"
- **Solution**: Refresh token or re-authenticate

**Certificate Validation Failed**
- **Error**: SSL/TLS handshake failure
- **Solution**: Verify certificate validity and chain

**HMAC Signature Mismatch**
- **Error**: 401 Unauthorized with "Invalid signature"
- **Solution**: Verify signature generation algorithm and timestamp

**Rate Limit Exceeded**
- **Error**: 429 Too Many Requests
- **Solution**: Wait for rate limit reset or implement exponential backoff

Debugging Authentication
~~~~~~~~~~~~~~~~~~~~~~~~

**Enable Debug Logging**
.. code-block:: bash

   # Enable authentication debug logging
   export RUST_LOG=provchain_auth=debug
   cargo run

**Check Authentication Headers**
.. code-block:: bash

   # Debug authentication with curl
   curl -v -H "Authorization: Bearer YOUR_TOKEN" \
     https://api.provchain-org.com/api/v1/status

**Validate JWT Tokens**
.. code-block:: bash

   # Decode JWT token (without verification)
   echo "YOUR_JWT_TOKEN" | cut -d. -f1 | base64 -d
   echo "YOUR_JWT_TOKEN" | cut -d. -f2 | base64 -d

Best Practices
-------------

1. **Use Appropriate Authentication**: Choose the right method for your use case
2. **Implement Proper Error Handling**: Handle authentication failures gracefully
3. **Monitor API Usage**: Track usage patterns and anomalies
4. **Regular Security Audits**: Periodically review authentication configurations
5. **Keep Credentials Secure**: Never expose credentials in client-side code
6. **Use HTTPS**: Always use encrypted connections
7. **Implement Rate Limiting**: Protect against abuse in your applications
8. **Log Security Events**: Maintain audit trails for security incidents

Example Implementations
----------------------

Python Implementation
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import requests
   import os
   import time
   import hmac
   import hashlib
   import base64
   
   class ProvChainAuth:
       def __init__(self, base_url, api_key=None, secret_key=None):
           self.base_url = base_url
           self.api_key = api_key or os.getenv('PROVCHAIN_API_KEY')
           self.secret_key = secret_key or os.getenv('PROVCHAIN_SECRET_KEY')
           self.session = requests.Session()
           self.session.headers.update({
               'Authorization': f'Bearer {self.api_key}'
           })
       
       def make_request(self, method, endpoint, data=None):
           url = f"{self.base_url}{endpoint}"
           
           # Add HMAC signature for write operations
           if method in ['POST', 'PUT', 'DELETE'] and self.secret_key:
               signature, timestamp = self._generate_hmac_signature(
                   method, endpoint, data or ''
               )
               self.session.headers.update({
                   'X-Timestamp': timestamp,
                   'X-Signature': signature
               })
           
           response = self.session.request(method, url, json=data)
           response.raise_for_status()
           return response.json()
       
       def _generate_hmac_signature(self, method, endpoint, body=''):
           timestamp = str(int(time.time()))
           signature_string = f"{method}\n{endpoint}\n{body}\n{timestamp}"
           
           signature = hmac.new(
               self.secret_key.encode(),
               signature_string.encode(),
               hashlib.sha256
           ).digest()
           
           return base64.b64encode(signature).decode(), timestamp

JavaScript Implementation
~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: javascript

   class ProvChainAuth {
       constructor(baseUrl, apiKey = null, secretKey = null) {
           this.baseUrl = baseUrl;
           this.apiKey = apiKey || process.env.PROVCHAIN_API_KEY;
           this.secretKey = secretKey || process.env.PROVCHAIN_SECRET_KEY;
       }
       
       async makeRequest(method, endpoint, data = null) {
           const url = `${this.baseUrl}${endpoint}`;
           const headers = {
               'Authorization': `Bearer ${this.apiKey}`,
               'Content-Type': 'application/json'
           };
           
           // Add HMAC signature for write operations
           if (['POST', 'PUT', 'DELETE'].includes(method) && this.secretKey) {
               const { signature, timestamp } = await this._generateHmacSignature(
                   method, endpoint, data ? JSON.stringify(data) : ''
               );
               headers['X-Timestamp'] = timestamp;
               headers['X-Signature'] = signature;
           }
           
           const response = await fetch(url, {
               method,
               headers,
               body: data ? JSON.stringify(data) : undefined
           });
           
           if (!response.ok) {
               throw new Error(`HTTP error! status: ${response.status}`);
           }
           
           return await response.json();
       }
       
       async _generateHmacSignature(method, endpoint, body = '') {
           const timestamp = Math.floor(Date.now() / 1000).toString();
           const signatureString = `${method}\n${endpoint}\n${body}\n${timestamp}`;
           
           const encoder = new TextEncoder();
           const key = await crypto.subtle.importKey(
               'raw',
               encoder.encode(this.secretKey),
               { name: 'HMAC', hash: 'SHA-256' },
               false,
               ['sign']
           );
           
           const signature = await crypto.subtle.sign(
               'HMAC',
               key,
               encoder.encode(signatureString)
           );
           
           const signatureB64 = btoa(String.fromCharCode(...new Uint8Array(signature)));
           return { signature: signatureB64, timestamp };
       }
   }

Getting Help
------------

For authentication-related issues:

1. **Check Documentation**: Review this authentication guide
2. **Review Error Messages**: Examine detailed error responses
3. **Check Logs**: Review audit logs for authentication attempts
4. **Contact Support**: Reach out for enterprise support when needed

**Support Channels:**
- **Documentation**: Comprehensive guides and API references
- **Issue Tracker**: Report bugs and feature requests
- **Community Forum**: Peer support and best practices
- **Enterprise Support**: Commercial support options

.. raw:: html

   <div class="footer-note">
     <p><strong>Need help with authentication?</strong> Check the <a href="client-libraries.html">Client Libraries</a> for ready-to-use implementations or visit our <a href="https://github.com/provchain-org">GitHub repositories</a> for examples.</p>
   </div>
