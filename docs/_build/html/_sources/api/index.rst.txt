API Reference
============

Welcome to the ProvChainOrg API documentation! This comprehensive reference provides detailed information about all available APIs, including REST endpoints, WebSocket connections, SPARQL queries, and integration interfaces.

.. raw:: html

   <div class="hero-section">
     <div class="hero-content">
       <h1>API Documentation</h1>
       <p class="hero-subtitle">Complete reference for integrating with ProvChainOrg through REST APIs, WebSocket connections, and SPARQL endpoints</p>
       <div class="hero-badges">
         <span class="badge badge-api">REST API</span>
         <span class="badge badge-websocket">WebSocket</span>
         <span class="badge badge-sparql">SPARQL</span>
         <span class="badge badge-integration">Integration Ready</span>
       </div>
     </div>
   </div>

Overview
--------

The ProvChainOrg API provides multiple interfaces for integrating with the semantic blockchain platform. Each API serves different purposes and can be used independently or in combination to build comprehensive applications.

**API Components:**
- **REST API**: Standard HTTP endpoints for web applications
- **WebSocket API**: Real-time updates and bidirectional communication
- **SPARQL Endpoint**: W3C-compliant semantic data querying
- **GraphQL API**: Flexible query interface (experimental)
- **Webhooks**: Event-driven notifications
- **SDKs**: Language-specific client libraries

**Authentication:**
- API Key authentication
- JWT token-based authentication
- OAuth 2.0 integration
- Certificate-based authentication

**Rate Limiting:**
- Configurable request limits
- Tiered access levels
- Burst handling
- Quota management

**Security Features:**
- TLS/SSL encryption
- Request signing
- IP whitelisting
- Audit logging

Getting Started
---------------

**Prerequisites**
- Basic understanding of HTTP APIs
- Familiarity with JSON data format
- Knowledge of blockchain concepts
- Understanding of semantic web technologies

**Quick Start**
.. code-block:: bash
   # Install the ProvChainOrg CLI
   cargo install provchain-cli
   
   # Test API connectivity
   provchain-cli api health
   
   # Get API key
   provchain-cli auth create-api-key --name "my-application"

**API Base URLs**
- **Production**: ``https://api.provchain-org.com``
- **Staging**: ``https://staging-api.provchain-org.com``
- **Development**: ``http://localhost:8080``

**Common Headers**
.. code-block:: http
   Content-Type: application/json
   Authorization: Bearer YOUR_API_KEY
   X-API-Version: 1.0
   X-Request-ID: unique-request-id

REST API
--------

Authentication
~~~~~~~~~~~~~~

**API Key Authentication**
.. code-block:: http
   POST /api/v1/auth/api-key
   Host: api.provchain-org.com
   Content-Type: application/json
   
   {
     "name": "my-application",
     "description": "Production API key",
     "permissions": ["read", "write", "query"]
   }

**Response**
.. code-block:: json
   {
     "api_key": "pk_test_1234567890abcdef",
     "secret_key": "sk_test_1234567890abcdef",
     "permissions": ["read", "write", "query"],
     "created_at": "2024-01-15T10:30:00Z",
     "expires_at": "2024-12-31T23:59:59Z"
   }

**JWT Token Authentication**
.. code-block:: http
   POST /api/v1/auth/token
   Host: api.provchain-org.com
   Content-Type: application/json
   
   {
     "api_key": "pk_test_1234567890abcdef",
     "secret_key": "sk_test_1234567890abcdef"
   }

**Response**
.. code-block:: json
   {
     "access_token": "eyJhbGciOiJIUzI1NiIs...",
     "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
     "expires_in": 3600,
     "token_type": "Bearer"
   }

Blockchain API
~~~~~~~~~~~~~~

**Get Blockchain Status**
.. code-block:: http
   GET /api/v1/blockchain/status
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "height": 12345,
     "latest_block_hash": "0x4a7b2c8f9e1d3a5b...",
     "total_transactions": 98765,
     "peers": 5,
     "uptime": "15d 3h 42m",
     "version": "0.1.0"
   }

