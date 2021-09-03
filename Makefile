# Makefile for interface time

export project_dir=$(abspath $(shell pwd))
include ./interface.mk

all::
	cd rust && cargo build
