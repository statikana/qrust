#!/nix/store/yisa2lg79zcvgk4ck4yr6r0lz6j63hs3-bash-interactive-5.3p9/bin/bash
# CHANGE THIS FOR WHOEVER USES THIS ^^^^

set -euo pipefail

# don't forget to chmod +x ./comp

### VARS AND INFO
PROJECT_NAME='qrust' # whatever is in Cargo.toml

GRAMMAR='Qrust' # WITHOUT THE .g4 suffix
GRAMMAR_SUFFIX='g4'
GRAMMAR_LOWER_CASE='qrust' # for imports and stuff

ANTLR4_JAR='./.antlrbin/antlr4-4.8-2-SNAPSHOT-complete.jar'  # must be from rrevenantt/antlr4rust releases for -Dlanguage=Rust

LIBRS_TEXT="
pub mod ${GRAMMAR_LOWER_CASE}parser;

pub mod ${GRAMMAR_LOWER_CASE}lexer;

pub mod ${GRAMMAR_LOWER_CASE}listener;

pub mod ${GRAMMAR_LOWER_CASE}visitor;"

LIB_NAME='parse'
LIB_DIR="./$LIB_NAME" # root of package, not src/


mkdir -p ${LIB_DIR}

# generate the rust module to use in main executor. if this gives u errors, install the antlr4rust jar (GH rrevenantt/antlr4rust)
java -jar ${ANTLR4_JAR} -o ${LIB_DIR}/src -Dlanguage=Rust -visitor ${GRAMMAR}.${GRAMMAR_SUFFIX}

echo ${LIBRS_TEXT} > ${LIB_DIR}/src/lib.rs;

# fix any errors
cargo fix --lib -p $LIB_NAME --allow-dirty && cargo check && cargo build
