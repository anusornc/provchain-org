REST API Reference
==================

The ProvChainOrg REST API provides a standard HTTP interface for interacting with the semantic blockchain platform. All endpoints return JSON responses and support standard HTTP methods for CRUD operations.

Base URL
--------

```
http://localhost:8080/api
```

Authentication
--------------

All API endpoints require authentication using API keys. Include your API key in the request headers:

```http
Authorization: Bearer YOUR_API_KEY
```

API keys can be generated through the web interface or via the CLI:

```bash
cargo run -- generate-api-key
```

Response Format
---------------

All API responses follow this JSON structure:

```json
{
  "success": true,
  "data": {},
  "message": "Operation completed successfully",
  "timestamp": "2025-01-14T18:30:00Z"
}
```

Error Responses
---------------

Error responses include detailed information about what went wrong:

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid RDF data format",
    "details": {
      "field": "data",
      "issue": "Malformed Turtle syntax"
    }
  },
  "timestamp": "2025-01-14T18:30:00Z"
}
```

Status Codes
------------

| Code | Description |
|------|-------------|
| 200 | OK - Request successful |
| 201 | Created - Resource created successfully |
| 400 | Bad Request - Invalid request parameters |
| 401 | Unauthorized - Authentication required |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource not found |
| 422 | Unprocessable Entity - Validation failed |
| 500 | Internal Server Error - Server error |

Endpoints
---------

Blockchain Status
~~~~~~~~~~~~~~~~~

Get the current status of the blockchain.

**Endpoint:** ``GET /status``

**Response:**
```json
{
  "success": true,
  "data": {
    "blockchain": {
      "current_height": 42,
      "latest_block_hash": "0x4a7b2c8f9e1d3a5b...",
      "total_transactions": 156,
      "network_status": "healthy"
    },
    "rdf_store": {
      "total_triples": 1247,
      "named_graphs": 42,
      "query_performance": "excellent"
    },
    "ontology": {
      "loaded": true,
      "validation_enabled": true,
      "last_updated": "2025-01-14T18:25:00Z"
    }
  }
}
```

Add RDF Data
~~~~~~~~~~~~

Add new RDF data as a blockchain block.

**Endpoint:** ``POST /data``

**Headers:**
```
Content-Type: text/turtle
Authorization: Bearer YOUR_API_KEY
```

**Request Body:**
```turtle
@prefix : <http://example.org/supply-chain#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

:Batch001 a :ProductBatch ;
    :hasBatchID "BATCH-001" ;
    :product :OrganicTomatoes ;
    :harvestDate "2025-01-14"^^xsd:date ;
    :originFarm :GreenValleyFarm .
```

**Response:**
```json
{
  "success": true,
  "data": {
    "block_index": 43,
    "block_hash": "0x8f3e2d1c9b8a7654...",
    "timestamp": "2025-01-14T18:30:15Z",
    "validation_passed": true
  }
}
```

Query Blockchain
~~~~~~~~~~~~~~~

Execute a SPARQL query against the blockchain.

**Endpoint:** ``POST /query``

**Headers:**
```
Content-Type: application/sparql-query
Authorization: Bearer YOUR_API_KEY
```

**Request Body:**
```sparql
PREFIX : <http://example.org/supply-chain#>
SELECT ?batch ?product ?farm WHERE {
    ?batch a :ProductBatch ;
           :product ?product ;
           :originFarm ?farm .
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "query_type": "SELECT",
    "result_count": 3,
    "results": [
      {
        "batch": "http://example.org/supply-chain#Batch001",
        "product": "http://example.org/supply-chain#OrganicTomatoes",
        "farm": "http://example.org/supply-chain#GreenValleyFarm"
      },
      {
        "batch": "http://example.org/supply-chain#Batch002",
        "product": "http://example.org/supply-chain#OrganicCarrots",
        "farm": "http://example.org/supply-chain#SunnyMeadowFarm"
      }
    ]
  }
}
```

Get Block
~~~~~~~~~

Retrieve a specific block by its index.

**Endpoint:** ``GET /blocks/{index}``

**Response:**
```json
{
  "success": true,
  "data": {
    "index": 42,
    "timestamp": "2025-01-14T18:25:00Z",
    "previous_hash": "0x1a2b3c4d5e6f7890...",
    "hash": "0x4a7b2c8f9e1d3a5b...",
    "data": {
      "graph_name": "http://provchain.org/block/42",
      "turtle_data": "@prefix : <http://example.org/supply-chain#> .\n:Batch001 a :ProductBatch ;",
      "triple_count": 15
    }
  }
}
```

Get Blocks
~~~~~~~~~~

Retrieve a range of blocks with pagination.

**Endpoint:** ``GET /blocks``

**Query Parameters:**
- ``limit`` (optional, default: 10) - Number of blocks to return
- ``offset`` (optional, default: 0) - Starting block index
- ``format`` (optional, default: "summary") - Response format: "summary" or "full"

**Response:**
```json
{
  "success": true,
  "data": {
    "pagination": {
      "limit": 10,
      "offset": 0,
      "total_blocks": 42
    },
    "blocks": [
      {
        "index": 42,
        "timestamp": "2025-01-14T18:25:00Z",
        "hash": "0x4a7b2c8f9e1d3a5b...",
        "triple_count": 15
      },
      {
        "index": 41,
        "timestamp": "2025-01-14T18:20:00Z",
        "hash": "0x8f3e2d1c9b8a7654...",
        "triple_count": 12
      }
    ]
  }
}
```

Validate Blockchain
~~~~~~~~~~~~~~~~~~

Validate the integrity of the blockchain.

**Endpoint:** ``POST /validate``

**Response:**
```json
{
  "success": true,
  "data": {
    "is_valid": true,
    "total_blocks": 42,
    "validation_time_ms": 245,
    "issues": []
  }
}
```

Export Data
~~~~~~~~~~~

Export blockchain data in various formats.

**Endpoint:** ``GET /export``

**Query Parameters:**
- ``format`` (required) - Export format: "turtle", "jsonld", "ntriples", "rdfxml"
- ``graph`` (optional) - Specific named graph to export
- ``compression`` (optional) - Compression: "gzip", "none"

**Response:**
Content-Type depends on the requested format.

**Example (Turtle):**
```turtle
@prefix : <http://example.org/supply-chain#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