**Get Block by Height**
.. code-block:: http
   GET /api/v1/blocks/12345
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "height": 12345,
     "hash": "0x4a7b2c8f9e1d3a5b...",
     "previous_hash": "0x9e8d7c6b5a4f3e2d...",
     "timestamp": "2024-01-15T10:30:00Z",
     "transactions": [
       {
         "id": "tx_1234567890",
         "type": "supply_chain_data",
         "data": {
           "@context": "https://schema.org",
           "@type": "ProductBatch",
           "product": "Organic Tomatoes",
           "origin": "Green Valley Farm",
           "harvestDate": "2024-01-15"
         }
       }
     ]
   }

**Get Blocks by Range**
.. code-block:: http
   GET /api/v1/blocks?start=12000&end=12300
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "blocks": [
       {
         "height": 12000,
         "hash": "0x1a2b3c4d5e6f7a8b...",
         "timestamp": "2024-01-15T09:00:00Z"
       },
       {
         "height": 12001,
         "hash": "0x9b8a7c6d5e4f3a2b...",
         "timestamp": "2024-01-15T09:00:10Z"
       }
     ],
     "total": 301
   }

**Search Blocks**
.. code-block:: http
   GET /api/v1/blocks/search?query=organic+tomatoes&limit=10
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "blocks": [
       {
         "height": 12345,
         "hash": "0x4a7b2c8f9e1d3a5b...",
         "timestamp": "2024-01-15T10:30:00Z",
         "matches": [
           {
             "field": "data.product",
             "value": "Organic Tomatoes",
             "score": 0.95
           }
         ]
       }
     ],
     "total": 5
   }

Transaction API
~~~~~~~~~~~~~~~

**Submit Transaction**
.. code-block:: http
   POST /api/v1/transactions
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/json
   
   {
     "type": "supply_chain_data",
     "data": {
       "@context": "https://schema.org",
       "@type": "ProductBatch",
       "product": "Organic Tomatoes",
       "origin": "Green Valley Farm",
       "harvestDate": "2024-01-15",
       "temperature": "2-4°C",
       "humidity": "85%"
     }
   }

**Response**
.. code-block:: json
   {
     "tx_id": "tx_1234567890",
     "status": "pending",
     "height": null,
     "hash": null,
     "fee": 0.001,
     "timestamp": "2024-01-15T10:30:00Z"
   }

**Get Transaction by ID**
.. code-block:: http
   GET /api/v1/transactions/tx_1234567890
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "tx_id": "tx_1234567890",
     "status": "confirmed",
     "height": 12345,
     "hash": "0x4a7b2c8f9e1d3a5b...",
     "fee": 0.001,
     "timestamp": "2024-01-15T10:30:00Z",
     "data": {
       "@context": "https://schema.org",
       "@type": "ProductBatch",
       "product": "Organic Tomatoes",
       "origin": "Green Valley Farm",
       "harvestDate": "2024-01-15"
     }
   }

**Get Transactions by Address**
.. code-block:: http
   GET /api/v1/transactions?address=0x1234567890abcdef
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "transactions": [
       {
         "tx_id": "tx_1234567890",
         "status": "confirmed",
         "height": 12345,
         "timestamp": "2024-01-15T10:30:00Z"
       }
     ],
     "total": 42
   }

**Batch Transaction Submission**
.. code-block:: http
   POST /api/v1/transactions/batch
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/json
   
   {
     "transactions": [
       {
         "type": "supply_chain_data",
         "data": {
           "@context": "https://schema.org",
           "@type": "ProductBatch",
           "product": "Organic Tomatoes",
           "origin": "Green Valley Farm",
           "harvestDate": "2024-01-15"
         }
       },
       {
         "type": "environmental_data",
         "data": {
           "@context": "https://schema.org",
           "@type": "EnvironmentalCondition",
           "temperature": "2-4°C",
           "humidity": "85%",
           "timestamp": "2024-01-15T10:30:00Z"
         }
       }
     ]
   }

**Response**
.. code-block:: json
   {
     "batch_id": "batch_1234567890",
     "transactions": [
       {
         "tx_id": "tx_1234567891",
         "status": "pending",
         "fee": 0.001
       },
       {
         "tx_id": "tx_1234567892",
         "status": "pending",
         "fee": 0.001
       }
     ],
     "total_fee": 0.002,
     "timestamp": "2024-01-15T10:30:00Z"
   }

