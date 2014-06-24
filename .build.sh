#!/bin/sh

MAKE=make
if [ "$OS" == "Windows_NT" ]; then
	MAKE=mingw32-make
fi

$MAKE link
