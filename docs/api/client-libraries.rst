Client Libraries
===============

Official and community-maintained client libraries for integrating with ProvChainOrg APIs in multiple programming languages.

.. raw:: html

   <div class="hero-section">
     <div class="hero-content">
       <h1>Client Libraries</h1>
       <p class="hero-subtitle">Ready-to-use libraries for integrating ProvChainOrg into your applications</p>
       <div class="hero-badges">
         <span class="badge badge-api">API</span>
         <span class="badge badge-integration">Integration</span>
         <span class="badge badge-sdks">SDKs</span>
         <span class="badge badge-languages">Languages</span>
       </div>
     </div>
   </div>

Overview
--------

ProvChainOrg provides official client libraries for the most popular programming languages, along with comprehensive documentation and examples. These libraries handle authentication, request signing, error handling, and other common tasks to make integration as simple as possible.

**Supported Languages:**
- **Python**: Official Python SDK with full feature support
- **JavaScript/TypeScript**: Official Node.js and browser libraries
- **Rust**: Official Rust crate with native performance
- **Java**: Official Java SDK for enterprise applications
- **Go**: Official Go module for cloud-native applications
- **C#**: Official .NET library for Windows and cross-platform applications

**Community Libraries:**
- **PHP**: Community-maintained library
- **Ruby**: Community-maintained library
- **Swift**: Community-maintained iOS library
- **Kotlin**: Community-maintained Android library

Installation
------------

Python SDK
~~~~~~~~~~

Install the official Python SDK:

.. code-block:: bash

   pip install provchain-sdk

.. code-block:: python

   from provchain import ProvChainClient
   
   # Initialize client
   client = ProvChainClient(
       base_url="https://api.provchain-org.com",
       api_key="YOUR_API_KEY"
   )
   
   # Get blockchain status
   status = client.get_status()
   print(f"Current block height: {status['blockchain']['current_height']}")
   
   # Add RDF data
   rdf_data = """
   @prefix : <http://example.org/supply-chain#> .
   :Batch001 a :ProductBatch ;
       :hasBatchID "TEST-001" ;
       :product :OrganicTomatoes .
   """
   
   result = client.add_rdf_data(rdf_data)
   print(f"Added block {result['block_index']}")

JavaScript/TypeScript SDK
~~~~~~~~~~~~~~~~~~~~~~~~~

Install the official JavaScript SDK:

.. code-block:: bash

   npm install @provchain/sdk

.. code-block:: javascript

   import { ProvChainClient } from '@provchain/sdk';
   
   // Initialize client
   const client = new ProvChainClient({
       baseUrl: 'https://api.provchain-org.com',
       apiKey: 'YOUR_API_KEY'
   });
   
   // Get blockchain status
   client.getStatus().then(status => {
       console.log(`Current block height: ${status.blockchain.current_height}`);
   });
   
   // Add RDF data
   const rdfData = `
   @prefix : <http://example.org/supply-chain#> .
   :Batch001 a :ProductBatch ;
       :hasBatchID "TEST-001" ;
       :product :OrganicTomatoes .
   `;
   
   client.addRdfData(rdfData).then(result => {
       console.log(`Added block ${result.block_index}`);
   });

Rust Crate
~~~~~~~~~~

Add to your Cargo.toml:

.. code-block:: toml

   [dependencies]
   provchain-sdk = "0.1.0"

.. code-block:: rust

   use provchain_sdk::{ProvChainClient, Config};
   
   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       // Initialize client
       let config = Config::new("https://api.provchain-org.com")
           .with_api_key("YOUR_API_KEY");
       let client = ProvChainClient::new(config);
       
       // Get blockchain status
       let status = client.get_status().await?;
       println!("Current block height: {}", status.blockchain.current_height);
       
       // Add RDF data
       let rdf_data = r#"
       @prefix : <http://example.org/supply-chain#> .
       :Batch001 a :ProductBatch ;
           :hasBatchID "TEST-001" ;
           :product :OrganicTomatoes .
       "#;
       
       let result = client.add_rdf_data(rdf_data).await?;
       println!("Added block {}", result.block_index);
       
       Ok(())
   }

Java SDK
~~~~~~~~

Add to your pom.xml:

.. code-block:: xml

   <dependency>
       <groupId>org.provchain</groupId>
       <artifactId>provchain-sdk</artifactId>
       <version>0.1.0</version>
   </dependency>

