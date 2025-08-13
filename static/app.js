// ProvChain Frontend Application

// Simple HTML escaping function to prevent XSS
function escapeHtml(text) {
    if (typeof text !== 'string') return text;
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

class ProvChainApp {
    constructor() {
        this.apiBaseUrl = 'http://localhost:8080';
        this.authToken = localStorage.getItem('authToken');
        this.currentSection = 'dashboard';
        this.init();
    }

    init() {
        this.setupEventListeners();
        this.loadDashboard();
        this.checkAuthStatus();
        
        // Load initial data
        this.refreshDashboard();
    }

    setupEventListeners() {
        // Navigation
        document.querySelectorAll('.nav-link').forEach(link => {
            link.addEventListener('click', (e) => {
                e.preventDefault();
                const section = link.dataset.section;
                this.showSection(section);
            });
        });

        // Authentication
        document.getElementById('loginBtn').addEventListener('click', () => {
            this.showLoginModal();
        });

        document.getElementById('logoutBtn').addEventListener('click', () => {
            this.logout();
        });

        document.getElementById('loginForm').addEventListener('submit', (e) => {
            e.preventDefault();
            this.handleLogin();
        });

        // Modal close
        document.querySelector('.close').addEventListener('click', () => {
            this.hideLoginModal();
        });

        // Dashboard refresh
        setInterval(() => {
            if (this.currentSection === 'dashboard') {
                this.refreshDashboard();
            }
        }, 30000); // Refresh every 30 seconds

        // Blocks section
        document.getElementById('refreshBlocks').addEventListener('click', () => {
            this.loadBlocks();
        });

        document.getElementById('blockSearch').addEventListener('input', (e) => {
            this.filterBlocks(e.target.value);
        });

        // Traceability
        document.getElementById('traceProduct').addEventListener('click', () => {
            this.traceProduct();
        });

        document.getElementById('enhancedTraceProduct').addEventListener('click', () => {
            this.enhancedTraceProduct();
        });

        // SPARQL
        document.getElementById('executeQuery').addEventListener('click', () => {
            this.executeSparqlQuery();
        });

        document.getElementById('queryTemplates').addEventListener('change', (e) => {
            this.loadQueryTemplate(e.target.value);
        });

        // Transactions
        document.getElementById('addTripleForm').addEventListener('submit', (e) => {
            e.preventDefault();
            this.addTriple();
        });
    }

    // Authentication Methods
    checkAuthStatus() {
        if (this.authToken) {
            this.showUserInfo();
        } else {
            this.showLoginButton();
        }
    }

    showLoginModal() {
        document.getElementById('loginModal').style.display = 'block';
    }

    hideLoginModal() {
        document.getElementById('loginModal').style.display = 'none';
    }

    async handleLogin() {
        const username = document.getElementById('loginUsername').value;
        const password = document.getElementById('loginPassword').value;

        try {
            const response = await fetch(`${this.apiBaseUrl}/auth/login`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ username, password }),
            });

            if (response.ok) {
                const data = await response.json();
                this.authToken = data.token;
                localStorage.setItem('authToken', this.authToken);
                localStorage.setItem('username', username);
                this.showUserInfo();
                this.hideLoginModal();
                this.showToast('Login successful!', 'success');
            } else {
                this.showToast('Login failed. Please check your credentials.', 'error');
            }
        } catch (error) {
            console.error('Login error:', error);
            this.showToast('Login failed. Please try again.', 'error');
        }
    }

    logout() {
        this.authToken = null;
        localStorage.removeItem('authToken');
        localStorage.removeItem('username');
        this.showLoginButton();
        this.showToast('Logged out successfully', 'success');
    }

    showUserInfo() {
        const username = localStorage.getItem('username') || 'User';
        document.getElementById('username').textContent = username;
        document.getElementById('loginBtn').style.display = 'none';
        document.getElementById('userInfo').style.display = 'flex';
    }

    showLoginButton() {
        document.getElementById('loginBtn').style.display = 'block';
        document.getElementById('userInfo').style.display = 'none';
    }

    // Navigation Methods
    showSection(sectionName) {
        // Hide all sections
        document.querySelectorAll('.content-section').forEach(section => {
            section.classList.remove('active');
        });

        // Remove active class from all nav links
        document.querySelectorAll('.nav-link').forEach(link => {
            link.classList.remove('active');
        });

        // Show selected section
        document.getElementById(sectionName).classList.add('active');
        document.querySelector(`[data-section="${sectionName}"]`).classList.add('active');

        this.currentSection = sectionName;

        // Load section-specific data
        switch (sectionName) {
            case 'dashboard':
                this.refreshDashboard();
                break;
            case 'blocks':
                this.loadBlocks();
                break;
            case 'transactions':
                this.loadTransactions();
                break;
        }
    }

    // API Methods
    async apiRequest(endpoint, options = {}) {
        const url = `${this.apiBaseUrl}${endpoint}`;
        const defaultOptions = {
            headers: {
                'Content-Type': 'application/json',
            },
        };

        if (this.authToken) {
            defaultOptions.headers['Authorization'] = `Bearer ${this.authToken}`;
        }

        const finalOptions = { ...defaultOptions, ...options };
        
        try {
            const response = await fetch(url, finalOptions);
            
            if (response.status === 401) {
                this.logout();
                this.showToast('Session expired. Please login again.', 'warning');
                return null;
            }
            
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            
            return await response.json();
        } catch (error) {
            console.error('API request failed:', error);
            this.showToast('API request failed. Please try again.', 'error');
            return null;
        }
    }

    // Dashboard Methods
    async refreshDashboard() {
        await Promise.all([
            this.loadBlockchainStatus(),
            this.loadRecentTransactions(),
            this.checkSystemHealth()
        ]);
    }

    async loadBlockchainStatus() {
        const status = await this.apiRequest('/api/blockchain/status');
        if (status) {
            document.getElementById('blockHeight').textContent = status.height;
            document.getElementById('totalTransactions').textContent = status.total_transactions;
            document.getElementById('networkPeers').textContent = status.network_peers;
            
            const validation = await this.apiRequest('/api/blockchain/validate');
            if (validation) {
                const statusElement = document.getElementById('blockchainStatus');
                statusElement.textContent = validation.is_valid ? 'Valid' : 'Invalid';
                statusElement.className = `health-status ${validation.is_valid ? 'healthy' : 'error'}`;
            }
        }
    }

    async loadRecentTransactions() {
        const transactions = await this.apiRequest('/api/transactions/recent');
        if (transactions) {
            const container = document.getElementById('recentTransactions');
            container.innerHTML = '';

            if (transactions.length === 0) {
                container.innerHTML = '<p>No recent transactions</p>';
                return;
            }

            transactions.slice(0, 5).forEach(tx => {
                const item = document.createElement('div');
                item.className = 'transaction-item';
                item.innerHTML = `
                    <div class="transaction-header">
                        <span class="transaction-block">Block ${escapeHtml(tx.block_index.toString())}</span>
                        <span class="transaction-time">${escapeHtml(this.formatDate(tx.timestamp))}</span>
                    </div>
                    <div class="transaction-triple">${escapeHtml(tx.subject)} ${escapeHtml(tx.predicate)} ${escapeHtml(tx.object)}</div>
                `;
                container.appendChild(item);
            });
        }
    }

    async checkSystemHealth() {
        // Check API status
        const health = await this.apiRequest('/health');
        const apiStatusElement = document.getElementById('apiStatus');
        if (health) {
            apiStatusElement.textContent = 'Online';
            apiStatusElement.className = 'health-status healthy';
        } else {
            apiStatusElement.textContent = 'Offline';
            apiStatusElement.className = 'health-status error';
        }

        // Check last block time
        const status = await this.apiRequest('/api/blockchain/status');
        if (status) {
            document.getElementById('lastBlockTime').textContent = this.formatDate(status.last_updated);
        }

        // Check validation status
        const validation = await this.apiRequest('/api/blockchain/validate');
        const validationElement = document.getElementById('validationStatus');
        if (validation) {
            validationElement.textContent = validation.is_valid ? 'Valid' : 'Invalid';
            validationElement.className = `health-status ${validation.is_valid ? 'healthy' : 'error'}`;
        }
    }

    // Blocks Methods
    async loadBlocks() {
        const blocks = await this.apiRequest('/api/blockchain/blocks');
        if (blocks) {
            this.displayBlocks(blocks);
            this.allBlocks = blocks; // Store for filtering
        }
    }

    displayBlocks(blocks) {
        const container = document.getElementById('blocksList');
        container.innerHTML = '';

        if (blocks.length === 0) {
            container.innerHTML = '<p>No blocks found</p>';
            return;
        }

        blocks.reverse().forEach(block => {
            const item = document.createElement('div');
            item.className = 'block-item';
            item.innerHTML = `
                <div class="block-header">
                    <span class="block-index">Block #${escapeHtml(block.index.toString())}</span>
                    <span class="block-timestamp">${escapeHtml(this.formatDate(block.timestamp))}</span>
                </div>
                <div class="block-hash">Hash: ${escapeHtml(block.hash)}</div>
                <div class="block-hash">Previous: ${escapeHtml(block.previous_hash)}</div>
                <div style="margin-top: 0.5rem; font-size: 0.875rem; color: #666;">
                    Size: ${escapeHtml(block.size_bytes.toString())} bytes | Transactions: ${escapeHtml(block.transaction_count.toString())}
                </div>
            `;
            container.appendChild(item);
        });
    }

    filterBlocks(searchTerm) {
        if (!this.allBlocks) return;

        const filtered = this.allBlocks.filter(block => 
            block.index.toString().includes(searchTerm) ||
            block.hash.toLowerCase().includes(searchTerm.toLowerCase()) ||
            block.previous_hash.toLowerCase().includes(searchTerm.toLowerCase())
        );

        this.displayBlocks(filtered);
    }

    // Traceability Methods
    async traceProduct() {
        const batchId = document.getElementById('batchId').value.trim();
        const productName = document.getElementById('productName').value.trim();

        if (!batchId) {
            this.showToast('Please enter a batch ID', 'warning');
            return;
        }

        const params = new URLSearchParams({ batch_id: batchId });
        if (productName) {
            params.append('product_name', productName);
        }

        const trace = await this.apiRequest(`/api/products/trace?${params}`);
        if (trace) {
            this.displayTraceResults(trace);
        }
    }

    async enhancedTraceProduct() {
        const batchId = document.getElementById('batchId').value.trim();
        const optimizationLevel = document.getElementById('optimizationLevel').value;

        if (!batchId) {
            this.showToast('Please enter a batch ID', 'warning');
            return;
        }

        const params = new URLSearchParams({ 
            batch_id: batchId,
            optimization_level: optimizationLevel
        });

        const startTime = Date.now();
        const trace = await this.apiRequest(`/api/products/trace/enhanced?${params}`);
        const executionTime = Date.now() - startTime;

        if (trace) {
            this.displayEnhancedTraceResults(trace, executionTime);
        }
    }

    displayTraceResults(trace) {
        const container = document.getElementById('traceResults');
        container.innerHTML = `
            <div class="product-info">
                <h3>${escapeHtml(trace.product_name)}</h3>
                <p>Batch ID: ${escapeHtml(trace.batch_id)}</p>
                <div class="product-details">
                    <div class="detail-item">
                        <span class="detail-label">Origin</span>
                        <span class="detail-value">${escapeHtml(trace.origin)}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">Current Location</span>
                        <span class="detail-value">${escapeHtml(trace.current_location)}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">Status</span>
                        <span class="detail-value">${escapeHtml(trace.status)}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">Certifications</span>
                        <span class="detail-value">${escapeHtml(trace.certifications.join(', ')) || 'None'}</span>
                    </div>
                </div>
            </div>
        `;

        if (trace.environmental_data) {
            const envData = trace.environmental_data;
            container.innerHTML += `
                <div class="environmental-data" style="background: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <h4><i class="fas fa-leaf"></i> Environmental Data</h4>
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 1rem; margin-top: 0.5rem;">
                        ${envData.temperature ? `<div><strong>Temperature:</strong> ${escapeHtml(envData.temperature.toString())}°C</div>` : ''}
                        ${envData.humidity ? `<div><strong>Humidity:</strong> ${escapeHtml(envData.humidity.toString())}%</div>` : ''}
                        ${envData.co2_footprint ? `<div><strong>CO2 Footprint:</strong> ${escapeHtml(envData.co2_footprint.toString())} kg</div>` : ''}
                    </div>
                </div>
            `;
        }

        if (trace.timeline && trace.timeline.length > 0) {
            container.innerHTML += '<div class="timeline">';
            trace.timeline.forEach((event, index) => {
                container.innerHTML += `
                    <div class="timeline-item">
                        <div class="timeline-icon">${index + 1}</div>
                        <div class="timeline-content">
                            <div class="timeline-header">
                                <span class="timeline-action">${escapeHtml(event.action)}</span>
                                <span class="timeline-time">${escapeHtml(this.formatDate(event.timestamp))}</span>
                            </div>
                            <div class="timeline-details">
                                <strong>Location:</strong> ${escapeHtml(event.location)}<br>
                                <strong>Actor:</strong> ${escapeHtml(event.actor)}<br>
                                ${escapeHtml(event.details)}
                            </div>
                        </div>
                    </div>
                `;
            });
            container.innerHTML += '</div>';
        } else {
            container.innerHTML += '<p style="text-align: center; color: #666; margin-top: 2rem;">No timeline events found for this product.</p>';
        }
    }

    displayEnhancedTraceResults(trace, executionTime) {
        const container = document.getElementById('traceResults');
        container.innerHTML = `
            <div class="product-info">
                <h3>${escapeHtml(trace.product_name || 'Unknown Product')}</h3>
                <p>Batch ID: ${escapeHtml(trace.batch_id || 'Unknown')}</p>
                <div class="product-details">
                    <div class="detail-item">
                        <span class="detail-label">Origin</span>
                        <span class="detail-value">${escapeHtml(trace.origin || 'Unknown')}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">Current Location</span>
                        <span class="detail-value">${escapeHtml(trace.current_location || 'Unknown')}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">Status</span>
                        <span class="detail-value">${escapeHtml(trace.status || 'Unknown')}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">Certifications</span>
                        <span class="detail-value">${escapeHtml(trace.certifications ? trace.certifications.join(', ') : 'None')}</span>
                    </div>
                </div>
            </div>
        `;

        // Display optimization information
        container.innerHTML += `
            <div class="optimization-info" style="background: #e8f4f8; padding: 1rem; border-radius: 8px; margin: 1rem 0;">
                <h4><i class="fas fa-bolt"></i> Enhanced Trace Optimization</h4>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-top: 0.5rem;">
                    <div><strong>Optimization Level:</strong> ${escapeHtml(trace.optimization_level?.toString() || 'N/A')}</div>
                    <div><strong>Execution Time:</strong> ${escapeHtml(executionTime?.toString() || 'N/A')}ms</div>
                    <div><strong>Path Length:</strong> ${escapeHtml(trace.path_length?.toString() || 'N/A')}</div>
                    <div><strong>Nodes Visited:</strong> ${escapeHtml(trace.nodes_visited?.toString() || 'N/A')}</div>
                </div>
            </div>
        `;

        if (trace.environmental_data) {
            const envData = trace.environmental_data;
            container.innerHTML += `
                <div class="environmental-data" style="background: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <h4><i class="fas fa-leaf"></i> Environmental Data</h4>
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 1rem; margin-top: 0.5rem;">
                        ${envData.temperature ? `<div><strong>Temperature:</strong> ${escapeHtml(envData.temperature.toString())}°C</div>` : ''}
                        ${envData.humidity ? `<div><strong>Humidity:</strong> ${escapeHtml(envData.humidity.toString())}%</div>` : ''}
                        ${envData.co2_footprint ? `<div><strong>CO2 Footprint:</strong> ${escapeHtml(envData.co2_footprint.toString())} kg</div>` : ''}
                    </div>
                </div>
            `;
        }

        if (trace.timeline && trace.timeline.length > 0) {
            container.innerHTML += '<div class="timeline">';
            trace.timeline.forEach((event, index) => {
                container.innerHTML += `
                    <div class="timeline-item">
                        <div class="timeline-icon">${index + 1}</div>
                        <div class="timeline-content">
                            <div class="timeline-header">
                                <span class="timeline-action">${escapeHtml(event.action)}</span>
                                <span class="timeline-time">${escapeHtml(this.formatDate(event.timestamp))}</span>
                            </div>
                            <div class="timeline-details">
                                <strong>Location:</strong> ${escapeHtml(event.location)}<br>
                                <strong>Actor:</strong> ${escapeHtml(event.actor)}<br>
                                ${escapeHtml(event.details)}
                            </div>
                        </div>
                    </div>
                `;
            });
            container.innerHTML += '</div>';
        } else {
            container.innerHTML += '<p style="text-align: center; color: #666; margin-top: 2rem;">No timeline events found for this product.</p>';
        }
    }

    // SPARQL Methods
    loadQueryTemplate(templateName) {
        const templates = {
            'all-triples': `PREFIX tc: <http://example.org/tracechain#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?subject ?predicate ?object
WHERE {
    ?subject ?predicate ?object .
}
LIMIT 100`,
            'batch-trace': `PREFIX tc: <http://example.org/tracechain#>

SELECT ?batch ?product ?origin ?location ?status
WHERE {
    ?batch tc:batchId ?batchId .
    ?batch tc:product ?product .
    OPTIONAL { ?batch tc:origin ?origin }
    OPTIONAL { ?batch tc:currentLocation ?location }
    OPTIONAL { ?batch tc:status ?status }
}`,
            'env-conditions': `PREFIX tc: <http://example.org/tracechain#>

SELECT ?batch ?temperature ?humidity ?co2
WHERE {
    ?batch tc:environmentalData ?envData .
    OPTIONAL { ?envData tc:temperature ?temperature }
    OPTIONAL { ?envData tc:humidity ?humidity }
    OPTIONAL { ?envData tc:co2Footprint ?co2 }
}`,
            'blockchain-metadata': `PREFIX tc: <http://example.org/tracechain#>

SELECT ?block ?hash ?timestamp ?previousHash
WHERE {
    ?block tc:blockHash ?hash .
    ?block tc:timestamp ?timestamp .
    OPTIONAL { ?block tc:previousHash ?previousHash }
}
ORDER BY ?timestamp`
        };

        if (templates[templateName]) {
            document.getElementById('sparqlQuery').value = templates[templateName];
        }
    }

    async executeSparqlQuery() {
        const query = document.getElementById('sparqlQuery').value.trim();
        
        if (!query) {
            this.showToast('Please enter a SPARQL query', 'warning');
            return;
        }

        const startTime = Date.now();
        const result = await this.apiRequest('/api/sparql/query', {
            method: 'POST',
            body: JSON.stringify({ query })
        });

        if (result) {
            const executionTime = Date.now() - startTime;
            this.displaySparqlResults(result, executionTime);
        }
    }

    displaySparqlResults(result, clientExecutionTime) {
        const container = document.getElementById('sparqlResults');
        const statsContainer = document.getElementById('queryStats');
        
        statsContainer.innerHTML = `
            Results: ${result.result_count} | 
            Server Time: ${result.execution_time_ms}ms | 
            Total Time: ${clientExecutionTime}ms
        `;

        if (result.results.boolean !== undefined) {
            // Boolean result
            container.innerHTML = `
                <div style="text-align: center; padding: 2rem;">
                    <h3>Query Result: ${result.results.boolean ? 'TRUE' : 'FALSE'}</h3>
                </div>
            `;
        } else if (result.results.results && result.results.results.bindings) {
            // Table results
            const bindings = result.results.results.bindings;
            
            if (bindings.length === 0) {
                container.innerHTML = '<p style="text-align: center; padding: 2rem;">No results found</p>';
                return;
            }

            // Get column headers
            const headers = Object.keys(bindings[0]);
            
            let tableHTML = '<table class="results-table"><thead><tr>';
            headers.forEach(header => {
                tableHTML += `<th>${header}</th>`;
            });
            tableHTML += '</tr></thead><tbody>';

            bindings.forEach(binding => {
                tableHTML += '<tr>';
                headers.forEach(header => {
                    const value = binding[header] || '';
                    tableHTML += `<td>${value}</td>`;
                });
                tableHTML += '</tr>';
            });

            tableHTML += '</tbody></table>';
            container.innerHTML = tableHTML;
        } else {
            container.innerHTML = `<pre>${JSON.stringify(result.results, null, 2)}</pre>`;
        }
    }

    // Transaction Methods
    async loadTransactions() {
        const transactions = await this.apiRequest('/api/transactions/recent');
        if (transactions) {
            this.displayTransactionsList(transactions);
        }
    }

    displayTransactionsList(transactions) {
        const container = document.getElementById('transactionsList');
        container.innerHTML = '';

        if (transactions.length === 0) {
            container.innerHTML = '<p>No transactions found</p>';
            return;
        }

        transactions.forEach(tx => {
            const item = document.createElement('div');
            item.className = 'transaction-item';
            item.innerHTML = `
                <div class="transaction-header">
                    <span class="transaction-block">Block ${escapeHtml(tx.block_index.toString())}</span>
                    <span class="transaction-time">${escapeHtml(this.formatDate(tx.timestamp))}</span>
                </div>
                <div class="transaction-triple">${escapeHtml(tx.subject)} ${escapeHtml(tx.predicate)} ${escapeHtml(tx.object)}</div>
            `;
            container.appendChild(item);
        });
    }

    async addTriple() {
        const subject = document.getElementById('subject').value.trim();
        const predicate = document.getElementById('predicate').value.trim();
        const object = document.getElementById('object').value.trim();

        if (!subject || !predicate || !object) {
            this.showToast('Please fill in all fields', 'warning');
            return;
        }

        const result = await this.apiRequest('/api/blockchain/add-triple', {
            method: 'POST',
            body: JSON.stringify({ subject, predicate, object })
        });

        if (result && result.success) {
            this.showToast('Triple added successfully!', 'success');
            
            // Clear form
            document.getElementById('subject').value = '';
            document.getElementById('predicate').value = '';
            document.getElementById('object').value = '';
            
            // Refresh transactions list
            this.loadTransactions();
            
            // Refresh dashboard if visible
            if (this.currentSection === 'dashboard') {
                this.refreshDashboard();
            }
        }
    }

    // Utility Methods
    formatDate(dateString) {
        const date = new Date(dateString);
        return date.toLocaleString();
    }

    showToast(message, type = 'info') {
        const container = document.getElementById('toastContainer');
        const toast = document.createElement('div');
        toast.className = `toast ${type}`;
        toast.innerHTML = `
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <i class="fas fa-${this.getToastIcon(type)}"></i>
                <span>${message}</span>
            </div>
        `;
        
        container.appendChild(toast);
        
        // Auto remove after 5 seconds
        setTimeout(() => {
            if (toast.parentNode) {
                toast.parentNode.removeChild(toast);
            }
        }, 5000);
    }

    getToastIcon(type) {
        const icons = {
            success: 'check-circle',
            error: 'exclamation-circle',
            warning: 'exclamation-triangle',
            info: 'info-circle'
        };
        return icons[type] || 'info-circle';
    }

    loadDashboard() {
        this.showSection('dashboard');
    }
}

// Initialize the application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new ProvChainApp();
});
