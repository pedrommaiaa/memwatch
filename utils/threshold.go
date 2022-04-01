package utils

import (
    "math/big"
)

const (
    DEFAULT = 5
)

var Threshold *big.Float


func LoadThreshold() {
    Threshold = big.NewFloat(DEFAULT)
}