Supply Chain API
~~~~~~~~~~~~~~~~

**Add Product Batch**
.. code-block:: http
   POST /api/v1/supply-chain/batches
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/json
   
   {
     "batch_id": "batch_1234567890",
     "product": "Organic Tomatoes",
     "origin": "Green Valley Farm",
     "harvest_date": "2024-01-15",
     "quantity": 1000,
     "unit": "kg",
     "certifications": ["Organic", "Non-GMO"],
     "environmental_conditions": {
       "temperature": "2-4°C",
       "humidity": "85%",
       "co2_level": "400ppm"
     }
   }

**Response**
.. code-block:: json
   {
     "batch_id": "batch_1234567890",
     "tx_id": "tx_1234567890",
     "status": "confirmed",
     "height": 12345,
     "timestamp": "2024-01-15T10:30:00Z"
   }

**Trace Product Batch**
.. code-block:: http
   GET /api/v1/supply-chain/batches/batch_1234567890/trace
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "batch_id": "batch_1234567890",
     "product": "Organic Tomatoes",
     "origin": "Green Valley Farm",
     "current_location": "Distribution Center",
     "trace_history": [
       {
         "event": "harvest",
         "location": "Green Valley Farm",
         "timestamp": "2024-01-15T10:00:00Z",
         "data": {
           "temperature": "15-20°C",
           "humidity": "70%",
           "workers": 5
         }
       },
       {
         "event": "transport",
         "location": "Farm to Processing Plant",
         "timestamp": "2024-01-15T12:00:00Z",
         "data": {
           "temperature": "2-4°C",
           "humidity": "85%",
           "vehicle": "Refrigerated Truck"
         }
       }
     ]
   }

**Get Supply Chain Events**
.. code-block:: http
   GET /api/v1/supply-chain/events?batch_id=batch_1234567890
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "events": [
       {
         "event_id": "event_1234567890",
         "batch_id": "batch_1234567890",
         "event_type": "harvest",
         "location": "Green Valley Farm",
         "timestamp": "2024-01-15T10:00:00Z",
         "data": {
           "temperature": "15-20°C",
           "humidity": "70%",
           "workers": 5
         }
       }
     ],
     "total": 5
   }

**Query Supply Chain Data**
.. code-block:: http
   POST /api/v1/supply-chain/query
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/json
   
   {
     "query": {
       "product": "Organic Tomatoes",
       "origin": "Green Valley Farm",
       "date_range": {
         "start": "2024-01-01",
         "end": "2024-01-31"
       }
     }
   }

**Response**
.. code-block:: json
   {
     "results": [
       {
         "batch_id": "batch_1234567890",
         "product": "Organic Tomatoes",
         "origin": "Green Valley Farm",
         "harvest_date": "2024-01-15",
         "quantity": 1000,
         "status": "in_transit"
       }
     ],
     "total": 3
   }

Ontology API
~~~~~~~~~~~~

**Get Ontology Schema**
.. code-block:: http
   GET /api/v1/ontology/schema
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "schema": {
       "@context": "https://schema.org",
       "@type": "Ontology",
       "classes": [
         {
           "@id": "ProductBatch",
           "@type": "Class",
           "description": "A batch of products in the supply chain",
           "properties": [
             {
               "@id": "product",
               "@type": "Property",
               "range": "Product",
               "cardinality": "1"
             },
             {
               "@id": "origin",
               "@type": "Property",
               "range": "Farm",
               "cardinality": "1"
             }
           ]
         }
       ]
     }
   }

**Validate Data Against Ontology**
.. code-block:: http
   POST /api/v1/ontology/validate
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/json
   
   {
     "data": {
       "@context": "https://schema.org",
       "@type": "ProductBatch",
       "product": "Organic Tomatoes",
       "origin": "Green Valley Farm"
     },
     "ontology": "supply_chain"
   }

**Response**
.. code-block:: json
   {
     "valid": true,
     "errors": [],
     "warnings": [],
     "suggestions": []
   }

