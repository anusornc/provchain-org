WebSocket API
============

The ProvChainOrg WebSocket API provides real-time communication for blockchain events, peer discovery, and live data updates. This API enables building responsive applications that can react immediately to blockchain changes.

Overview
--------

The WebSocket API supports:

- **Real-time Block Updates** - Instant notifications for new blocks
- **Peer Discovery** - Dynamic peer connection management
- **Live Query Results** - Streaming query results as they change
- **Event Subscriptions** - Subscribe to specific blockchain events
- **Bidirectional Communication** - Send commands and receive responses

Connection
----------

**WebSocket URL:** ``ws://localhost:8080/ws``

**Authentication:**
Connect with API key query parameter:
```
ws://localhost:8080/ws?api_key=YOUR_API_KEY
```

**Connection Example:**
```javascript
const ws = new WebSocket('ws://localhost:8080/ws?api_key=YOUR_API_KEY');

ws.onopen = () => {
    console.log('Connected to ProvChainOrg WebSocket');
};

ws.onmessage = (event) => {
    const message = JSON.parse(event.data);
    console.log('Received message:', message);
};

ws.onclose = () => {
    console.log('WebSocket connection closed');
};

ws.onerror = (error) => {
    console.error('WebSocket error:', error);
};
```

Message Format
--------------

All WebSocket messages follow this JSON structure:

```json
{
  "type": "message_type",
  "id": "unique_message_id",
  "timestamp": "2025-01-14T18:30:00Z",
  "data": {},
  "error": null
}
```

Message Types
-------------

Block Events
~~~~~~~~~~~~

**New Block Notification**
Sent when a new block is added to the blockchain.

```json
{
  "type": "new_block",
  "data": {
    "index": 43,
    "timestamp": "2025-01-14T18:30:15Z",
    "hash": "0x8f3e2d1c9b8a7654...",
    "previous_hash": "0x4a7b2c8f9e1d3a5b...",
    "triple_count": 15,
    "graph_name": "http://provchain.org/block/43"
  }
}
```

**Block Validation Result**
Sent when a block validation completes.

```json
{
  "type": "block_validation",
  "data": {
    "block_index": 43,
    "is_valid": true,
    "validation_time_ms": 245,
    "issues": []
  }
}
```

Network Events
~~~~~~~~~~~~~~

**Peer Connected**
Notification when a new peer connects.

```json
{
  "type": "peer_connected",
  "data": {
    "peer_id": "peer-001",
    "address": "192.168.1.100:8080",
    "connection_time": "2025-01-14T18:28:00Z",
    "capabilities": ["query", "block_sync"]
  }
}
```

**Peer Disconnected**
Notification when a peer disconnects.

```json
{
  "type": "peer_disconnected",
  "data": {
    "peer_id": "peer-001",
    "address": "192.168.1.100:8080",
    "disconnection_time": "2025-01-14T18:35:00Z",
    "reason": "normal_closure"
  }
}
```

Query Events
~~~~~~~~~~~~

**Query Started**
Notification when a query execution begins.

```json
{
  "type": "query_started",
  "data": {
    "query_id": "query_123456789",
    "sparql_query": "SELECT ?batch ?product WHERE { ?batch a :ProductBatch ; :product ?product }",
    "started_at": "2025-01-14T18:30:00Z"
  }
}
```

**Query Progress**
Streaming updates for long-running queries.

```json
{
  "type": "query_progress",
  "data": {
    "query_id": "query_123456789",
    "progress": 45,
    "current_result_count": 150,
    "estimated_completion": "2025-01-14T18:30:15Z"
  }
}
```

**Query Completed**
Final result when query execution completes.

```json
{
  "type": "query_completed",
  "data": {
    "query_id": "query_123456789",
    "result_count": 324,
    "execution_time_ms": 1250,
    "results": [
      {
        "batch": "http://example.org/supply-chain#Batch001",
        "product": "http://example.org/supply-chain#OrganicTomatoes"
      }
    ]
  }
}
```

