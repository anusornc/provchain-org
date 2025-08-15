// Interactive documentation elements

// Code playground functionality
document.addEventListener('DOMContentLoaded', function() {
    // Initialize code playgrounds
    const playgrounds = document.querySelectorAll('.code-playground');
    playgrounds.forEach(function(playground) {
        const runButton = playground.querySelector('.run-button');
        if (runButton) {
            runButton.addEventListener('click', function() {
                runCode(playground);
            });
        }
    });

    // Initialize toggle sections
    const toggleHeaders = document.querySelectorAll('.toggle-header');
    toggleHeaders.forEach(function(header) {
        header.addEventListener('click', function() {
            toggleSection(this);
        });
    });

    // Initialize sortable tables
    const sortableHeaders = document.querySelectorAll('.sortable');
    sortableHeaders.forEach(function(header) {
        header.addEventListener('click', function() {
            sortTable(this);
        });
    });
});

// Run code in playground
function runCode(playground) {
    const editor = playground.querySelector('.code-editor');
    const output = playground.querySelector('.code-output');
    
    if (editor && output) {
        const code = editor.textContent || editor.innerText;
        output.textContent = 'Running code...\n\n';
        
        // Simulate code execution
        setTimeout(function() {
            output.textContent += 'Code executed successfully!\n';
            output.textContent += 'Output: Hello, ProvChainOrg!\n';
        }, 1000);
    }
}

// Toggle section visibility
function toggleSection(header) {
    const section = header.parentElement;
    const content = section.querySelector('.toggle-content');
    const icon = header.querySelector('.toggle-icon');
    
    if (content && icon) {
        if (content.style.display === 'none') {
            content.style.display = 'block';
            icon.textContent = '−';
        } else {
            content.style.display = 'none';
            icon.textContent = '+';
        }
    }
}

// Sort table by column
function sortTable(header) {
    const table = header.closest('table');
    const columnIndex = Array.from(header.parentElement.children).indexOf(header);
    const rows = Array.from(table.querySelectorAll('tbody tr'));
    
    const isAscending = header.classList.contains('ascending');
    header.classList.toggle('ascending', !isAscending);
    header.classList.toggle('descending', isAscending);
    
    rows.sort(function(a, b) {
        const aText = a.children[columnIndex].textContent.trim();
        const bText = b.children[columnIndex].textContent.trim();
        
        if (isAscending) {
            return aText.localeCompare(bText);
        } else {
            return bText.localeCompare(aText);
        }
    });
    
    const tbody = table.querySelector('tbody');
    rows.forEach(function(row) {
        tbody.appendChild(row);
    });
}

// API explorer functionality
function executeApiRequest(explorer) {
    const request = explorer.querySelector('.api-request');
    const response = explorer.querySelector('.api-response');
    
    if (request && response) {
        const method = request.querySelector('.method').textContent;
        const url = request.querySelector('.url').textContent;
        
        response.textContent = 'Executing request...\n\n';
        
        // Simulate API request
        setTimeout(function() {
            response.textContent += `${method} ${url}\n`;
            response.textContent += 'Status: 200 OK\n\n';
            response.textContent += '{\n';
            response.textContent += '  "success": true,\n';
            response.textContent += '  "data": {\n';
            response.textContent += '    "message": "API request executed successfully"\n';
            response.textContent += '  }\n';
            response.textContent += '}\n';
        }, 1500);
    }
}

// Initialize API explorers
document.addEventListener('DOMContentLoaded', function() {
    const explorers = document.querySelectorAll('.api-explorer');
    explorers.forEach(function(explorer) {
        const runButton = explorer.querySelector('.run-button');
        if (runButton) {
            runButton.addEventListener('click', function() {
                executeApiRequest(explorer);
            });
        }
    });
});

// Diagram zoom functionality
function zoomDiagram(diagram, factor) {
    const img = diagram.querySelector('img');
    if (img) {
        const currentWidth = img.style.width || '100%';
        const currentPercent = parseInt(currentWidth) || 100;
        const newPercent = currentPercent * factor;
        
        // Limit zoom between 25% and 400%
        if (newPercent >= 25 && newPercent <= 400) {
            img.style.width = newPercent + '%';
        }
    }
}

// Reset diagram view
function resetDiagram(diagram) {
    const img = diagram.querySelector('img');
    if (img) {
        img.style.width = '100%';
    }
}

// Initialize diagram controls
document.addEventListener('DOMContentLoaded', function() {
    const diagrams = document.querySelectorAll('.interactive-diagram');
    diagrams.forEach(function(diagram) {
        const zoomIn = diagram.querySelector('.zoom-in');
        const zoomOut = diagram.querySelector('.zoom-out');
        const reset = diagram.querySelector('.reset-view');
        
        if (zoomIn) {
            zoomIn.addEventListener('click', function() {
                zoomDiagram(diagram, 1.2);
            });
        }
        
        if (zoomOut) {
            zoomOut.addEventListener('click', function() {
                zoomDiagram(diagram, 0.8);
            });
        }
        
        if (reset) {
            reset.addEventListener('click', function() {
                resetDiagram(diagram);
            });
        }
    });
});

// Form submission
function submitForm(form) {
    const submitButton = form.querySelector('.form-submit');
    const originalText = submitButton.textContent;
    
    submitButton.textContent = 'Submitting...';
    submitButton.disabled = true;
    
    // Simulate form submission
    setTimeout(function() {
        submitButton.textContent = originalText;
        submitButton.disabled = false;
        
        // Show success message
        const successMessage = document.createElement('div');
        successMessage.className = 'success-message';
        successMessage.textContent = 'Form submitted successfully!';
        successMessage.style.cssText = 'color: #28a745; margin-top: 10px; font-weight: bold;';
        
        form.appendChild(successMessage);
        
        // Remove message after 3 seconds
        setTimeout(function() {
            if (successMessage.parentNode) {
                successMessage.parentNode.removeChild(successMessage);
            }
        }, 3000);
    }, 1000);
}

// Initialize forms
document.addEventListener('DOMContentLoaded', function() {
    const forms = document.querySelectorAll('.interactive-form');
    forms.forEach(function(form) {
        form.addEventListener('submit', function(e) {
            e.preventDefault();
            submitForm(form);
        });
    });
});
