#!/bin/bash
# License header check script for REChain SDK
# Ensures all source files have proper license headers

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# License header that should be present
LICENSE_HEADER="//! REChain SDK - Copyright (C) $(date +%Y) REChain Network Solutions LLC
//! Licensed under the GPL-3.0 License"

# Check if file has license header
check_file() {
    local file="$1"

    # Skip certain file types
    if [[ "$file" =~ \.(lock|sum|sha256|sig)$ ]] || [[ "$file" =~ target/ ]] || [[ "$file" =~ node_modules/ ]]; then
        return 0
    fi

    # Check Rust files
    if [[ "$file" =~ \.rs$ ]]; then
        # Check if file starts with license header (allow for comments and whitespace)
        if ! head -5 "$file" | grep -q "REChain SDK"; then
            print_error "Missing license header in $file"
            return 1
        fi
    fi

    # Check TypeScript/JavaScript files
    if [[ "$file" =~ \.(ts|js)$ ]]; then
        if ! head -3 "$file" | grep -q "REChain SDK"; then
            print_error "Missing license header in $file"
            return 1
        fi
    fi

    # Check Python files
    if [[ "$file" =~ \.py$ ]]; then
        if ! head -3 "$file" | grep -q "REChain SDK"; then
            print_error "Missing license header in $file"
            return 1
        fi
    fi

    return 0
}

# Add license header to file
add_license_header() {
    local file="$1"

    print_warning "Adding license header to $file"

    # Create temporary file with license header
    local temp_file=$(mktemp)

    # Add appropriate license header based on file type
    if [[ "$file" =~ \.rs$ ]]; then
        echo "$LICENSE_HEADER" > "$temp_file"
        echo "" >> "$temp_file"
    elif [[ "$file" =~ \.(ts|js)$ ]]; then
        echo "/*" > "$temp_file"
        echo " * REChain SDK - Copyright (C) $(date +%Y) REChain Network Solutions LLC" >> "$temp_file"
        echo " * Licensed under the GPL-3.0 License" >> "$temp_file"
        echo " */" >> "$temp_file"
        echo "" >> "$temp_file"
    elif [[ "$file" =~ \.py$ ]]; then
        echo "# REChain SDK - Copyright (C) $(date +%Y) REChain Network Solutions LLC" > "$temp_file"
        echo "# Licensed under the GPL-3.0 License" >> "$temp_file"
        echo "" >> "$temp_file"
    fi

    # Append original file content
    cat "$file" >> "$temp_file"

    # Replace original file
    mv "$temp_file" "$file"
}

# Main function
main() {
    local has_errors=0

    print_success "Checking license headers in REChain SDK..."

    # Find all relevant source files
    while IFS= read -r -d '' file; do
        if ! check_file "$file"; then
            has_errors=1
            add_license_header "$file"
        fi
    done < <(find . -type f \( -name "*.rs" -o -name "*.ts" -o -name "*.js" -o -name "*.py" \) -not -path "./target/*" -not -path "./node_modules/*" -print0)

    if [[ $has_errors -eq 0 ]]; then
        print_success "All files have proper license headers"
        return 0
    else
        print_error "Some files were missing license headers (fixed automatically)"
        return 1
    fi
}

# Handle script arguments
case "${1:-}" in
    "help"|"-h"|"--help")
        echo "Usage: $0 [check|fix]"
        echo ""
        echo "Commands:"
        echo "  check    Check for license headers (default)"
        echo "  fix      Add missing license headers"
        echo ""
        echo "Examples:"
        echo "  $0              # Check license headers"
        echo "  $0 fix          # Add missing license headers"
        exit 0
        ;;
    "fix")
        main
        ;;
    *)
        main
        ;;
esac