Error Events
~~~~~~~~~~~~

**Connection Error**
WebSocket connection errors.

```json
{
  "type": "connection_error",
  "data": {
    "error_code": "AUTHENTICATION_FAILED",
    "message": "Invalid API key provided",
    "retry_after": 30
  }
}
```

**Validation Error**
Data validation errors.

```json
{
  "type": "validation_error",
  "data": {
    "error_code": "INVALID_RDF_DATA",
    "message": "Malformed Turtle syntax",
    "details": {
      "line": 5,
      "column": 12,
      "expected": "property URI",
      "found": "invalid_token"
    }
  }
}
```

Client Commands
---------------

Subscribe to Events
~~~~~~~~~~~~~~~~~~

Subscribe to specific event types.

**Command:**
```json
{
  "type": "subscribe",
  "command": {
    "events": ["new_block", "peer_connected", "query_completed"],
    "filters": {
      "block_index_min": 40
    }
  }
}
```

**Response:**
```json
{
  "type": "subscription_ack",
  "data": {
    "subscription_id": "sub_123456789",
    "events": ["new_block", "peer_connected", "query_completed"],
    "active": true
  }
}
```

Unsubscribe from Events
~~~~~~~~~~~~~~~~~~~~~~

Cancel event subscriptions.

**Command:**
```json
{
  "type": "unsubscribe",
  "command": {
    "subscription_id": "sub_123456789"
  }
}
```

Execute Query
~~~~~~~~~~~~~

Execute a SPARQL query with streaming results.

**Command:**
```json
{
  "type": "execute_query",
  "command": {
    "sparql_query": "SELECT ?batch ?product WHERE { ?batch a :ProductBatch ; :product ?product }",
    "stream_results": true,
    "batch_size": 100
  }
}
```

**Response:**
```json
{
  "type": "query_started",
  "data": {
    "query_id": "query_123456789",
    "sparql_query": "SELECT ?batch ?product WHERE { ?batch a :ProductBatch ; :product ?product }"
  }
}
```

**Streaming Results:**
```json
{
  "type": "query_results",
  "data": {
    "query_id": "query_123456789",
    "batch_number": 1,
    "results": [
      {
        "batch": "http://example.org/supply-chain#Batch001",
        "product": "http://example.org/supply-chain#OrganicTomatoes"
      }
    ]
  }
}
```

Add RDF Data
~~~~~~~~~~~~

Add new RDF data through WebSocket.

**Command:**
```json
{
  "type": "add_rdf_data",
  "command": {
    "turtle_data": "@prefix : <http://example.org/supply-chain#> .\n:Batch001 a :ProductBatch ; :hasBatchID \"TEST-001\" .",
    "validate": true
  }
}
```

**Response:**
```json
{
  "type": "add_rdf_data_response",
  "data": {
    "block_index": 44,
    "hash": "0x1a2b3c4d5e6f7890...",
    "validation_passed": true
  }
}
```

Request Blockchain Status
~~~~~~~~~~~~~~~~~~~~~~~~~

Get current blockchain status.

**Command:**
```json
{
  "type": "get_status",
  "command": {}
}
```

**Response:**
```json
{
  "type": "status_response",
  "data": {
    "blockchain": {
      "current_height": 43,
      "latest_block_hash": "0x4a7b2c8f9e1d3a5b...",
      "total_transactions": 156
    },
    "rdf_store": {
      "total_triples": 1247,
      "named_graphs": 43
    }
  }
}
```

Peer Management
~~~~~~~~~~~~~~

Get connected peers.

**Command:**
```json
{
  "type": "get_peers",
  "command": {}
}
```

**Response:**
```json
{
  "type": "peers_response",
  "data": {
    "connected_peers": [
      {
        "id": "peer-001",
        "address": "192.168.1.100:8080",
        "last_seen": "2025-01-14T18:28:00Z",
        "capabilities": ["query", "block_sync"]
      }
    ]
  }
}
```