**Get Ontology Classes**
.. code-block:: http
   GET /api/v1/ontology/classes
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "classes": [
       {
         "id": "ProductBatch",
         "label": "Product Batch",
         "description": "A batch of products in the supply chain",
         "properties": [
           {
             "id": "product",
             "label": "Product",
             "type": "Product",
             "required": true
           },
           {
             "id": "origin",
             "label": "Origin",
             "type": "Farm",
             "required": true
           }
         ]
       }
     ]
   }

**Get Ontology Properties**
.. code-block:: http
   GET /api/v1/ontology/properties
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "properties": [
       {
         "id": "product",
         "label": "Product",
         "description": "The product being tracked",
         "domain": "ProductBatch",
         "range": "Product",
         "cardinality": "1",
         "required": true
       },
       {
         "id": "origin",
         "label": "Origin",
         "description": "The origin of the product",
         "domain": "ProductBatch",
         "range": "Farm",
         "cardinality": "1",
         "required": true
       }
     ]
   }

WebSocket API
-------------

**Connection**
.. code-block:: javascript
   const ws = new WebSocket('wss://api.provchain-org.com/ws');
   
   ws.onopen = function(event) {
     console.log('WebSocket connected');
     
     // Authenticate
     ws.send(JSON.stringify({
       type: 'auth',
       token: 'YOUR_JWT_TOKEN'
     }));
   };

**New Block Notification**
.. code-block:: javascript
   ws.onmessage = function(event) {
     const message = JSON.parse(event.data);
     
     if (message.type === 'new_block') {
       console.log('New block:', message.block);
       // Update UI or take action
     }
   };

**Transaction Status Updates**
.. code-block:: javascript
   ws.onmessage = function(event) {
     const message = JSON.parse(event.data);
     
     if (message.type === 'transaction_update') {
       console.log('Transaction update:', message.transaction);
       // Update transaction status in UI
     }
   };

**Supply Chain Events**
.. code-block:: javascript
   ws.onmessage = function(event) {
     const message = JSON.parse(event.data);
     
     if (message.type === 'supply_chain_event') {
       console.log('Supply chain event:', message.event);
       // Process real-time supply chain events
     }
   };

**Query Results**
.. code-block:: javascript
   ws.onmessage = function(event) {
     const message = JSON.parse(event.data);
     
     if (message.type === 'query_result') {
       console.log('Query result:', message.results);
       // Update query results in UI
     }
   };

**Subscription Management**
.. code-block:: javascript
   // Subscribe to new blocks
   ws.send(JSON.stringify({
     type: 'subscribe',
     channel: 'blocks'
   }));
   
   // Subscribe to transaction updates
   ws.send(JSON.stringify({
     type: 'subscribe',
     channel: 'transactions'
   }));
   
   // Subscribe to supply chain events
   ws.send(JSON.stringify({
     type: 'subscribe',
     channel: 'supply_chain'
   }));
   
   // Unsubscribe from channels
   ws.send(JSON.stringify({
     type: 'unsubscribe',
     channel: 'blocks'
   }));

SPARQL API
-----------

**SPARQL Query Endpoint**
.. code-block:: http
   POST /sparql
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/sparql-query
   
   SELECT ?batch ?product ?farm WHERE {
     ?batch a :ProductBatch ;
            :product ?product ;
            :originFarm ?farm .
     ?farm :farmName "Green Valley Farm" .
   }

**Response**
.. code-block:: json
   {
     "head": {
       "vars": ["batch", "product", "farm"]
     },
     "results": {
       "bindings": [
         {
           "batch": { "type": "uri", "value": "batch_1234567890" },
           "product": { "type": "literal", "value": "Organic Tomatoes" },
           "farm": { "type": "uri", "value": "farm_1234567890" }
         }
       ]
     }
   }

**SPARQL Update Endpoint**
.. code-block:: http
   POST /sparql
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/sparql-update
   
   INSERT DATA {
     :batch_1234567890 a :ProductBatch ;
                       :product "Organic Tomatoes" ;
                       :originFarm :farm_1234567890 .
   }

**Response**
.. code-block:: json
   {
     "status": "success",
     "message": "Data inserted successfully",
     "timestamp": "2024-01-15T10:30:00Z"
   }

