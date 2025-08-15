SPARQL API Guide
================

The ProvChainOrg SPARQL API provides a W3C-compliant interface for querying semantic blockchain data. This guide covers all aspects of using SPARQL with ProvChainOrg, from basic queries to advanced optimization techniques.

Overview
--------

ProvChainOrg exposes a full SPARQL 1.1 endpoint that allows you to query across all blockchain data stored as RDF graphs. The SPARQL endpoint supports:

- **SPARQL 1.1 Query** - Data retrieval with complex patterns
- **SPARQL 1.1 Update** - Data modification (when authorized)
- **SPARQL Federated Queries** - Querying external RDF datasets
- **SPARQL Algebra** - Advanced query optimization
- **SPARQL Results** - Multiple serialization formats

Endpoint
--------

**SPARQL Endpoint:** ``POST /sparql``

**Content-Type Headers:**
- ``application/sparql-query`` - For SELECT, ASK, and DESCRIBE queries
- ``application/sparql-update`` - For INSERT, DELETE, and MODIFY operations

**Authentication:**
Include API key in headers:
```http
Authorization: Bearer YOUR_API_KEY
```

Named Graphs
------------

ProvChainOrg organizes blockchain data in named graphs:

```
http://provchain.org/block/{index}    - Data for block {index}
http://provchain.org/ontology         - Traceability ontology
http://provchain.org/metadata         - Blockchain metadata
```

Basic Queries
-------------

SELECT Queries
~~~~~~~~~~~~~~

Retrieve specific data with variable bindings.

**Example: Find all product batches**
```sparql
PREFIX : <http://example.org/supply-chain#>
SELECT ?batch ?product ?farm WHERE {
    ?batch a :ProductBatch ;
           :product ?product ;
           :originFarm ?farm .
}
```

**Example: Get batch details with filtering**
```sparql
PREFIX : <http://example.org/supply-chain#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

SELECT ?batch ?product ?harvestDate WHERE {
    ?batch a :ProductBatch ;
           :product ?product ;
           :harvestDate ?harvestDate .
    FILTER(?harvestDate >= "2025-01-01"^^xsd:date)
}
ORDER BY DESC(?harvestDate)
LIMIT 10
```

ASK Queries
~~~~~~~~~~

Check if data exists without retrieving results.

**Example: Check for specific batch**
```sparql
PREFIX : <http://example.org/supply-chain#>

ASK WHERE {
    :Batch001 a :ProductBatch .
}
```

DESCRIBE Queries
~~~~~~~~~~~~~~~

Retrieve RDF data describing resources.

**Example: Get complete batch information**
```sparql
PREFIX : <http://example.org/supply-chain#>

DESCRIBE :Batch001
```

CONSTRUCT Queries
~~~~~~~~~~~~~~~~~

Generate new RDF data from query results.

**Example: Create batch summary**
```sparql
PREFIX : <http://example.org/supply-chain#>
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

CONSTRUCT {
    ?batch a :ProductBatchSummary ;
           :hasProduct ?product ;
           :hasOriginFarm ?farm ;
           :hasHarvestDate ?date .
} WHERE {
    ?batch a :ProductBatch ;
           :product ?product ;
           :originFarm ?farm ;
           :harvestDate ?date .
}
```

Advanced Queries
----------------

Graph Patterns
~~~~~~~~~~~~~~

Query across multiple named graphs.

**Example: Query all blockchain data**
```sparql
PREFIX : <http://example.org/supply-chain#>

SELECT ?batch ?product ?timestamp WHERE {
    GRAPH ?blockGraph {
        ?batch a :ProductBatch ;
               :product ?product .
    }
    ?blockGraph :blockIndex ?index ;
                :timestamp ?timestamp .
}
ORDER BY ?timestamp
```

Optional Patterns
~~~~~~~~~~~~~~~~~

Include optional data that may not exist.

**Example: Get batches with optional environmental data**
```sparql
PREFIX : <http://example.org/supply-chain#>

SELECT ?batch ?product ?temperature ?humidity WHERE {
    ?batch a :ProductBatch ;
           :product ?product .
    OPTIONAL {
        ?batch :transportedThrough ?transport .
        ?transport :environmentalCondition ?condition .
        ?condition :temperature ?temperature ;
                   :humidity ?humidity .
    }
}
```