Connection Management
---------------------

Reconnection Strategy
~~~~~~~~~~~~~~~~~~~~~

Implement automatic reconnection with exponential backoff.

```javascript
class ProvChainWebSocket {
    constructor(url, apiKey) {
        this.url = url;
        this.apiKey = apiKey;
        this.ws = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 1000;
        this.subscriptions = new Set();
        this.messageHandlers = new Map();
    }

    connect() {
        const wsUrl = `${this.url}?api_key=${this.apiKey}`;
        this.ws = new WebSocket(wsUrl);

        this.ws.onopen = () => {
            console.log('WebSocket connected');
            this.reconnectAttempts = 0;
            this.resubscribe();
        };

        this.ws.onmessage = (event) => {
            const message = JSON.parse(event.data);
            this.handleMessage(message);
        };

        this.ws.onclose = () => {
            console.log('WebSocket disconnected');
            this.scheduleReconnect();
        };

        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
        };
    }

    scheduleReconnect() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts);
            console.log(`Reconnecting in ${delay}ms...`);
            
            setTimeout(() => {
                this.reconnectAttempts++;
                this.connect();
            }, delay);
        }
    }

    resubscribe() {
        this.subscriptions.forEach(subscription => {
            this.send({
                type: 'subscribe',
                command: subscription
            });
        });
    }

    handleMessage(message) {
        const handler = this.messageHandlers.get(message.type);
        if (handler) {
            handler(message);
        }
    }

    send(message) {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(message));
        }
    }

    subscribe(events, filters = {}) {
        const subscription = { events, filters };
        this.subscriptions.add(subscription);
        
        this.send({
            type: 'subscribe',
            command: subscription
        });
    }

    unsubscribe(subscriptionId) {
        this.subscriptions = new Set([...this.subscriptions].filter(
            sub => sub.id !== subscriptionId
        ));
        
        this.send({
            type: 'unsubscribe',
            command: { subscription_id: subscriptionId }
        });
    }

    onMessage(messageType, handler) {
        this.messageHandlers.set(messageType, handler);
    }
}

// Usage
const wsClient = new ProvChainWebSocket('ws://localhost:8080/ws', 'YOUR_API_KEY');

wsClient.connect();

// Subscribe to events
wsClient.subscribe(['new_block', 'query_completed']);

// Handle specific message types
wsClient.onMessage('new_block', (message) => {
    console.log('New block:', message.data);
});

wsClient.onMessage('query_completed', (message) => {
    console.log('Query completed with', message.data.result_count, 'results');
});
```

Message Ordering
~~~~~~~~~~~~~~~~

Ensure proper message ordering with sequence numbers.

```javascript
class OrderedWebSocketClient {
    constructor(url, apiKey) {
        this.url = url;
        this.apiKey = apiKey;
        this.ws = null;
        this.expectedSequence = 1;
        this.messageQueue = new Map();
    }

    connect() {
        const wsUrl = `${this.url}?api_key=${this.apiKey}`;
        this.ws = new WebSocket(wsUrl);

        this.ws.onmessage = (event) => {
            const message = JSON.parse(event.data);
            if (message.sequence !== undefined) {
                this.handleSequencedMessage(message);
            } else {
                this.handleMessage(message);
            }
        };
    }

    handleSequencedMessage(message) {
        if (message.sequence === this.expectedSequence) {
            this.handleMessage(message);
            this.expectedSequence++;
            this.processQueue();
        } else {
            this.messageQueue.set(message.sequence, message);
        }
    }

    processQueue() {
        while (this.messageQueue.has(this.expectedSequence)) {
            const message = this.messageQueue.get(this.expectedSequence);
            this.handleMessage(message);
            this.messageQueue.delete(this.expectedSequence);
            this.expectedSequence++;
        }
    }

    handleMessage(message) {
        // Process message in correct order
        console.log('Processing message:', message.type, 'at sequence', message.sequence);
    }
}
```