:Batch001 a :ProductBatch ;
    :hasBatchID "BATCH-001" ;
    :product :OrganicTomatoes ;
    :harvestDate "2025-01-14"^^xsd:date .
```

Get Ontology
~~~~~~~~~~~~

Retrieve the current ontology configuration.

**Endpoint:** ``GET /ontology``

**Response:**
```json
{
  "success": true,
  "data": {
    "loaded": true,
    "path": "ontology/traceability.owl.ttl",
    "graph_name": "http://provchain.org/ontology",
    "validation_enabled": true,
    "last_updated": "2025-01-14T18:25:00Z",
    "classes": [
      "ProductBatch",
      "ProcessingActivity",
      "EnvironmentalCondition"
    ]
  }
}
```

Validate RDF Data
~~~~~~~~~~~~~~~~~

Validate RDF data against the loaded ontology.

**Endpoint:** ``POST /ontology/validate``

**Headers:**
```
Content-Type: text/turtle
Authorization: Bearer YOUR_API_KEY
```

**Request Body:**
```turtle
@prefix : <http://example.org/supply-chain#> .

:Batch001 a :ProductBatch ;
    :hasBatchID "TEST-BATCH" .
```

**Response:**
```json
{
  "success": true,
  "data": {
    "is_valid": true,
    "validation_time_ms": 45,
    "issues": []
  }
}
```

Network Information
~~~~~~~~~~~~~~~~~~~

Get information about the P2P network.

**Endpoint:** ``GET /network``

**Response:**
```json
{
  "success": true,
  "data": {
    "node_id": "provchain-node-001",
    "is_authority": false,
    "connected_peers": 3,
    "known_peers": [
      {
        "id": "peer-001",
        "address": "192.168.1.100:8080",
        "last_seen": "2025-01-14T18:28:00Z"
      }
    ],
    "network_status": "healthy"
  }
}
```

Rate Limiting
------------

API endpoints are rate limited to prevent abuse:

- **Authentication endpoints**: 5 requests per minute
- **Read endpoints**: 100 requests per minute
- **Write endpoints**: 10 requests per minute
- **Query endpoints**: 50 requests per minute

Rate limit headers are included in responses:

```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1642125600
```

Webhooks
--------

Subscribe to blockchain events via webhooks.

**Endpoint:** ``POST /webhooks``

**Request Body:**
```json
{
  "url": "https://your-app.com/webhooks/provchain",
  "events": ["new_block", "validation_error", "network_change"],
  "secret": "your-webhook-secret"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "webhook_id": "wh_123456789",
    "url": "https://your-app.com/webhooks/provchain",
    "events": ["new_block", "validation_error", "network_change"],
    "created_at": "2025-01-14T18:30:00Z"
  }
}
```

Example Usage
------------

cURL
~~~~

```bash
# Get blockchain status
curl -X GET http://localhost:8080/api/status \
  -H "Authorization: Bearer YOUR_API_KEY"

# Add RDF data
curl -X POST http://localhost:8080/api/data \
  -H "Content-Type: text/turtle" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d "@/path/to/data.ttl"

# Execute SPARQL query
curl -X POST http://localhost:8080/api/query \
  -H "Content-Type: application/sparql-query" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d "SELECT ?batch ?product WHERE { ?batch a :ProductBatch ; :product ?product }"
