#!/bin/bash

# End-to-End Test Execution Script for ProvChainOrg
# This script runs the complete E2E testing suite with proper setup and reporting

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
E2E_TIMEOUT=${E2E_TIMEOUT:-300}
E2E_PARALLEL=${E2E_PARALLEL:-true}
E2E_BROWSER_HEADLESS=${E2E_BROWSER_HEADLESS:-true}
E2E_SERVER_PORT=${E2E_SERVER_PORT:-8080}
E2E_REPORT_DIR=${E2E_REPORT_DIR:-"./test_reports"}

echo -e "${BLUE}🚀 ProvChainOrg End-to-End Testing Suite${NC}"
echo "========================================"
echo "Configuration:"
echo "  Timeout: ${E2E_TIMEOUT}s"
echo "  Parallel: ${E2E_PARALLEL}"
echo "  Headless: ${E2E_BROWSER_HEADLESS}"
echo "  Port: ${E2E_SERVER_PORT}"
echo "  Report Dir: ${E2E_REPORT_DIR}"
echo ""

# Create report directory
mkdir -p "${E2E_REPORT_DIR}"

# Function to run tests with timing
run_test_suite() {
    local test_name="$1"
    local test_file="$2"
    
    echo -e "${YELLOW}📋 Running ${test_name}...${NC}"
    local start_time=$(date +%s)
    
    if cargo test --test "${test_file}" -- --nocapture > "${E2E_REPORT_DIR}/${test_file}.log" 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo -e "${GREEN}✅ ${test_name} completed in ${duration}s${NC}"
        return 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo -e "${RED}❌ ${test_name} failed after ${duration}s${NC}"
        echo "   Check log: ${E2E_REPORT_DIR}/${test_file}.log"
        return 1
    fi
}

# Function to check prerequisites
check_prerequisites() {
    echo -e "${BLUE}🔍 Checking prerequisites...${NC}"
    
    # Check Rust installation
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Cargo not found. Please install Rust.${NC}"
        exit 1
    fi
    
    # Check Chrome/Chromium for browser tests
    if ! command -v google-chrome &> /dev/null && ! command -v chromium-browser &> /dev/null; then
        echo -e "${YELLOW}⚠️  Chrome/Chromium not found. Browser tests may fail.${NC}"
    fi
    
    # Check if project builds
    echo "Building project..."
    if ! cargo build > "${E2E_REPORT_DIR}/build.log" 2>&1; then
        echo -e "${RED}❌ Project build failed. Check ${E2E_REPORT_DIR}/build.log${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Prerequisites check passed${NC}"
}

# Function to generate summary report
generate_summary() {
    local total_tests="$1"
    local passed_tests="$2"
    local failed_tests="$3"
    local total_duration="$4"
    
    local success_rate=$(( (passed_tests * 100) / total_tests ))
    
    cat > "${E2E_REPORT_DIR}/summary.md" << EOF
# End-to-End Test Summary

## Overview
- **Total Tests**: ${total_tests}
- **Passed**: ${passed_tests} ✅
- **Failed**: ${failed_tests} ❌
- **Success Rate**: ${success_rate}%
- **Total Duration**: ${total_duration}s

## Test Results

EOF

    # Add individual test results
    for log_file in "${E2E_REPORT_DIR}"/*.log; do
        if [[ -f "$log_file" ]]; then
            local test_name=$(basename "$log_file" .log)
            if grep -q "test result: ok" "$log_file"; then
                echo "- ✅ **${test_name}**: PASSED" >> "${E2E_REPORT_DIR}/summary.md"
            else
                echo "- ❌ **${test_name}**: FAILED" >> "${E2E_REPORT_DIR}/summary.md"
            fi
        fi
    done

    cat >> "${E2E_REPORT_DIR}/summary.md" << EOF

## Performance Metrics

$(if [[ -f "${E2E_REPORT_DIR}/performance.json" ]]; then
    echo "Performance data available in performance.json"
else
    echo "No performance data collected"
fi)

## Logs

Individual test logs are available in the test_reports directory:
EOF

    for log_file in "${E2E_REPORT_DIR}"/*.log; do
        if [[ -f "$log_file" ]]; then
            echo "- $(basename "$log_file")" >> "${E2E_REPORT_DIR}/summary.md"
        fi
    done
}

# Main execution
main() {
    local start_time=$(date +%s)
    local total_tests=0
    local passed_tests=0
    local failed_tests=0
    
    # Check prerequisites
    check_prerequisites
    
    echo ""
    echo -e "${BLUE}🧪 Starting Test Execution${NC}"
    echo "=========================="
    
    # Test suites to run
    declare -a test_suites=(
        "User Journey Tests:e2e_user_journeys"
        "Web Interface Tests:e2e_web_interface" 
        "API Workflow Tests:e2e_api_workflows"
    )
    
    # Run each test suite
    for suite in "${test_suites[@]}"; do
        IFS=':' read -r test_name test_file <<< "$suite"
        total_tests=$((total_tests + 1))
        
        if run_test_suite "$test_name" "$test_file"; then
            passed_tests=$((passed_tests + 1))
        else
            failed_tests=$((failed_tests + 1))
        fi
        echo ""
    done
    
    # Calculate total duration
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    # Generate summary
    generate_summary "$total_tests" "$passed_tests" "$failed_tests" "$total_duration"
    
    # Print final summary
    echo -e "${BLUE}🎯 Final Summary${NC}"
    echo "==============="
    echo "Total Tests: $total_tests"
    echo -e "Passed: ${GREEN}$passed_tests${NC}"
    echo -e "Failed: ${RED}$failed_tests${NC}"
    echo "Duration: ${total_duration}s"
    echo "Report: ${E2E_REPORT_DIR}/summary.md"
    
    # Exit with appropriate code
    if [[ $failed_tests -eq 0 ]]; then
        echo -e "${GREEN}🎉 All tests passed!${NC}"
        exit 0
    else
        echo -e "${RED}🚨 Some tests failed. Check logs for details.${NC}"
        exit 1
    fi
}

# Handle script arguments
case "${1:-}" in
    --help|-h)
        echo "Usage: $0 [options]"
        echo ""
        echo "Options:"
        echo "  --help, -h          Show this help message"
        echo "  --quick, -q         Run quick test suite only"
        echo "  --verbose, -v       Enable verbose output"
        echo "  --no-browser        Skip browser tests"
        echo ""
        echo "Environment Variables:"
        echo "  E2E_TIMEOUT         Test timeout in seconds (default: 300)"
        echo "  E2E_PARALLEL        Run tests in parallel (default: true)"
        echo "  E2E_BROWSER_HEADLESS Run browser tests headless (default: true)"
        echo "  E2E_SERVER_PORT     Server port for tests (default: 8080)"
        echo "  E2E_REPORT_DIR      Report output directory (default: ./test_reports)"
        exit 0
        ;;
    --quick|-q)
        echo -e "${YELLOW}🏃 Running quick test suite...${NC}"
        # Run only essential tests
        cargo test --test e2e_api_workflows test_complete_data_ingestion_pipeline
        exit $?
        ;;
    --verbose|-v)
        set -x  # Enable verbose mode
        ;;
    --no-browser)
        export E2E_SKIP_BROWSER_TESTS=true
        ;;
esac

# Run main function
main "$@"