Error Handling
--------------

Connection Errors
~~~~~~~~~~~~~~~~~

Handle various connection error scenarios.

```javascript
class RobustWebSocketClient {
    constructor(url, apiKey) {
        this.url = url;
        this.apiKey = apiKey;
        this.ws = null;
        this.connectionState = 'disconnected';
        this.retryCount = 0;
        this.maxRetries = 5;
    }

    connect() {
        try {
            const wsUrl = `${this.url}?api_key=${this.apiKey}`;
            this.ws = new WebSocket(wsUrl);
            this.connectionState = 'connecting';

            this.ws.onopen = () => {
                this.connectionState = 'connected';
                this.retryCount = 0;
                console.log('WebSocket connected');
            };

            this.ws.onclose = (event) => {
                this.connectionState = 'disconnected';
                this.handleDisconnection(event);
            };

            this.ws.onerror = (error) => {
                this.connectionState = 'error';
                this.handleError(error);
            };

        } catch (error) {
            this.handleConnectionError(error);
        }
    }

    handleDisconnection(event) {
        console.log('WebSocket disconnected:', event.code, event.reason);

        if (event.code === 1008) { // Policy violation
            console.error('Authentication failed');
            this.handleAuthenticationError();
        } else if (event.code === 1006) { // Abnormal closure
            console.warn('Abnormal closure, attempting reconnect');
            this.scheduleReconnect();
        } else {
            this.scheduleReconnect();
        }
    }

    handleError(error) {
        console.error('WebSocket error:', error);
        
        if (this.retryCount < this.maxRetries) {
            this.scheduleReconnect();
        } else {
            console.error('Max retry attempts reached');
            this.handlePermanentFailure();
        }
    }

    handleConnectionError(error) {
        console.error('Connection error:', error);
        this.scheduleReconnect();
    }

    scheduleReconnect() {
        const delay = Math.min(1000 * Math.pow(2, this.retryCount), 30000);
        console.log(`Scheduling reconnect in ${delay}ms...`);
        
        setTimeout(() => {
            this.retryCount++;
            this.connect();
        }, delay);
    }

    handleAuthenticationError() {
        console.error('Authentication failed. Please check your API key.');
        // Notify user or refresh authentication
    }

    handlePermanentFailure() {
        console.error('Permanent connection failure');
        // Implement fallback or notify user
    }
}
```

Performance Optimization
------------------------

Message Batching
~~~~~~~~~~~~~~~~~

Batch multiple messages for better performance.

```javascript
class BatchedWebSocketClient {
    constructor(url, apiKey) {
        this.url = url;
        this.apiKey = apiKey;
        this.ws = null;
        this.messageQueue = [];
        this.batchTimer = null;
        this.batchSize = 10;
        this.batchInterval = 100; // ms
    }

    connect() {
        const wsUrl = `${this.url}?api_key=${this.apiKey}`;
        this.ws = new WebSocket(wsUrl);

        this.ws.onopen = () => {
            console.log('WebSocket connected');
            this.startBatching();
        };

        this.ws.onmessage = (event) => {
            this.handleMessage(JSON.parse(event.data));
        };
    }

    startBatching() {
        this.batchTimer = setInterval(() => {
            if (this.messageQueue.length > 0) {
                this.flushBatch();
            }
        }, this.batchInterval);
    }

    send(message) {
        this.messageQueue.push(message);
        
        if (this.messageQueue.length >= this.batchSize) {
            this.flushBatch();
        }
    }

    flushBatch() {
        if (this.messageQueue.length === 0) return;

        const batch = this.messageQueue.splice(0, this.batchSize);
        const batchMessage = {
            type: 'batch',
            messages: batch
        };

        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(batchMessage));
        }
    }

    handleMessage(message) {
        if (message.type === 'batch_response') {
            message.messages.forEach(msg => this.processMessage(msg));
        } else {
            this.processMessage(message);
        }
    }

    processMessage(message) {
        // Process individual message
        console.log('Processed message:', message.type);
    }
}
```

