#!/bin/bash

# colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # no color

# get root directory
ROOT_DIR=$(pwd)
SCRIPTS_DIR="${ROOT_DIR}/canister/scripts"

# utility functions
generate_declarations() {
    echo -e "${BLUE}Generating declarations...${NC}"

    # ask for environment
    echo -e "${YELLOW}Select environment:${NC}"
    echo -e "1) Development"
    echo -e "2) Staging"
    echo -e "3) Production"
    read -r env_choice

    case $env_choice in
    1) env="dev" ;;
    2) env="staging" ;;
    3) env="prod" ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        return 1
        ;;
    esac

    # execute the script
    if [ -f "${SCRIPTS_DIR}/declaration/generate.sh" ]; then
        bash "${SCRIPTS_DIR}/declaration/generate.sh" "$env"
    else
        echo -e "${RED}Error: declaration/generate.sh script not found${NC}"
        return 1
    fi
}

reinstall_staging() {
    echo -e "${BLUE}Reinstalling staging environment...${NC}"

    # confirm action
    echo -e "${YELLOW}This will reinstall the staging environment. Are you sure? (y/N)${NC}"
    read -r confirm
    if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Operation cancelled${NC}"
        return 0
    fi

    # execute the script
    if [ -f "${SCRIPTS_DIR}/deployments/reinstall-staging.sh" ]; then
        bash "${SCRIPTS_DIR}/deployments/reinstall-staging.sh"
    else
        echo -e "${RED}Error: deployments/reinstall-staging.sh script not found${NC}"
        return 1
    fi
}

upgrade_emr() {
    echo -e "${BLUE}Upgrading EMR...${NC}"

    # collect version information
    echo -e "${YELLOW}Enter EMR version to upgrade to:${NC}"
    read -r version

    # confirm action
    echo -e "${YELLOW}This will upgrade EMR to version $version. Are you sure? (y/N)${NC}"
    read -r confirm
    if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Operation cancelled${NC}"
        return 0
    fi

    # execute the script
    if [ -f "${SCRIPTS_DIR}/upgrades/upgrade-emr.sh" ]; then
        bash "${SCRIPTS_DIR}/upgrades/upgrade-emr.sh" "$version"
    else
        echo -e "${RED}Error: upgrades/upgrade-emr.sh script not found${NC}"
        return 1
    fi
}

# print menu
print_menu() {
    echo -e "\n${YELLOW}=== Medblock Utils Menu ===${NC}"
    echo -e "${BLUE}1)${NC} Generate Declarations"
    echo -e "${BLUE}2)${NC} Reinstall Staging"
    echo -e "${BLUE}3)${NC} Upgrade EMR"
    echo -e "${BLUE}q)${NC} Quit"
    echo -e "\n${YELLOW}Choose an option:${NC} "
}

# main menu loop
while true; do
    print_menu
    read -r opt

    case $opt in
    1) generate_declarations ;;
    2) reinstall_staging ;;
    3) upgrade_emr ;;
    q | Q)
        echo -e "${GREEN}Goodbye!${NC}"
        exit 0
        ;;
    *) echo -e "${RED}Invalid option${NC}" ;;
    esac

    echo -e "\nPress any key to continue..."
    read -n 1 -s -r
    clear
done