```

Interactive API Explorer
~~~~~~~~~~~~~~~~~~~~~~~~

.. raw:: html

   <div class="api-explorer">
     <div class="api-request">
       <h4>Try the Status API</h4>
       <p>Method: <span class="method">GET</span></p>
       <p>URL: <span class="url">http://localhost:8080/api/status</span></p>
       <p>Headers: Authorization: Bearer YOUR_API_KEY</p>
       <button class="run-button">Execute Request</button>
     </div>
     <div class="api-response">
       # Response will appear here
     </div>
   </div>

Python
~~~~~~

```python
import requests
import json

class ProvChainAPI:
    def __init__(self, base_url, api_key):
        self.base_url = base_url
        self.headers = {
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json"
        }
    
    def get_status(self):
        response = requests.get(f"{self.base_url}/status", headers=self.headers)
        return response.json()
    
    def add_rdf_data(self, turtle_data):
        headers = self.headers.copy()
        headers["Content-Type"] = "text/turtle"
        response = requests.post(f"{self.base_url}/data", headers=headers, data=turtle_data)
        return response.json()
    
    def execute_query(self, sparql_query):
        headers = self.headers.copy()
        headers["Content-Type"] = "application/sparql-query"
        response = requests.post(f"{self.base_url}/query", headers=headers, data=sparql_query)
        return response.json()

# Usage
api = ProvChainAPI("http://localhost:8080/api", "YOUR_API_KEY")
status = api.get_status()
print(f"Blockchain height: {status['data']['blockchain']['current_height']}")

# Add data
rdf_data = """
@prefix : <http://example.org/supply-chain#> .
:Batch001 a :ProductBatch ; :hasBatchID "TEST-001" .
"""
result = api.add_rdf_data(rdf_data)
print(f"Added block: {result['data']['block_index']}")
```

JavaScript
~~~~~~~~~~

```javascript
class ProvChainAPI {
    constructor(baseUrl, apiKey) {
        this.baseUrl = baseUrl;
        this.headers = {
            'Authorization': `Bearer ${apiKey}`,
            'Content-Type': 'application/json'
        };
    }

    async getStatus() {
        const response = await fetch(`${this.baseUrl}/status`, {
            headers: this.headers
        });
        return await response.json();
    }

    async addRDFData(turtleData) {
        const headers = { ...this.headers };
        headers['Content-Type'] = 'text/turtle';
        
        const response = await fetch(`${this.baseUrl}/data`, {
            method: 'POST',
            headers: headers,
            body: turtleData
        });
        return await response.json();
    }

    async executeQuery(sparqlQuery) {
        const headers = { ...this.headers };
        headers['Content-Type'] = 'application/sparql-query';
        
        const response = await fetch(`${this.baseUrl}/query`, {
            method: 'POST',
            headers: headers,
            body: sparqlQuery
        });
        return await response.json();
    }
}

// Usage
const api = new ProvChainAPI('http://localhost:8080/api', 'YOUR_API_KEY');

api.getStatus().then(status => {
    console.log(`Blockchain height: ${status.data.blockchain.current_height}`);
});

const rdfData = `@prefix : <http://example.org/supply-chain#> .
:Batch001 a :ProductBatch ; :hasBatchID "TEST-001" .`;

api.addRDFData(rdfData).then(result => {
    console.log(`Added block: ${result.data.block_index}`);
});
```

Best Practices
--------------

1. **Error Handling**: Always check the `success` field in responses and handle errors appropriately.

2. **Rate Limiting**: Monitor rate limit headers and implement exponential backoff for failed requests.

3. **Authentication**: Store API keys securely and never expose them in client-side code.

4. **Batch Operations**: For bulk operations, consider implementing client-side batching to reduce API calls.

5. **Caching**: Cache frequently accessed data like blockchain status to reduce API calls.

6. **Webhooks**: Use webhooks for real-time updates instead of polling for new blocks.

7. **Validation**: Always validate data on the client side before sending to the API.

8. **Monitoring**: Implement logging and monitoring for API usage and errors.

Troubleshooting
---------------

Common Issues
~~~~~~~~~~~~~

**Authentication Failed**
   - Verify your API key is correct and not expired
   - Check that the Authorization header is properly formatted

**Rate Limit Exceeded**
   - Wait for the rate limit to reset (check X-RateLimit-Reset header)
   - Implement exponential backoff in your client

**Invalid RDF Data**
   - Validate your Turtle syntax before sending
   - Check that all required properties are included

**Query Timeout**
   - Optimize your SPARQL queries
   - Use LIMIT clauses for large result sets

**Network Connection Issues**
   - Verify the ProvChainOrg node is running
   - Check firewall settings and network connectivity

Getting Help
~~~~~~~~~~~~

- **API Documentation**: Refer to this documentation for detailed endpoint information
- **GitHub Issues**: Report bugs and request features
- **Community Support**: Join discussions for help and best practices
- **Support Email**: For enterprise customers, contact support@provchain-org.com
