ProvChainOrg Developer Documentation
====================================

.. raw:: html

   <div class="hero-section">
     <div class="hero-content">
       <h1>Build with Semantic Blockchain Technology</h1>
       <p class="hero-subtitle">ProvChainOrg combines blockchain security with semantic web technologies for transparent, queryable supply chain traceability.</p>
       <div class="hero-badges">
         <span class="badge badge-version">Version 0.1.0</span>
         <span class="badge badge-rust">Rust 1.70+</span>
         <span class="badge badge-license">MIT License</span>
       </div>
     </div>
   </div>

.. note::
   This documentation is designed to help you build with ProvChainOrg. It covers concepts, explains the technology stack, and provides practical guides for building semantic blockchain applications.

Quick Start
-----------

Get up and running with ProvChainOrg in minutes:

.. code-block:: bash

   # Install Rust (if needed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Clone and run
   git clone https://github.com/anusornc/provchain-org.git
   cd provchain-org
   cargo run demo

   # Try a SPARQL query
   cargo run -- query queries/trace_by_batch_ontology.sparql

.. raw:: html

   <div class="quick-links">
     <a href="foundational/intro-to-provchainorg.html" class="quick-link">
       <h3>🚀 New to ProvChainOrg?</h3>
       <p>Start with the basics and learn core concepts</p>
     </a>
     <a href="stack/intro-to-stack.html" class="quick-link">
       <h3>🛠️ Ready to Build?</h3>
       <p>Explore the development stack and tools</p>
     </a>
     <a href="tutorials/first-supply-chain.html" class="quick-link">
       <h3>📚 Follow a Tutorial</h3>
       <p>Build your first supply chain application</p>
     </a>
   </div>

Development Modules
-------------------

If this is your first time with ProvChainOrg development, we recommend starting at the beginning and working your way through like a book.

Foundational Topics
~~~~~~~~~~~~~~~~~~~

Learn the core concepts that make ProvChainOrg unique:

.. toctree::
   :maxdepth: 1

   foundational/intro-to-provchainorg
   foundational/intro-to-rdf-blockchain
   foundational/intro-to-supply-chain-traceability

ProvChainOrg Stack
~~~~~~~~~~~~~~~~~~

Understand the tools and technologies for building applications:

.. toctree::
   :maxdepth: 1

   stack/intro-to-stack

Tutorials & Guides
~~~~~~~~~~~~~~~~~~

Step-by-step guides for common use cases:

.. toctree::
   :maxdepth: 1

   tutorials/first-supply-chain

What Makes ProvChainOrg Different?
----------------------------------

.. raw:: html

   <div class="feature-grid">
     <div class="feature-item">
       <h3>🔗 RDF-Native Blockchain</h3>
       <p>Store semantic data directly in blocks with cryptographic integrity</p>
     </div>
     <div class="feature-item">
       <h3>🔍 SPARQL Queries</h3>
       <p>Query across the entire blockchain using standard semantic web technologies</p>
     </div>
     <div class="feature-item">
       <h3>🧠 Ontology Validation</h3>
       <p>Automatic validation against formal ontologies ensures data quality</p>
     </div>
     <div class="feature-item">
       <h3>📊 Supply Chain Focus</h3>
       <p>Built specifically for transparent, verifiable supply chain traceability</p>
     </div>
   </div>

Use Cases
---------

ProvChainOrg is designed for applications that need:

- **Food Safety**: Track products from farm to table with environmental monitoring
- **Pharmaceutical Traceability**: Ensure drug authenticity and prevent counterfeiting  
- **Luxury Goods Authentication**: Verify provenance and prevent fraud
- **Regulatory Compliance**: Maintain immutable audit trails for compliance
- **Sustainability Tracking**: Monitor environmental impact across supply chains

Community & Support
--------------------

.. raw:: html

   <div class="community-links">
     <a href="https://github.com/anusornc/provchain-org" class="community-link">
       <h4>📦 GitHub Repository</h4>
       <p>Source code, issues, and contributions</p>
     </a>
     <a href="https://github.com/anusornc/provchain-org/discussions" class="community-link">
       <h4>💬 Discussions</h4>
       <p>Community Q&A and feature requests</p>
     </a>
     <a href="https://github.com/anusornc/provchain-org/issues" class="community-link">
       <h4>🐛 Issue Tracker</h4>
       <p>Bug reports and feature requests</p>
     </a>
   </div>

Contributing
------------

ProvChainOrg is open source and welcomes contributions:

- **Documentation**: Help improve these docs
- **Code**: Submit bug fixes and new features
- **Testing**: Help test new releases
- **Examples**: Share your use cases and implementations

See our `Contributing Guide <https://github.com/anusornc/provchain-org/blob/main/CONTRIBUTING.md>`_ for details.

Research Background
-------------------

ProvChainOrg is based on the GraphChain research concept:

.. epigraph::

   "GraphChain – A Distributed Database with Explicit Semantics and Chained RDF Graphs"
   
   -- Sopek, M., et al. (2018), The 2018 Web Conference

Our implementation extends the original research with production-ready features, comprehensive ontology support, and real-world supply chain use cases.

License
-------

ProvChainOrg is released under the `MIT License <https://github.com/anusornc/provchain-org/blob/main/LICENSE>`_.

.. raw:: html

   <div class="footer-note">
     <p><strong>Ready to get started?</strong> Begin with <a href="foundational/intro-to-provchainorg.html">Introduction to ProvChainOrg</a> or jump straight to <a href="tutorials/first-supply-chain.html">building your first application</a>.</p>
   </div>
