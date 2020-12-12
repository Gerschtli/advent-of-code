package main

type status int

const (
	statusFloor status = iota
	statusFree
	statusOccupied
)

type seats [][]status