Compression
~~~~~~~~~~~

Implement message compression for large data transfers.

```javascript
class CompressedWebSocketClient {
    constructor(url, apiKey) {
        this.url = url;
        this.apiKey = apiKey;
        this.ws = null;
        this.compressionEnabled = true;
    }

    connect() {
        const wsUrl = `${this.url}?api_key=${this.apiKey}`;
        this.ws = new WebSocket(wsUrl);

        this.ws.onopen = () => {
            console.log('WebSocket connected');
        };

        this.ws.onmessage = (event) => {
            const message = this.decompressMessage(event.data);
            this.handleMessage(message);
        };
    }

    send(message) {
        const messageString = JSON.stringify(message);
        const compressedData = this.compressMessage(messageString);
        
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(compressedData);
        }
    }

    compressMessage(data) {
        if (!this.compressionEnabled) {
            return data;
        }
        
        // Implement compression (e.g., using pako or similar library)
        // For demonstration, we'll just return the original data
        return data;
    }

    decompressMessage(data) {
        if (!this.compressionEnabled) {
            return JSON.parse(data);
        }
        
        // Implement decompression
        return JSON.parse(data);
    }

    handleMessage(message) {
        console.log('Received compressed message:', message.type);
    }
}
```

Security Considerations
----------------------

Authentication
~~~~~~~~~~~~~~

Implement secure authentication mechanisms.

```javascript
class SecureWebSocketClient {
    constructor(url, apiKey) {
        this.url = url;
        this.apiKey = apiKey;
        this.ws = null;
        this.authToken = null;
        this.refreshToken = null;
    }

    async connect() {
        try {
            // First, authenticate to get a WebSocket token
            const authResponse = await this.authenticate();
            this.authToken = authResponse.token;
            this.refreshToken = authResponse.refresh_token;

            // Connect with the token
            const wsUrl = `${this.url}?token=${this.authToken}`;
            this.ws = new WebSocket(wsUrl);

            this.ws.onopen = () => {
                console.log('Secure WebSocket connected');
            };

            this.ws.onclose = () => {
                this.handleDisconnection();
            };

        } catch (error) {
            console.error('Authentication failed:', error);
            this.handleAuthenticationError(error);
        }
    }

    async authenticate() {
        const response = await fetch(`${this.url.replace('/ws', '/auth')}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${this.apiKey}`
            },
            body: JSON.stringify({
                grant_type: 'client_credentials',
                scope: 'websocket'
            })
        });

        if (!response.ok) {
            throw new Error('Authentication failed');
        }

        return response.json();
    }

    async refreshToken() {
        try {
            const response = await fetch(`${this.url.replace('/ws', '/auth')}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${this.refreshToken}`
                },
                body: JSON.stringify({
                    grant_type: 'refresh_token'
                })
            });

            if (response.ok) {
                const authData = await response.json();
                this.authToken = authData.token;
                return true;
            }
        } catch (error) {
            console.error('Token refresh failed:', error);
        }
        return false;
    }

    handleDisconnection() {
        // Attempt to reconnect with refreshed token
        this.connect();
    }

    handleAuthenticationError(error) {
        console.error('Authentication error:', error);
        // Implement proper error handling and user notification
    }
}
```

Message Encryption
~~~~~~~~~~~~~~~~~~

Implement end-to-end encryption for sensitive messages.