.. code-block:: java

   import org.provchain.sdk.ProvChainClient;
   import org.provchain.sdk.Config;
   import org.provchain.sdk.models.StatusResponse;
   
   public class Example {
       public static void main(String[] args) {
           // Initialize client
           Config config = new Config("https://api.provchain-org.com")
               .withApiKey("YOUR_API_KEY");
           ProvChainClient client = new ProvChainClient(config);
           
           try {
               // Get blockchain status
               StatusResponse status = client.getStatus();
               System.out.println("Current block height: " + 
                   status.getBlockchain().getCurrentHeight());
               
               // Add RDF data
               String rdfData = """
               @prefix : <http://example.org/supply-chain#> .
               :Batch001 a :ProductBatch ;
                   :hasBatchID "TEST-001" ;
                   :product :OrganicTomatoes .
               """;
               
               var result = client.addRdfData(rdfData);
               System.out.println("Added block " + result.getBlockIndex());
               
           } catch (Exception e) {
               e.printStackTrace();
           }
       }
   }

Go Module
~~~~~~~~~

.. code-block:: bash

   go get github.com/provchain-org/provchain-sdk-go

.. code-block:: go

   package main
   
   import (
       "fmt"
       "log"
       "github.com/provchain-org/provchain-sdk-go/client"
       "github.com/provchain-org/provchain-sdk-go/config"
   )
   
   func main() {
       // Initialize client
       cfg := config.NewConfig("https://api.provchain-org.com")
       cfg.SetAPIKey("YOUR_API_KEY")
       client := client.NewClient(cfg)
       
       // Get blockchain status
       status, err := client.GetStatus()
       if err != nil {
           log.Fatal(err)
       }
       fmt.Printf("Current block height: %d\n", status.Blockchain.CurrentHeight)
       
       // Add RDF data
       rdfData := `
       @prefix : <http://example.org/supply-chain#> .
       :Batch001 a :ProductBatch ;
           :hasBatchID "TEST-001" ;
           :product :OrganicTomatoes .
       `
       
       result, err := client.AddRdfData(rdfData)
       if err != nil {
           log.Fatal(err)
       }
       fmt.Printf("Added block %d\n", result.BlockIndex)
   }

C# Library
~~~~~~~~~~

.. code-block:: bash

   dotnet add package ProvChain.SDK

.. code-block:: csharp

   using ProvChain.SDK;
   using ProvChain.SDK.Models;
   
   class Program
   {
       static async Task Main(string[] args)
       {
           // Initialize client
           var config = new Config
           {
               BaseUrl = "https://api.provchain-org.com",
               ApiKey = "YOUR_API_KEY"
           };
           var client = new ProvChainClient(config);
           
           try
           {
               // Get blockchain status
               var status = await client.GetStatusAsync();
               Console.WriteLine($"Current block height: {status.Blockchain.CurrentHeight}");
               
               // Add RDF data
               var rdfData = @"
               @prefix : <http://example.org/supply-chain#> .
               :Batch001 a :ProductBatch ;
                   :hasBatchID ""TEST-001"" ;
                   :product :OrganicTomatoes .
               ";
               
               var result = await client.AddRdfDataAsync(rdfData);
               Console.WriteLine($"Added block {result.BlockIndex}");
           }
           catch (Exception ex)
           {
               Console.WriteLine($"Error: {ex.Message}");
           }
       }
   }

Authentication
--------------

All client libraries support the same authentication methods as the REST API:

API Key Authentication
~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python
   # Python
   client = ProvChainClient(api_key="pk_1234567890abcdef")

.. code-block:: javascript
   // JavaScript
   const client = new ProvChainClient({ apiKey: 'pk_1234567890abcdef' });

.. code-block:: rust
   // Rust
   let config = Config::new("https://api.provchain-org.com")
       .with_api_key("pk_1234567890abcdef");

JWT Authentication
~~~~~~~~~~~~~~~~~~

.. code-block:: python
   # Python
   client = ProvChainClient(jwt_token="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")