**SPARQL Describe**
.. code-block:: http
   POST /sparql
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/sparql-query
   
   DESCRIBE ?batch WHERE {
     ?batch a :ProductBatch ;
            :product "Organic Tomatoes" .
   }

**Response**
.. code-block:: turtle
   @prefix : <http://example.org/> .
   
   :batch_1234567890 a :ProductBatch ;
                    :product "Organic Tomatoes" ;
                    :originFarm :farm_1234567890 ;
                    :harvestDate "2024-01-15" .

**SPARQL Ask**
.. code-block:: http
   POST /sparql
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/sparql-query
   
   ASK WHERE {
     ?batch a :ProductBatch ;
            :product "Organic Tomatoes" ;
            :originFarm :farm_1234567890 .
   }

**Response**
.. code-block:: json
   {
     "boolean": true
   }

**SPARQL Construct**
.. code-block:: http
   POST /sparql
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/sparql-query
   
   CONSTRUCT { ?batch ?p ?o }
   WHERE {
     ?batch a :ProductBatch ;
            ?p ?o .
   }

**Response**
.. code-block:: turtle
   @prefix : <http://example.org/> .
   
   :batch_1234567890 a :ProductBatch ;
                    :product "Organic Tomatoes" ;
                    :originFarm :farm_1234567890 ;
                    :harvestDate "2024-01-15" .

GraphQL API
-----------

**GraphQL Schema**
.. code-block:: graphql
   type Query {
     blockchainStatus: BlockchainStatus
     block(height: Int!): Block
     blocks(start: Int, end: Int): [Block]
     transaction(id: ID!): Transaction
     transactionsByAddress(address: String!): [Transaction]
     supplyChainBatch(id: ID!): SupplyChainBatch
     traceProductBatch(id: ID!): TraceResult
     ontologySchema: OntologySchema
   }
   
   type Mutation {
     submitTransaction(data: JSON!): Transaction
     addSupplyChainBatch(batch: SupplyChainBatchInput!): SupplyChainBatch
     validateData(data: JSON!, ontology: String!): ValidationResult
   }
   
   type Subscription {
     newBlock: Block
     transactionUpdate: Transaction
     supplyChainEvent: SupplyChainEvent
   }

**GraphQL Query Example**
.. code-block:: graphql
   query GetSupplyChainTrace {
     traceProductBatch(id: "batch_1234567890") {
       batchId
       product
       origin
       traceHistory {
         event
         location
         timestamp
         data
       }
     }
   }

**Response**
.. code-block:: json
   {
     "data": {
       "traceProductBatch": {
         "batchId": "batch_1234567890",
         "product": "Organic Tomatoes",
         "origin": "Green Valley Farm",
         "traceHistory": [
           {
             "event": "harvest",
             "location": "Green Valley Farm",
             "timestamp": "2024-01-15T10:00:00Z",
             "data": {
               "temperature": "15-20°C",
               "humidity": "70%",
               "workers": 5
             }
           }
         ]
       }
     }
   }

**GraphQL Mutation Example**
.. code-block:: graphql
   mutation AddSupplyChainBatch {
     addSupplyChainBatch(
       batch: {
         batchId: "batch_1234567890",
         product: "Organic Tomatoes",
         origin: "Green Valley Farm",
         harvestDate: "2024-01-15",
         quantity: 1000,
         unit: "kg"
       }
     ) {
       batchId
       transaction {
         id
         status
         timestamp
       }
     }
   }

**Response**
.. code-block:: json
   {
     "data": {
       "addSupplyChainBatch": {
         "batchId": "batch_1234567890",
         "transaction": {
           "id": "tx_1234567890",
           "status": "confirmed",
           "timestamp": "2024-01-15T10:30:00Z"
         }
       }
     }
   }

**GraphQL Subscription Example**
.. code-block:: graphql
   subscription ListenForNewBlocks {
     newBlock {
       height
       hash
       timestamp
       transactionCount
     }
   }

**Response**
.. code-block:: json
   {
     "data": {
       "newBlock": {
         "height": 12345,
         "hash": "0x4a7b2c8f9e1d3a5b...",
         "timestamp": "2024-01-15T10:30:00Z",
         "transactionCount": 3
       }
     }
   }

Webhooks
--------