```javascript
class EncryptedWebSocketClient {
    constructor(url, apiKey) {
        this.url = url;
        this.apiKey = apiKey;
        this.ws = null;
        this.encryptionKey = null;
        this.keyExchangeComplete = false;
    }

    async connect() {
        const wsUrl = `${this.url}?api_key=${this.apiKey}`;
        this.ws = new WebSocket(wsUrl);

        this.ws.onopen = () => {
            console.log('WebSocket connected, initiating key exchange');
            this.initiateKeyExchange();
        };

        this.ws.onmessage = (event) => {
            this.handleEncryptedMessage(event.data);
        };
    }

    async initiateKeyExchange() {
        // Generate ephemeral key pair
        const ephemeralKey = this.generateEphemeralKey();
        
        // Send key exchange request
        const keyExchangeMessage = {
            type: 'key_exchange',
            public_key: ephemeralKey.publicKey
        };

        this.send(keyExchangeMessage);

        // Wait for server response
        // In a real implementation, this would be more sophisticated
        this.encryptionKey = await this.deriveSharedKey(ephemeralKey);
        this.keyExchangeComplete = true;
    }

    generateEphemeralKey() {
        // Implement key generation (e.g., using Web Crypto API)
        return {
            publicKey: 'generated_public_key',
            privateKey: 'generated_private_key'
        };
    }

    async deriveSharedKey(ephemeralKey) {
        // Implement key derivation
        return 'derived_shared_key';
    }

    encryptMessage(message) {
        if (!this.keyExchangeComplete) {
            return message;
        }

        // Implement message encryption
        // For demonstration, return original message
        return message;
    }

    decryptMessage(encryptedData) {
        if (!this.keyExchangeComplete) {
            return JSON.parse(encryptedData);
        }

        // Implement message decryption
        return JSON.parse(encryptedData);
    }

    send(message) {
        const encryptedMessage = this.encryptMessage(JSON.stringify(message));
        
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(encryptedMessage);
        }
    }

    handleEncryptedMessage(encryptedData) {
        const message = this.decryptMessage(encryptedData);
        this.handleMessage(message);
    }

    handleMessage(message) {
        console.log('Received encrypted message:', message.type);
    }
}
```

Example Applications
-------------------

Real-time Dashboard
~~~~~~~~~~~~~~~~~~~

```javascript
class ProvChainDashboard {
    constructor() {
        this.ws = new WebSocket('ws://localhost:8080/ws?api_key=YOUR_API_KEY');
        this.dashboardElement = document.getElementById('dashboard');
        this.setupEventHandlers();
    }

    setupEventHandlers() {
        this.ws.onopen = () => {
            console.log('Dashboard connected');
            this.subscribeToEvents();
        };

        this.ws.onmessage = (event) => {
            const message = JSON.parse(event.data);
            this.handleMessage(message);
        };
    }

    subscribeToEvents() {
        this.ws.send(JSON.stringify({
            type: 'subscribe',
            command: {
                events: ['new_block', 'query_completed', 'peer_connected']
            }
        }));
    }

    handleMessage(message) {
        switch (message.type) {
            case 'new_block':
                this.updateBlockInfo(message.data);
                break;
            case 'query_completed':
                this.updateQueryResults(message.data);
                break;
            case 'peer_connected':
                this.updatePeerList(message.data);
                break;
        }
    }

    updateBlockInfo(blockData) {
        const blockElement = document.getElementById('latest-block');
        blockElement.innerHTML = `
            <div>Block #${blockData.index}</div>
            <div>Hash: ${blockData.hash.substring(0, 16)}...</div>
            <div>Triples: ${blockData.triple_count}</div>
            <div>Time: ${new Date(blockData.timestamp).toLocaleTimeString()}</div>
        `;
    }

    updateQueryResults(queryData) {
        const resultsElement = document.getElementById('query-results');
        resultsElement.innerHTML = `
            <div>Query completed with ${queryData.result_count} results</div>
            <div>Execution time: ${queryData.execution_time_ms}ms</div>
        `;
    }

    updatePeerList(peerData) {
        const peerElement = document.getElementById('peer-count');
        peerElement.textContent = peerData.connected_peers.length;
    }
}

// Initialize dashboard
const dashboard = new ProvChainDashboard();
```

Live Query Monitor
~~~~~~~~~~~~~~~~~~