Union Queries
~~~~~~~~~~~~~

Combine results from multiple patterns.

**Example: Find products from farms or processing plants**
```sparql
PREFIX : <http://example.org/supply-chain#>

SELECT ?batch ?location ?type WHERE {
    {
        ?batch a :ProductBatch ;
               :originFarm ?farm .
        ?farm :location ?location .
        BIND("Farm" AS ?type)
    }
    UNION
    {
        ?batch a :ProductBatch ;
               :processedAt ?plant .
        ?plant :location ?location .
        BIND("Processing Plant" AS ?type)
    }
}
```

Subqueries
~~~~~~~~~~

Use nested queries for complex filtering.

**Example: Find batches with recent environmental monitoring**
```sparql
PREFIX : <http://example.org/supply-chain#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

SELECT ?batch ?product WHERE {
    ?batch a :ProductBatch ;
           :product ?product .
    ?batch :transportedThrough ?transport .
    ?transport :environmentalCondition ?condition .
    ?condition :recordedAt ?timestamp .
    FILTER(?timestamp >= NOW() - "PT24H"^^xsd:duration)
}
```

Aggregate Functions
~~~~~~~~~~~~~~~~~~~

Perform calculations on query results.

**Example: Count batches by product type**
```sparql
PREFIX : <http://example.org/supply-chain#>

SELECT ?product (COUNT(?batch) AS ?batchCount) WHERE {
    ?batch a :ProductBatch ;
           :product ?product .
}
GROUP BY ?product
ORDER BY DESC(?batchCount)
```

**Example: Calculate average temperature**
```sparql
PREFIX : <http://example.org/supply-chain#>

SELECT (AVG(?temperature) AS ?avgTemp) (MAX(?temperature) AS ?maxTemp) WHERE {
    ?condition :temperature ?temperature .
}
```

Window Functions
~~~~~~~~~~~~~~~

Perform calculations over result sets.

**Example: Rank batches by harvest date**
```sparql
PREFIX : <http://example.org/supply-chain#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

SELECT ?batch ?harvestDate (RANK() OVER (ORDER BY ?harvestDate DESC) AS ?rank) WHERE {
    ?batch a :ProductBatch ;
           :harvestDate ?harvestDate .
}
```

Federated Queries
-----------------

Query external RDF datasets alongside blockchain data.

**Example: Query external product catalog**
```sparql
PREFIX : <http://example.org/supply-chain#>
PREFIX skos: <http://www.w3.org/2004/02/skos/core#>

SELECT ?batch ?product ?externalDescription WHERE {
    ?batch a :ProductBatch ;
           :product ?product .
    SERVICE <http://external-catalog.org/sparql> {
        ?product skos:prefLabel ?externalDescription .
    }
}
```

Ontology-Aware Queries
----------------------

Leverage the traceability ontology for semantic queries.

**Example: Find all organic products**
```sparql
PREFIX trace: <http://provchain.org/trace#>
PREFIX owl: <http://www.w3.org/2002/07/owl#>

SELECT ?batch ?product WHERE {
    ?batch a trace:ProductBatch ;
           trace:product ?product .
    ?product trace:isOrganic true .
}
```

**Example: Trace complete supply chain**
```sparql
PREFIX trace: <http://provchain.org/trace#>
PREFIX prov: <http://www.w3.org/ns/prov#>

SELECT ?batch ?activity ?agent ?timestamp WHERE {
    ?batch a trace:ProductBatch ;
           trace:hasBatchID "MB001" .
    ?activity prov:used ?batch ;
              prov:wasAssociatedWith ?agent ;
              trace:recordedAt ?timestamp .
}
ORDER BY ?timestamp
```

Property Paths
--------------

Use property paths for complex graph traversals.

**Example: Find all products from a specific farm (direct or indirect)**
```sparql
PREFIX : <http://example.org/supply-chain#>

SELECT ?batch ?product WHERE {
    :GreenValleyFarm :supplies*/:hasBatch ?batch .
    ?batch :product ?product .
}
```

**Example: Find environmental conditions along transport path**
```sparql
PREFIX : <http://example.org/supply-chain#>

SELECT ?batch ?temp ?humidity ?location WHERE {
    ?batch :transportedThrough*/:hasCondition ?condition .
    ?condition :temperature ?temp ;
               :humidity ?humidity ;
               :location ?location .
}
```

