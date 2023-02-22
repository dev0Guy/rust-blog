#!/bin/bash

function inject_env_var_into_toml_file (){
    local ENV_FILE=$1;
    local TOML_FILE=$2;
    [ -f "$ENV_FILE" ] && echo  "✅ [$ENV_FILE] Found" ||  "❌ [$ENV_FILE] Does not exist";
    [ -f "$TOML_FILE" ] && echo "✅ [$TOML_FILE] Found" ||  "❌ [$TOML_FILE] Does not exist";
    
}


inject_env_var_into_toml_file ".env" "./Trunk.toml"