```javascript
class QueryMonitor {
    constructor() {
        this.ws = new WebSocket('ws://localhost:8080/ws?api_key=YOUR_API_KEY');
        this.activeQueries = new Map();
        this.setupEventHandlers();
    }

    setupEventHandlers() {
        this.ws.onopen = () => {
            console.log('Query monitor connected');
        };

        this.ws.onmessage = (event) => {
            const message = JSON.parse(event.data);
            this.handleMessage(message);
        };
    }

    executeQuery(sparqlQuery) {
        const queryId = `query_${Date.now()}`;
        
        this.activeQueries.set(queryId, {
            query: sparqlQuery,
            startTime: Date.now(),
            progress: 0,
            resultCount: 0
        });

        this.ws.send(JSON.stringify({
            type: 'execute_query',
            command: {
                query_id: queryId,
                sparql_query: sparqlQuery,
                stream_results: true
            }
        }));

        return queryId;
    }

    handleMessage(message) {
        switch (message.type) {
            case 'query_started':
                this.handleQueryStarted(message.data);
                break;
            case 'query_progress':
                this.handleQueryProgress(message.data);
                break;
            case 'query_completed':
                this.handleQueryCompleted(message.data);
                break;
            case 'query_results':
                this.handleQueryResults(message.data);
                break;
        }
    }

    handleQueryStarted(data) {
        console.log(`Query ${data.query_id} started`);
        this.updateQueryDisplay(data.query_id, 'started');
    }

    handleQueryProgress(data) {
        const query = this.activeQueries.get(data.query_id);
        if (query) {
            query.progress = data.progress;
            query.resultCount = data.current_result_count;
            this.updateQueryDisplay(data.query_id, 'progress', query);
        }
    }

    handleQueryResults(data) {
        const query = this.activeQueries.get(data.query_id);
        if (query) {
            query.results = data.results;
            this.updateQueryDisplay(data.query_id, 'results', query);
        }
    }

    handleQueryCompleted(data) {
        const query = this.activeQueries.get(data.query_id);
        if (query) {
            query.endTime = Date.now();
            query.executionTime = query.endTime - query.startTime;
            this.activeQueries.delete(data.query_id);
            this.updateQueryDisplay(data.query_id, 'completed', query);
        }
    }

    updateQueryDisplay(queryId, status, queryData = null) {
        const element = document.getElementById(`query-${queryId}`);
        if (!element) {
            this.createQueryElement(queryId);
        }

        switch (status) {
            case 'started':
                this.updateElement(queryId, 'Query started...');
                break;
            case 'progress':
                this.updateElement(queryId, 
                    `Progress: ${queryData.progress}%, Results: ${queryData.resultCount}`);
                break;
            case 'results':
                this.updateElement(queryId, 
                    `Received ${queryData.results.length} results`);
                break;
            case 'completed':
                this.updateElement(queryId, 
                    `Completed in ${queryData.executionTime}ms with ${data.result_count} results`);
                break;
        }
    }

    createQueryElement(queryId) {
        const container = document.getElementById('query-monitor');
        const element = document.createElement('div');
        element.id = `query-${queryId}`;
        element.className = 'query-item';
        container.appendChild(element);
    }

    updateElement(queryId, text) {
        const element = document.getElementById(`query-${queryId}`);
        if (element) {
            element.textContent = text;
        }
    }
}

// Usage
const monitor = new QueryMonitor();

// Execute a query
const queryId = monitor.executeQuery(`
    SELECT ?batch ?product ?farm WHERE {
        ?batch a :ProductBatch ;
               :product ?product ;
               :originFarm ?farm .
    }
`);
```

Troubleshooting
---------------

Common Issues
~~~~~~~~~~~~~

**Connection Refused**
   - Verify ProvChainOrg server is running
   - Check WebSocket URL and port
   - Ensure firewall allows WebSocket connections

