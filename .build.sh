#!/bin/sh

MAKE=make
if [ "$OS" == "Windows_NT" ]; then
	MAKE=mingw32-make
fi

if [ ! -f src/link.rs ]; then
	$MAKE link
fi