.. code-block:: javascript
   // JavaScript
   const client = new ProvChainClient({ jwtToken: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...' });

Certificate Authentication
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python
   # Python
   client = ProvChainClient(
       cert_file="client.crt",
       key_file="client.key"
   )

.. code-block:: javascript
   // JavaScript (Node.js)
   const client = new ProvChainClient({
       cert: fs.readFileSync('client.crt'),
       key: fs.readFileSync('client.key')
   });

Core Features
-------------

All client libraries provide consistent interfaces for core ProvChainOrg functionality:

Blockchain Status
~~~~~~~~~~~~~~~~~

.. code-block:: python
   # Python
   status = client.get_status()
   print(f"Blockchain height: {status['blockchain']['current_height']}")
   print(f"Total transactions: {status['blockchain']['total_transactions']}")

.. code-block:: javascript
   // JavaScript
   const status = await client.getStatus();
   console.log(`Blockchain height: ${status.blockchain.currentHeight}`);
   console.log(`Total transactions: ${status.blockchain.totalTransactions}`);

Adding RDF Data
~~~~~~~~~~~~~~~

.. code-block:: python
   # Python
   rdf_data = """
   @prefix : <http://example.org/supply-chain#> .
   :Batch001 a :ProductBatch ;
       :hasBatchID "BATCH-001" ;
       :product :OrganicTomatoes ;
       :harvestDate "2025-01-15"^^xsd:date .
   """
   
   result = client.add_rdf_data(rdf_data)
   print(f"Added block {result['block_index']} with hash {result['block_hash']}")

SPARQL Queries
~~~~~~~~~~~~~~

.. code-block:: python
   # Python
   query = """
   PREFIX : <http://example.org/supply-chain#>
   SELECT ?batch ?product ?farm WHERE {
       ?batch a :ProductBatch ;
              :product ?product ;
              :originFarm ?farm .
   }
   """
   
   results = client.execute_sparql(query)
   for binding in results['results']['bindings']:
       print(f"Batch: {binding['batch']['value']}")
       print(f"Product: {binding['product']['value']}")
       print(f"Farm: {binding['farm']['value']}")

Block Operations
~~~~~~~~~~~~~~~~

.. code-block:: python
   # Python
   # Get specific block
   block = client.get_block(42)
   print(f"Block {block['index']} contains {block['triple_count']} triples")
   
   # Get blocks with pagination
   blocks = client.get_blocks(limit=10, offset=0)
   print(f"Retrieved {len(blocks['blocks'])} blocks")

Data Validation
~~~~~~~~~~~~~~~

.. code-block:: python
   # Python
   rdf_data = """
   @prefix : <http://example.org/supply-chain#> .
   :Batch001 a :ProductBatch ;
       :hasBatchID "BATCH-001" .
   """
   
   validation = client.validate_rdf(rdf_data)
   if validation['is_valid']:
       print("RDF data is valid")
   else:
       print(f"Validation errors: {validation['issues']}")

Advanced Features
----------------

Rate Limiting Handling
~~~~~~~~~~~~~~~~~~~~~~

All client libraries automatically handle rate limiting:

.. code-block:: python
   # Python - Automatic retry with exponential backoff
   client = ProvChainClient(
       api_key="YOUR_API_KEY",
       retry_strategy="exponential_backoff",
       max_retries=3
   )

.. code-block:: javascript
   // JavaScript - Promise-based retry
   const client = new ProvChainClient({
       apiKey: 'YOUR_API_KEY',
       retryStrategy: 'exponentialBackoff',
       maxRetries: 3
   });

Error Handling
~~~~~~~~~~~~~~

Comprehensive error handling with detailed error information:

.. code-block:: python
   # Python
   try:
       result = client.add_rdf_data(invalid_rdf)
   except provchain.exceptions.ValidationError as e:
       print(f"Validation error: {e.message}")
       print(f"Details: {e.details}")
   except provchain.exceptions.RateLimitError as e:
       print(f"Rate limit exceeded. Retry after: {e.retry_after} seconds")
   except provchain.exceptions.AuthenticationError as e:
       print(f"Authentication failed: {e.message}")

Streaming Responses
~~~~~~~~~~~~~~~~~~~

For large query results, client libraries support streaming:

.. code-block:: python
   # Python - Streaming query results
   def process_binding(binding):
       # Process each result as it arrives
       print(f"Processing batch: {binding['batch']['value']}")
   
   client.stream_sparql(query, process_binding)

Async Operations
~~~~~~~~~~~~~~~~

All client libraries support asynchronous operations where applicable:

.. code-block:: python
   # Python - Async operations
   import asyncio
   
   async def main():
       client = ProvChainClientAsync(api_key="YOUR_API_KEY")
       
       # Concurrent operations
       status_task = client.get_status_async()
       blocks_task = client.get_blocks_async(limit=5)
       
       status, blocks = await asyncio.gather(status_task, blocks_task)
       print(f"Status: {status}")
       print(f"Blocks: {len(blocks['blocks'])}")
   
   asyncio.run(main())

.. code-block:: javascript
   // JavaScript - Async/await
   async function main() {
       const client = new ProvChainClient({ apiKey: 'YOUR_API_KEY' });
       
       // Concurrent operations
       const [status, blocks] = await Promise.all([
           client.getStatus(),
           client.getBlocks({ limit: 5 })
       ]);
       
       console.log('Status:', status);
       console.log('Blocks:', blocks.blocks.length);
   }

Configuration Options
---------------------

Client libraries support extensive configuration options:

.. code-block:: python
   # Python - Advanced configuration
   client = ProvChainClient(
       base_url="https://api.provchain-org.com",
       api_key="YOUR_API_KEY",
       timeout=30,  # 30 second timeout
       max_retries=3,
       retry_delay=1.0,  # 1 second initial delay
       user_agent="MyApp/1.0",
       proxy="http://proxy.example.com:8080",
       ssl_verify=True,
       connection_pool_size=10
   )

.. code-block:: javascript
   // JavaScript - Advanced configuration
   const client = new ProvChainClient({
       baseUrl: 'https://api.provchain-org.com',
       apiKey: 'YOUR_API_KEY',
       timeout: 30000,  // 30 seconds
       maxRetries: 3,
       retryDelay: 1000,  // 1 second
       userAgent: 'MyApp/1.0',
       proxy: 'http://proxy.example.com:8080',
       sslVerify: true
   });

Logging and Debugging
---------------------

Client libraries provide comprehensive logging for debugging:

.. code-block:: python
   # Python - Enable logging
   import logging
   logging.basicConfig(level=logging.DEBUG)
   
   client = ProvChainClient(api_key="YOUR_API_KEY")
   # All requests and responses will be logged

.. code-block:: javascript
   // JavaScript - Enable debug mode
   const client = new ProvChainClient({
       apiKey: 'YOUR_API_KEY',
       debug: true  // Enable debug logging
   });

Performance Optimization
------------------------

Client libraries include performance optimizations:

Connection Pooling
~~~~~~~~~~~~~~~~~~

.. code-block:: python
   # Python - Connection pooling
   client = ProvChainClient(
       api_key="YOUR_API_KEY",
       connection_pool_size=20,  # Reuse connections
       keep_alive=True
   )

Caching
~~~~~~~

.. code-block:: python
   # Python - Response caching
   client = ProvChainClient(
       api_key="YOUR_API_KEY",
       cache_responses=True,
       cache_ttl=300  # 5 minutes
   )

Batch Operations
~~~~~~~~~~~~~~~~

.. code-block:: python
   # Python - Batch operations
   rdf_data_list = [
       "@prefix : <http://example.org/supply-chain#> . :Batch001 a :ProductBatch .",
       "@prefix : <http://example.org/supply-chain#> . :Batch002 a :ProductBatch .",
       "@prefix : <http://example.org/supply-chain#> . :Batch003 a :ProductBatch ."
   ]
   
   # Add multiple RDF datasets in a single batch
   results = client.add_rdf_data_batch(rdf_data_list)
   print(f"Added {len(results)} blocks")

Examples and Tutorials
----------------------

Supply Chain Tracking
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python
   # Python - Complete supply chain tracking example
   from provchain import ProvChainClient
   
   class SupplyChainTracker:
       def __init__(self, api_key):
           self.client = ProvChainClient(api_key=api_key)
       
       def add_product_batch(self, batch_id, product, farm, harvest_date):
           """Add a new product batch to the blockchain"""
           rdf_data = f"""
           @prefix : <http://example.org/supply-chain#> .
           @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
           
           :{batch_id} a :ProductBatch ;
               :hasBatchID "{batch_id}" ;
               :product :{product} ;
               :originFarm :{farm} ;
               :harvestDate "{harvest_date}"^^xsd:date .
           """
           
           result = self.client.add_rdf_data(rdf_data)
           return result['block_index']
       
       def trace_batch(self, batch_id):
           """Trace a product batch through the supply chain"""
           query = f"""
           PREFIX : <http://example.org/supply-chain#>
           PREFIX prov: <http://www.w3.org/ns/prov#>
           
           SELECT ?activity ?agent ?timestamp ?location WHERE {{
               :{batch_id} prov:wasUsedIn ?activity .
               ?activity prov:wasAssociatedWith ?agent ;
                         :recordedAt ?timestamp .
               OPTIONAL {{ ?activity :atLocation ?location . }}
           }}
           ORDER BY ?timestamp
           """
           
           results = self.client.execute_sparql(query)
           return results['results']['bindings']
   
   # Usage
   tracker = SupplyChainTracker("YOUR_API_KEY")
   
   # Add a new batch
   block_index = tracker.add_product_batch(
       batch_id="TOMATO-2025-001",
       product="OrganicTomatoes",
       farm="GreenValleyFarm",
       harvest_date="2025-01-15"
   )
   print(f"Added batch to block {block_index}")
   
   # Trace the batch
   trace = tracker.trace_batch("TOMATO-2025-001")
   for event in trace:
       print(f"Event: {event.get('activity', {}).get('value', 'Unknown')}")

Environmental Monitoring
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python
   # Python - Environmental monitoring example
   def add_environmental_data(client, batch_id, temperature, humidity, location, timestamp):
       """Add environmental monitoring data"""
       rdf_data = f"""
       @prefix : <http://example.org/supply-chain#> .
       @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
       
       :{batch_id} :transportedThrough [
           a :TransportActivity ;
           :environmentalCondition [
               a :EnvironmentalCondition ;
               :temperature "{temperature}"^^xsd:decimal ;
               :humidity "{humidity}"^^xsd:decimal ;
               :location :{location} ;
               :recordedAt "{timestamp}"^^xsd:dateTime
           ]
       ] .
       """
       
       result = client.add_rdf_data(rdf_data)
       return result['block_index']

Quality Assurance
~~~~~~~~~~~~~~~~~

.. code-block:: python
   # Python - Quality assurance example
   def check_temperature_compliance(client, batch_id, max_temp=8.0):
       """Check if a batch has maintained proper temperature"""
       query = f"""
       PREFIX : <http://example.org/supply-chain#>
       
       SELECT ?temperature ?timestamp WHERE {{
           :{batch_id} :transportedThrough ?transport .
           ?transport :environmentalCondition ?condition .
           ?condition :temperature ?temperature ;
                      :recordedAt ?timestamp .
           FILTER(?temperature > {max_temp})
       }}
       ORDER BY ?timestamp
       """
       
       results = client.execute_sparql(query)
       violations = results['results']['bindings']
       
       if violations:
           print(f"Temperature violations found: {len(violations)}")
           for violation in violations:
               temp = violation['temperature']['value']
               timestamp = violation['timestamp']['value']
               print(f"  {timestamp}: {temp}°C")
       else:
           print("All temperature readings within compliance limits")
       
       return len(violations) == 0

Best Practices
-------------

1. **Secure Credential Storage**: Never hardcode API keys in source code
2. **Error Handling**: Always implement proper error handling
3. **Rate Limiting**: Respect API rate limits and implement backoff strategies
4. **Connection Management**: Use connection pooling for better performance
5. **Logging**: Enable appropriate logging levels for debugging
6. **Validation**: Validate data before sending to the API
7. **Caching**: Use caching for frequently accessed data
8. **Monitoring**: Monitor API usage and performance metrics

Troubleshooting
--------------

Common Issues
~~~~~~~~~~~~~

**Connection Timeouts**
- Increase timeout values in client configuration
- Check network connectivity to the API endpoint
- Verify firewall settings

**Authentication Errors**
- Verify API key validity and format
- Check that the key has appropriate permissions
- Ensure proper authentication method is being used

**Rate Limiting**
- Implement exponential backoff strategies
- Use appropriate authentication methods for higher limits
- Consider batching operations to reduce request count

**Data Validation Errors**
- Validate RDF syntax before sending
- Check ontology compliance
- Ensure all required properties are present

**SSL/TLS Issues**
- Update certificates and CA bundles
- Verify certificate chain validity
- Check for certificate expiration

Getting Help
------------

For issues with client libraries:

1. **Check Documentation**: Review this documentation and API references
2. **Review Examples**: Look at provided examples for common patterns
3. **Check GitHub Issues**: Search existing issues or report new ones
4. **Community Support**: Join community discussions for help
5. **Enterprise Support**: Contact support for commercial assistance

**Resources:**
- **GitHub Repositories**: Source code and issue tracking
- **API Documentation**: Complete API reference
- **Community Forum**: Peer support and best practices
- **Example Projects**: Complete working examples

.. raw:: html

   <div class="footer-note">
     <p><strong>Ready to integrate?</strong> Check out our <a href="https://github.com/provchain-org">GitHub repositories</a> for complete examples or dive into the <a href="rest-api.html">REST API documentation</a> for detailed endpoint information.</p>
   </div>