Query Optimization
------------------

Indexing Strategies
~~~~~~~~~~~~~~~~~~~

ProvChainOrg automatically indexes common query patterns:

- **Subject Index**: Fast lookup by subject URI
- **Predicate Index**: Fast lookup by predicate URI
- **Object Index**: Fast lookup by literal values
- **Triple Pattern Index**: Optimized for triple pattern matching

**Best Practices:**
1. Use LIMIT clauses for large result sets
2. Filter early in the query
3. Use specific property paths instead of wildcards
4. Leverage named graph patterns when possible

Query Hints
~~~~~~~~~~~

Provide optimization hints to the query engine.

**Example: Use specific index hint**
```sparql
PREFIX : <http://example.org/supply-chain#>

SELECT ?batch ?product WHERE {
    /*+ INDEX(subject) */
    ?batch a :ProductBatch ;
           :product ?product .
}
```

Performance Monitoring
~~~~~~~~~~~~~~~~~~~~~~

Monitor query performance with EXPLAIN.

**Example: Get query execution plan**
```sparql
PREFIX : <http://example.org/supply-chain#>

EXPLAIN
SELECT ?batch ?product WHERE {
    ?batch a :ProductBatch ;
           :product ?product .
}
```

Update Operations
-----------------

INSERT Data
~~~~~~~~~~~

Add new RDF data to the blockchain.

**Example: Add new product batch**
```sparql
PREFIX : <http://example.org/supply-chain#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

INSERT DATA {
    :Batch003 a :ProductBatch ;
              :hasBatchID "BATCH-003" ;
              :product :OrganicApples ;
              :harvestDate "2025-01-15"^^xsd:date ;
              :originFarm :AppleOrchardFarm .
}
```

DELETE Data
~~~~~~~~~~~

Remove specific RDF triples.

**Example: Remove outdated environmental data**
```sparql
PREFIX : <http://example.org/supply-chain#>

DELETE {
    :Batch001 :hasCondition ?oldCondition .
}
WHERE {
    :Batch001 :hasCondition ?oldCondition .
    ?oldCondition :recordedAt ?timestamp .
    FILTER(?timestamp < "2025-01-01"^^xsd:dateTime)
}
```

MODIFY Operations
~~~~~~~~~~~~~~~~~

Combine insert and delete operations.

**Example: Update batch status**
```sparql
PREFIX : <http://example.org/supply-chain#>

MODIFY {
    DELETE {
        :Batch001 :status ?oldStatus .
    }
    INSERT {
        :Batch001 :status :Shipped .
    }
    WHERE {
        :Batch001 :status ?oldStatus .
        FILTER(?oldStatus = :Processed)
    }
}
```

LOAD Operations
~~~~~~~~~~~~~~

Load external RDF data.

**Example: Load external ontology**
```sparql
LOAD <http://example.org/external-ontology.ttl>
INTO GRAPH <http://provchain.org/external>
```

Query Templates
---------------

Reusable query patterns for common operations.

Batch Tracking Template
~~~~~~~~~~~~~~~~~~~~~~~

```sparql
# Template: Track batch through supply chain
PREFIX trace: <http://provchain.org/trace#>
PREFIX prov: <http://www.w3.org/ns/prov#>

SELECT ?batch ?activity ?agent ?timestamp ?location WHERE {
    ?batch a trace:ProductBatch ;
           trace:hasBatchID "{{BATCH_ID}}" .
    ?activity prov:used ?batch ;
              prov:wasAssociatedWith ?agent ;
              trace:recordedAt ?timestamp .
    OPTIONAL {
        ?activity :atLocation ?location .
    }
}
ORDER BY ?timestamp
```

Environmental Monitoring Template
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

```sparql
# Template: Monitor environmental conditions
PREFIX : <http://example.org/supply-chain#>

SELECT ?batch ?temperature ?humidity ?location ?timestamp WHERE {
    ?batch :transportedThrough ?transport .
    ?transport :environmentalCondition ?condition .
    ?condition :temperature ?temperature ;
               :humidity ?humidity ;
               :location ?location ;
               :recordedAt ?timestamp .
    FILTER(?temperature > {{TEMP_THRESHOLD}})
}
ORDER BY ?timestamp DESC
```