**Create Webhook**
.. code-block:: http
   POST /api/v1/webhooks
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/json
   
   {
     "name": "New Block Notification",
     "url": "https://your-app.com/webhooks/new-block",
     "events": ["new_block", "transaction_confirmed"],
     "secret": "your-webhook-secret",
     "active": true
   }

**Response**
.. code-block:: json
   {
     "webhook_id": "wh_1234567890",
     "name": "New Block Notification",
     "url": "https://your-app.com/webhooks/new-block",
     "events": ["new_block", "transaction_confirmed"],
     "secret": "your-webhook-secret",
     "active": true,
     "created_at": "2024-01-15T10:30:00Z"
   }

**Webhook Payload Example**
.. code-block:: json
   {
     "event": "new_block",
     "webhook_id": "wh_1234567890",
     "timestamp": "2024-01-15T10:30:00Z",
     "data": {
       "height": 12345,
       "hash": "0x4a7b2c8f9e1d3a5b...",
       "previous_hash": "0x9e8d7c6b5a4f3e2d...",
       "timestamp": "2024-01-15T10:30:00Z",
       "transactions": [
         {
           "id": "tx_1234567890",
           "type": "supply_chain_data",
           "data": {
             "@context": "https://schema.org",
             "@type": "ProductBatch",
             "product": "Organic Tomatoes",
             "origin": "Green Valley Farm"
           }
         }
       ]
     }
   }

**Webhook Signature Verification**
.. code-block:: javascript
   const crypto = require('crypto');
   
   function verifyWebhookSignature(payload, signature, secret) {
     const hmac = crypto.createHmac('sha256', secret);
     const digest = hmac.update(payload).digest('hex');
     return crypto.timingSafeEqual(
       Buffer.from(signature),
       Buffer.from(`sha256=${digest}`)
     );
   }

**Response Handling**
.. code-block:: http
   HTTP/1.1 200 OK
   Content-Type: application/json
   
   {
     "status": "received",
     "webhook_id": "wh_1234567890",
     "timestamp": "2024-01-15T10:30:00Z"
   }

SDKs and Libraries
------------------

**Python SDK**
.. code-block:: python
   from provchain import ProvChainClient
   
   # Initialize client
   client = ProvChainClient(
     api_key="pk_test_1234567890abcdef",
     base_url="https://api.provchain-org.com"
   )
   
   # Get blockchain status
   status = client.blockchain.get_status()
   print(f"Current height: {status.height}")
   
   # Submit transaction
   transaction = client.transactions.submit({
     "type": "supply_chain_data",
     "data": {
       "@context": "https://schema.org",
       "@type": "ProductBatch",
       "product": "Organic Tomatoes",
       "origin": "Green Valley Farm"
     }
   })
   
   # Query supply chain data
   results = client.supply_chain.query({
     "product": "Organic Tomatoes",
     "origin": "Green Valley Farm"
   })

**JavaScript SDK**
.. code-block:: javascript
   import { ProvChainClient } from '@provchain/client';
   
   // Initialize client
   const client = new ProvChainClient({
     apiKey: 'pk_test_1234567890abcdef',
     baseUrl: 'https://api.provchain-org.com'
   });
   
   // Get blockchain status
   const status = await client.blockchain.getStatus();
   console.log(`Current height: ${status.height}`);
   
   // Submit transaction
   const transaction = await client.transactions.submit({
     type: 'supply_chain_data',
     data: {
       '@context': 'https://schema.org',
       '@type': 'ProductBatch',
       product: 'Organic Tomatoes',
       origin: 'Green Valley Farm'
     }
   });
   
   // Query supply chain data
   const results = await client.supplyChain.query({
     product: 'Organic Tomatoes',
     origin: 'Green Valley Farm'
   });

**Java SDK**
.. code-block:: java
   import org.provchain.client.ProvChainClient;
   import org.provchain.model.BlockchainStatus;
   
   public class ProvChainExample {
     public static void main(String[] args) {
       // Initialize client
       ProvChainClient client = new ProvChainClient(
         "pk_test_1234567890abcdef",
         "https://api.provchain-org.com"
       );
       
       // Get blockchain status
       BlockchainStatus status = client.blockchain().getStatus();
       System.out.println("Current height: " + status.getHeight());
       
       // Submit transaction
       Transaction transaction = client.transactions().submit(
         new TransactionRequest()
           .setType("supply_chain_data")
           .setData(Map.of(
             "@context", "https://schema.org",
             "@type", "ProductBatch",
             "product", "Organic Tomatoes",
             "origin", "Green Valley Farm"
           ))
       );
     }
   }