**Authentication Failed**
   - Verify API key is correct and valid
   - Check API key permissions for WebSocket access
   - Ensure authentication token is properly formatted

**Message Loss**
   - Implement message acknowledgment system
   - Add sequence numbers for message ordering
   - Implement proper error handling and retransmission

**Performance Issues**
   - Enable message compression for large data
   - Implement message batching
   - Use connection pooling for multiple clients

**Memory Leaks**
   - Clean up event listeners properly
   - Cancel subscriptions when disconnecting
   - Implement proper garbage collection for message queues

Debugging Techniques
~~~~~~~~~~~~~~~~~~~~

**Enable Debug Logging**
```javascript
const ws = new WebSocket('ws://localhost:8080/ws?api_key=YOUR_API_KEY&debug=true');

ws.onmessage = (event) => {
    console.log('Raw message:', event.data);
    const message = JSON.parse(event.data);
    console.log('Parsed message:', message);
};
```

**Monitor Connection State**
```javascript
class ConnectionMonitor {
    constructor(ws) {
        this.ws = ws;
        this.connectionStates = [];
        this.setupMonitoring();
    }

    setupMonitoring() {
        this.ws.addEventListener('open', () => {
            this.logConnectionState('connected');
        });

        this.ws.addEventListener('close', (event) => {
            this.logConnectionState('disconnected', event);
        });

        this.ws.addEventListener('error', (error) => {
            this.logConnectionState('error', error);
        });
    }

    logConnectionState(state, event = null) {
        const logEntry = {
            timestamp: new Date().toISOString(),
            state: state,
            event: event
        };
        
        this.connectionStates.push(logEntry);
        console.log('Connection state change:', logEntry);
    }

    getConnectionHistory() {
        return this.connectionStates;
    }
}
```

**Message Flow Analysis**
```javascript
class MessageAnalyzer {
    constructor() {
        this.messageStats = {
            total: 0,
            byType: {},
            errors: 0,
            averageLatency: 0
        };
        this.messageHistory = [];
    }

    analyzeMessage(message) {
        const analysis = {
            timestamp: Date.now(),
            type: message.type,
            size: JSON.stringify(message).length,
            hasError: !!message.error
        };

        this.messageStats.total++;
        this.messageStats.byType[message.type] = (this.messageStats.byType[message.type] || 0) + 1;
        
        if (message.error) {
            this.messageStats.errors++;
        }

        this.messageHistory.push(analysis);
        this.updateStatistics();

        return analysis;
    }

    updateStatistics() {
        if (this.messageHistory.length > 0) {
            const recentMessages = this.messageHistory.slice(-100);
            const totalLatency = recentMessages.reduce((sum, msg) => sum + (Date.now() - msg.timestamp), 0);
            this.messageStats.averageLatency = totalLatency / recentMessages.length;
        }
    }

    getStatistics() {
        return { ...this.messageStats };
    }
}
```

Best Practices
--------------

1. **Implement Proper Error Handling**: Handle all WebSocket error scenarios gracefully.

2. **Use Connection Management**: Implement automatic reconnection with backoff strategies.

3. **Message Ordering**: Ensure messages are processed in the correct order.

4. **Resource Management**: Clean up resources properly when disconnecting.

5. **Security**: Use secure authentication and encryption for sensitive data.

6. **Performance**: Optimize message handling with batching and compression.

7. **Monitoring**: Implement logging and monitoring for WebSocket connections.

8. **Testing**: Test WebSocket connections under various network conditions.

9. **Documentation**: Document WebSocket API usage and message formats.

10. **Versioning**: Implement versioning for WebSocket messages to handle API changes.

Related Resources
-----------------

- **WebSocket API Specification**: RFC 6455
- **SPARQL 1.1 Protocol**: W3C Recommendation
- **Real-time Web Applications**: Best practices and patterns
- **WebSocket Security**: Authentication and encryption guidelines
- **Performance Optimization**: WebSocket performance tuning