Quality Assurance Template
~~~~~~~~~~~~~~~~~~~~~~~~~~

```sparql
# Template: Quality assurance checks
PREFIX trace: <http://provchain.org/trace#>

SELECT ?batch ?issue ?severity WHERE {
    ?batch a trace:ProductBatch ;
           trace:hasBatchID "{{BATCH_ID}}" .
    ?batch trace:qualityIssue ?issue .
    ?issue trace:hasSeverity ?severity .
    FILTER(?severity IN {{SEVERITY_LEVELS}})
}
```

Error Handling
--------------

Common Query Errors
~~~~~~~~~~~~~~~~~~~

**Syntax Errors**
   - Check SPARQL syntax carefully
   - Use proper prefixes and URIs
   - Ensure balanced parentheses and brackets

**Timeout Errors**
   - Add LIMIT clauses to large queries
   - Optimize FILTER conditions
   - Use more specific patterns

**Memory Errors**
   - Break complex queries into smaller parts
   - Use pagination with OFFSET/LIMIT
   - Consider using CONSTRUCT instead of SELECT for large results

**Permission Errors**
   - Verify API key is valid
   - Check user permissions for update operations
   - Ensure authentication headers are properly set

Debugging Techniques
~~~~~~~~~~~~~~~~~~~~

**Step-by-step Query Building**
1. Start with simple pattern matching
2. Add optional elements gradually
3. Include filters one at a time
4. Test with LIMIT clauses first

**Query Validation**
```sparql
# Validate query structure
ASK WHERE {
    # Test basic pattern matching first
    ?s ?p ?o .
}
```

**Variable Binding Inspection**
```sparql
# Check intermediate results
SELECT ?batch ?product WHERE {
    ?batch a :ProductBatch ;
           :product ?product .
}
LIMIT 5  # Test with small result set first
```

Best Practices
--------------

1. **Use Meaningful Variable Names**: Make queries more readable and maintainable.

2. **Leverage Named Graphs**: Specify exact graphs when possible for better performance.

3. **Filter Early**: Apply FILTER conditions as early as possible in the query.

4. **Use LIMIT Clauses**: Always use LIMIT for exploratory queries to avoid large result sets.

5. **Optimize Property Paths**: Use specific paths instead of wildcards when possible.

6. **Cache Results**: Cache frequently executed queries for better performance.

7. **Use Templates**: Create reusable query templates for common operations.

8. **Monitor Performance**: Regularly review query performance and optimize as needed.

9. **Document Complex Queries**: Add comments to explain complex query logic.

10. **Test with Sample Data**: Test queries with representative sample data before production use.

Example Integration
------------------

Python Integration
~~~~~~~~~~~~~~~~~~~

```python
import requests
import json

class ProvChainSPARQL:
    def __init__(self, base_url, api_key):
        self.base_url = base_url
        self.headers = {
            'Authorization': f'Bearer {api_key}',
            'Content-Type': 'application/sparql-query'
        }
    
    def execute_query(self, sparql_query, format='json'):
        """Execute a SPARQL query and return results."""
        params = {'format': format} if format else {}
        response = requests.post(
            f'{self.base_url}/sparql',
            headers=self.headers,
            data=sparql_query,
            params=params
        )
        response.raise_for_status()
        return response.json()
    
    def track_batch(self, batch_id):
        """Track a batch through the supply chain."""
        query = f"""
        PREFIX trace: <http://provchain.org/trace#>
        PREFIX prov: <http://www.w3.org/ns/prov#>
        
        SELECT ?activity ?agent ?timestamp ?location WHERE {{
            ?batch a trace:ProductBatch ;
                   trace:hasBatchID "{batch_id}" .
            ?activity prov:used ?batch ;
                      prov:wasAssociatedWith ?agent ;
                      trace:recordedAt ?timestamp .
            OPTIONAL {{
                ?activity :atLocation ?location .
            }}
        }}
        ORDER BY ?timestamp
        """
        return self.execute_query(query)
    
    def get_environmental_conditions(self, batch_id, temp_threshold=5.0):
        """Get environmental conditions for a batch."""
        query = f"""
        PREFIX : <http://example.org/supply-chain#>
        
        SELECT ?temperature ?humidity ?location ?timestamp WHERE {{
            :{batch_id} :transportedThrough ?transport .
            ?transport :environmentalCondition ?condition .
            ?condition :temperature ?temperature ;
                       :humidity ?humidity ;
                       :location ?location ;
                       :recordedAt ?timestamp .
            FILTER(?temperature > {temp_threshold})
        }}
        ORDER BY ?timestamp DESC
        """
        return self.execute_query(query)

# Usage
sparql_api = ProvChainSPARQL('http://localhost:8080', 'YOUR_API_KEY')

# Track a batch
trace = sparql_api.track_batch('BATCH-001')
print(f"Batch trace: {len(trace['results']['bindings'])} events")

# Check environmental conditions
conditions = sparql_api.get_environmental_conditions('BATCH-001')
print(f"High temperature events: {len(conditions['results']['bindings'])}")
```