**Go SDK**
.. code-block:: go
   package main
   
   import (
     "fmt"
     "github.com/provchain/client-go"
   )
   
   func main() {
     // Initialize client
     client := provchain.NewClient(
       "pk_test_1234567890abcdef",
       "https://api.provchain-org.com",
     )
     
     // Get blockchain status
     status, err := client.Blockchain.GetStatus()
     if err != nil {
       panic(err)
     }
     fmt.Printf("Current height: %d\n", status.Height)
     
     // Submit transaction
     transaction, err := client.Transactions.Submit(map[string]interface{}{
       "type": "supply_chain_data",
       "data": map[string]interface{}{
         "@context": "https://schema.org",
         "@type":   "ProductBatch",
         "product": "Organic Tomatoes",
         "origin":  "Green Valley Farm",
       },
     })
     if err != nil {
       panic(err)
     }
     fmt.Printf("Transaction ID: %s\n", transaction.ID)
   }

Rate Limiting and Quotas
------------------------

**Rate Limits**
.. code-block:: http
   HTTP/1.1 200 OK
   X-RateLimit-Limit: 1000
   X-RateLimit-Remaining: 999
   X-RateLimit-Reset: 1642248600
   X-RateLimit-Reset-Time: "2024-01-15T10:30:00Z"
   
   {
     "data": {...}
   }

**Rate Limit Headers**
- ``X-RateLimit-Limit``: Maximum number of requests allowed in the time window
- ``X-RateLimit-Remaining``: Number of requests remaining in the current window
- ``X-RateLimit-Reset``: Unix timestamp when the rate limit resets
- ``X-RateLimit-Reset-Time``: Human-readable timestamp when the rate limit resets

**Rate Limit Response**
.. code-block:: http
   HTTP/1.1 429 Too Many Requests
   Content-Type: application/json
   X-RateLimit-Limit: 1000
   X-RateLimit-Remaining: 0
   X-RateLimit-Reset: 1642248600
   Retry-After: 3600
   
   {
     "error": {
       "code": "RATE_LIMIT_EXCEEDED",
       "message": "Rate limit exceeded. Try again later.",
       "retry_after": 3600
     }
   }

**Quota Management**
.. code-block:: http
   GET /api/v1/account/quotas
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "quotas": {
       "requests_per_minute": 1000,
       "requests_per_hour": 10000,
       "requests_per_day": 100000,
       "data_storage_mb": 1000,
       "api_calls_per_month": 1000000
     },
     "usage": {
       "requests_per_minute": 150,
       "requests_per_hour": 1200,
       "requests_per_day": 8500,
       "data_storage_mb": 450,
       "api_calls_this_month": 45000
     }
   }

Error Handling
--------------

**Error Response Format**
.. code-block:: json
   {
     "error": {
       "code": "INVALID_REQUEST",
       "message": "The request is invalid",
       "details": {
         "field": "data.product",
         "issue": "Product name is required"
       },
       "request_id": "req_1234567890"
     }
   }

**Common Error Codes**
- ``INVALID_REQUEST``: The request format is invalid
- ``AUTHENTICATION_FAILED``: Authentication failed
- ``INSUFFICIENT_PERMISSIONS``: User lacks required permissions
- ``RESOURCE_NOT_FOUND``: The requested resource doesn't exist
- ``RATE_LIMIT_EXCEEDED``: Rate limit exceeded
- ``VALIDATION_ERROR``: Data validation failed
- ``INTERNAL_ERROR``: Internal server error
- ``SERVICE_UNAVAILABLE``: Service is temporarily unavailable

