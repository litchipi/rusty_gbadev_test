#!/bin/bash

set -e

clear
cargo +nightly check --target="thumbv4t-none-eabi"

function get_game_version {
	python3 -c "print ((int('$1'.split('.')[0])*10) + (int('$1'.split('.')[1])*4) + (int('$1'.split('.')[2])*1))"
}

function progress {
	echo -e "\n\033[96;1m> $@ \033[0m"
}

rm -f ./*.sav ./*.gba
VERSION=$(cat Cargo.toml | grep "version" | cut -d "\"" -f 2)
GAMEVERSION=$(get_game_version $VERSION)
PROJECT_NAME=$(cat Cargo.toml | grep "name" | cut -d "\"" -f 2)
MAKERCODE="01"
GAMECODE="02"

echo "$GAMEVERSION $MAKERCODE $GAMECODE $PROJECT_NAME"
if [[ -z "$GAMEVERSION" || -z "$MAKERCODE" || -z "$GAMECODE" || -z "$PROJECT_NAME" ]]; then
	echo "Failed to get configuration from Cargo.toml"
	echo "Make sure the \"name\" field and \"version\" field of the manifest are present and fileld"
	exit 1;
fi

clear
progress "Building $PROJECT_NAME v$VERSION"
cargo +nightly build --target="thumbv4t-none-eabi" --release

progress "Creating the GBA ROM"
mkdir -p target/release/
arm-none-eabi-objcopy -g --verbose -O binary target/thumbv4t-none-eabi/release/$PROJECT_NAME ./$PROJECT_NAME.gba

progress "Patching the ROM with MAKERCODE=$MAKERCODE, GAMECODE=$GAMECODE, GAMEVERSION=$GAMEVERSION"
gbafix -c$GAMECODE -m$MAKERCODE -r$GAMEVERSION -p $PROJECT_NAME.gba

progress "Starting the game"
mgba -l 15 $PROJECT_NAME.gba