JavaScript Integration
~~~~~~~~~~~~~~~~~~~~~~

```javascript
class ProvChainSPARQL {
    constructor(baseUrl, apiKey) {
        this.baseUrl = baseUrl;
        this.headers = {
            'Authorization': `Bearer ${apiKey}`,
            'Content-Type': 'application/sparql-query'
        };
    }

    async executeQuery(sparqlQuery, format = 'json') {
        const params = format ? { format } : {};
        
        const response = await fetch(`${this.baseUrl}/sparql`, {
            method: 'POST',
            headers: this.headers,
            body: sparqlQuery,
            params
        });
        
        if (!response.ok) {
            throw new Error(`Query failed: ${response.statusText}`);
        }
        
        return await response.json();
    }

    async getBatchStatus(batchId) {
        const query = `
            PREFIX trace: <http://provchain.org/trace#>
            
            SELECT ?status ?timestamp WHERE {
                :${batchId} a trace:ProductBatch ;
                           trace:hasStatus ?status ;
                           trace:statusTimestamp ?timestamp .
            }
            ORDER BY DESC(?timestamp)
            LIMIT 1
        `;
        
        return await this.executeQuery(query);
    }

    async findBatchesByFarm(farmName) {
        const query = `
            PREFIX : <http://example.org/supply-chain#>
            
            SELECT ?batch ?product ?harvestDate WHERE {
                ?batch a :ProductBatch ;
                       :originFarm ?farm ;
                       :product ?product ;
                       :harvestDate ?harvestDate .
                ?farm :name "${farmName}" .
            }
            ORDER BY ?harvestDate DESC
        `;
        
        return await this.executeQuery(query);
    }
}

// Usage
const sparqlApi = new ProvChainSPARQL('http://localhost:8080', 'YOUR_API_KEY');

// Get batch status
sparqlApi.getBatchStatus('BATCH-001').then(status => {
    console.log('Current status:', status.results.bindings[0]);
});

// Find batches from specific farm
sparqlApi.findBatchesByFarm('Green Valley Farm').then(batches => {
    console.log(`Found ${batches.results.bindings.length} batches`);
});
```

Troubleshooting
---------------

Common Issues
~~~~~~~~~~~~~

**Query Timeout**
   - Reduce result set size with LIMIT
   - Optimize FILTER conditions
   - Break complex queries into simpler parts

**Memory Issues**
   - Use streaming results for large datasets
   - Implement pagination with OFFSET/LIMIT
   - Consider using CONSTRUCT instead of SELECT

**Permission Denied**
   - Verify API key validity
   - Check user permissions for update operations
   - Ensure authentication headers are correct

**No Results**
   - Verify URIs and prefixes are correct
   - Check data exists in expected graphs
   - Test with simpler patterns first

**Performance Issues**
   - Add appropriate indexes
   - Use specific property paths
   - Filter early in the query

Getting Help
~~~~~~~~~~~~

- **SPARQL Specification**: Refer to W3C SPARQL 1.1 specifications
- **Query Examples**: Browse the examples section for common patterns
- **Performance Tuning**: Consult optimization best practices
- **Community Support**: Join discussions for help with complex queries
- **Enterprise Support**: Contact support for production issues

Related Resources
-----------------

- **SPARQL 1.1 Query Language**: W3C Recommendation
- **SPARQL 1.1 Update Language**: W3C Recommendation
- **RDF 1.1 Concepts**: W3C Recommendation
- **ProvChainOntology**: Traceability ontology documentation
- **Best Practices Guide**: Query optimization and performance