**Error Handling Example**
.. code-block:: javascript
   async function callAPI() {
     try {
       const response = await fetch('https://api.provchain-org.com/api/v1/blocks', {
         headers: {
           'Authorization': `Bearer ${token}`,
           'Content-Type': 'application/json'
         }
       });
       
       if (!response.ok) {
         const error = await response.json();
         throw new Error(`API Error: ${error.error.message}`);
       }
       
       const data = await response.json();
       return data;
     } catch (error) {
       console.error('API call failed:', error);
       // Handle error appropriately
     }
   }

**Retry Logic**
.. code-block:: javascript
   async function retryAPIRequest(requestFn, maxRetries = 3) {
     let lastError;
     
     for (let i = 0; i < maxRetries; i++) {
       try {
         const response = await requestFn();
         if (response.status >= 200 && response.status < 300) {
           return response;
         }
         
         // Don't retry client errors
         if (response.status >= 400 && response.status < 500) {
           throw new Error(`Client error: ${response.status}`);
         }
         
         lastError = new Error(`Server error: ${response.status}`);
         await new Promise(resolve => setTimeout(resolve, 1000 * Math.pow(2, i)));
       } catch (error) {
         lastError = error;
         if (i === maxRetries - 1) {
           throw lastError;
         }
       }
     }
     
     throw lastError;
   }

Webhooks Integration
-------------------

**Webhook Event Types**
- ``new_block``: New block added to blockchain
- ``transaction_pending``: Transaction received and pending
- ``transaction_confirmed``: Transaction confirmed in block
- ``transaction_failed``: Transaction failed
- ``supply_chain_event``: Supply chain event occurred
- ``ontology_update``: Ontology schema updated
- ``system_alert``: System alert or notification

**Webhook Security**
.. code-block:: python
   import hmac
   import hashlib
   import json
   
   def verify_webhook_signature(payload, signature, secret):
     # Parse payload
     data = json.loads(payload)
     
     # Create signature
     hmac_obj = hmac.new(
       secret.encode('utf-8'),
       payload.encode('utf-8'),
       hashlib.sha256
     )
     expected_signature = hmac_obj.hexdigest()
     
     # Compare signatures
     return hmac.compare_digest(signature, expected_signature)

**Webhook Processing**
.. code-block:: python
   from flask import Flask, request, jsonify
   
   app = Flask(__name__)
   
   @app.route('/webhooks/provchain', methods=['POST'])
   def handle_webhook():
     # Verify signature
     signature = request.headers.get('X-Signature')
     if not verify_webhook_signature(
       request.data, 
       signature, 
       app.config['WEBHOOK_SECRET']
     ):
       return jsonify({'error': 'Invalid signature'}), 401
     
     # Process webhook
     event = request.json
     
     if event['event'] == 'new_block':
       process_new_block(event['data'])
     elif event['event'] == 'transaction_confirmed':
       process_transaction(event['data'])
     elif event['event'] == 'supply_chain_event':
       process_supply_chain_event(event['data'])
     
     return jsonify({'status': 'received'}), 200

Monitoring and Analytics
------------------------

**API Metrics**
.. code-block:: http
   GET /api/v1/metrics
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "metrics": {
       "requests_total": 123456,
       "requests_per_second": 15.5,
       "error_rate": 0.02,
       "average_response_time": 150,
       "p95_response_time": 350,
       "p99_response_time": 800
     }
   }

**API Usage Analytics**
.. code-block:: http
   GET /api/v1/analytics/usage
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN
   Content-Type: application/json
   
   {
     "start_date": "2024-01-01",
     "end_date": "2024-01-31",
     "granularity": "day"
   }

**Response**
.. code-block:: json
   {
     "usage": [
       {
         "date": "2024-01-01",
         "requests": 1234,
         "errors": 12,
         "data_transferred_mb": 45.6
       },
       {
         "date": "2024-01-02",
         "requests": 1456,
         "errors": 15,
         "data_transferred_mb": 52.3
       }
     ]
   }

**Performance Monitoring**
.. code-block:: http
   GET /api/v1/metrics/performance
   Host: api.provchain-org.com
   Authorization: Bearer YOUR_JWT_TOKEN

**Response**
.. code-block:: json
   {
     "performance": {
       "endpoints": {
         "/api/v1/blocks": {
           "avg_response_time": 120,
           "p95_response_time": 250,
           "p99_response_time": 500
