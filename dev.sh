#!/bin/bash

# colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # no color

# get root directory
ROOT_DIR=$(pwd)

# function to log with colors
log() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# function to check environment files
check_env_files() {
    log "Checking environment files..."

    # function to check env files against their examples
    check_single_env() {
        local env_file="$1"
        local example_file="$2"
        local folder_name="$3"

        # check if .env exists
        if [ ! -f "$env_file" ]; then
            error "$folder_name/.env file is missing. Please create one from $folder_name/.env.example"
        fi

        # get required keys from .env.example (lines starting with letters, excluding comments and empty lines)
        local required_keys=$(grep -E '^[A-Za-z]' "$example_file" | cut -d'=' -f1)

        # check each required key
        local missing_keys=()
        for key in $required_keys; do
            if ! grep -q "^${key}=" "$env_file"; then
                missing_keys+=("$key")
            fi
        done

        # if there are missing keys, show error
        if [ ${#missing_keys[@]} -ne 0 ]; then
            error "$folder_name/.env is missing required variables: ${missing_keys[*]}"
        fi
    }

    # check web environment
    check_single_env \
        "$ROOT_DIR/final_demo/web/.env" \
        "$ROOT_DIR/final_demo/web/.env.example" \
        "web"

    # check internal-dashboard environment
    check_single_env \
        "$ROOT_DIR/internal-dashboard/.env" \
        "$ROOT_DIR/internal-dashboard/.env.example" \
        "internal-dashboard"

    # check pwa environment
    check_single_env \
        "$ROOT_DIR/final_demo/pwa/.env" \
        "$ROOT_DIR/final_demo/pwa/.env.example" \
        "pwa"

    log "Environment files check passed"
}

# function to check if required tools are installed
check_requirements() {
    log "Checking requirements..."

    command -v dfx >/dev/null 2>&1 || error "dfx is required but not installed"
    command -v bun >/dev/null 2>&1 || error "bun is required but not installed"
    command -v rustup >/dev/null 2>&1 || error "rustup is required but not installed"
}

# function to start the ic replica and deploy canisters
deploy_local() {
    log "Deploying local environment..."
    cd "$ROOT_DIR"
    # generate declarations first
    bash utils.sh --generate-declarations --all || error "Failed to generate declarations"
    cd "$ROOT_DIR/canister/scripts/deployments"
    bash local.sh --background || error "Failed to deploy local environment"
    cd "$ROOT_DIR"
}

# function to select package manager
select_package_manager() {
    printf "${BLUE}[INFO]${NC} Select your preferred package manager:\n"
    select pm in "bun" "npm" "yarn" "pnpm"; do
        if [ "$pm" = "bun" ] || [ "$pm" = "npm" ] || [ "$pm" = "yarn" ] || [ "$pm" = "pnpm" ]; then
            echo "$pm"
            return
        else
            warning "Invalid selection. Please choose a number from 1-4."
        fi
    done
}

# function to install dependencies with selected package manager
install_deps() {
    local pm=$1
    if [ "$pm" = "bun" ]; then
        bun install
    elif [ "$pm" = "npm" ]; then
        npm install
    elif [ "$pm" = "yarn" ]; then
        yarn install
    elif [ "$pm" = "pnpm" ]; then
        pnpm install
    fi
}

# function to start dev server with selected package manager
start_dev() {
    local pm=$1
    local port=$2
    local prefix=$3
    if [ "$pm" = "bun" ]; then
        PORT=$port bun dev 2>&1 | sed "s/^/[$prefix] /" &
    elif [ "$pm" = "npm" ]; then
        PORT=$port npm run dev 2>&1 | sed "s/^/[$prefix] /" &
    elif [ "$pm" = "yarn" ]; then
        PORT=$port yarn dev 2>&1 | sed "s/^/[$prefix] /" &
    elif [ "$pm" = "pnpm" ]; then
        PORT=$port pnpm dev 2>&1 | sed "s/^/[$prefix] /" &
    fi
}

# function to start the web app
start_webapp() {
    log "Starting web application..."
    cd "$ROOT_DIR/final_demo/web"
    install_deps $PACKAGE_MANAGER
    start_dev $PACKAGE_MANAGER 3012 "WEBAPP"
    cd "$ROOT_DIR"
}

# function to start the internal dashboard
start_dashboard() {
    log "Starting internal dashboard..."
    cd "$ROOT_DIR/internal-dashboard"
    install_deps $PACKAGE_MANAGER
    start_dev $PACKAGE_MANAGER 3011 "WEBADMIN"
    cd "$ROOT_DIR"
}

# function to start the pwa
start_pwa() {
    log "Starting PWA application..."
    cd "$ROOT_DIR/final_demo/pwa"
    install_deps $PACKAGE_MANAGER
    start_dev $PACKAGE_MANAGER 3010 "PWA"
    cd "$ROOT_DIR"
}

# function to cleanup processes
cleanup() {
    log "Cleaning up processes..."
    # kill all processes running on our ports
    for port in 3010 3011 3012; do
        lsof -ti:$port | xargs kill -9 2>/dev/null || true
    done

    if [ -f /tmp/dfx.pid ]; then
        kill $(cat /tmp/dfx.pid) 2>/dev/null || true
        rm /tmp/dfx.pid
    else
        dfx stop || true
    fi
}

# trap ctrl-c and call cleanup
trap cleanup INT TERM

# main execution
main() {
    if [ "$1" = "--canister" ]; then
        # check requirements
        check_requirements

        # start services
        deploy_local

        # keep running until ctrl+c
        log "Canister is running. Press Ctrl+C to stop."
        wait

    elif [ "$1" = "--front" ]; then
        # check environment files first
        check_env_files

        # select package manager for frontend projects
        export PACKAGE_MANAGER=$(select_package_manager)
        log "Using ${PACKAGE_MANAGER} as package manager"

        # start frontends
        start_webapp
        start_dashboard
        start_pwa

        # show urls
        log "Frontend applications are ready!"
        log "PWA running at: http://localhost:3010"
        log "Internal dashboard running at: http://localhost:3011"
        log "Web app running at: http://localhost:3012"

        # keep running until ctrl-c
        log "Press Ctrl+C to stop all frontends."

        # wait for all background jobs
        while true; do
            sleep 1
            if ! jobs %% >/dev/null 2>&1; then
                break
            fi
        done

    elif [ -z "$1" ]; then
        # original behavior - run everything
        check_env_files
        check_requirements

        # select package manager for frontend projects
        export PACKAGE_MANAGER=$(select_package_manager)
        log "Using ${PACKAGE_MANAGER} as package manager"

        # start services
        deploy_local
        start_webapp
        start_dashboard
        start_pwa

        # keep script running and show urls
        log "Development environment is ready!"
        log "PWA running at: http://localhost:3010"
        log "Internal dashboard running at: http://localhost:3011"
        log "Web app running at: http://localhost:3012"
        log "IC replica running at: http://localhost:4943"
        log "Candid UI and other endpoints are listed above ⬆️"

        # wait for user input to stop
        read -p "Press any key to stop all services..."
    else
        print_usage
        exit 1
    fi
}

# run main function
main "$@"
