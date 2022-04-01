package utils


import (
    "log"
    "math/big"

    "github.com/ethereum/go-ethereum/common/hexutil"
)

const (
	WEI   = 1
	GWEI  = 1e9
	ETHER = 1e18
)

const (
	COLOR_RESET  = "\033[0m"
	COLOR_RED    = "\033[31m"
	COLOR_GREEN  = "\033[32m"
	COLOR_YELLOW = "\033[33m"
	COLOR_BLUE   = "\033[34m"
)

func ValueInEth(hexValueInWei string) *big.Float {
    decodedValue, err := hexutil.DecodeBig(hexValueInWei)
    if err != nil {
        log.Fatalf("error decoding value: %s\n", hexValueInWei)
    }
    f := new(big.Float).SetInt(decodedValue)
    return new(big.Float).Quo(f, big.NewFloat(ETHER))
}

func ValueInGwei(hexValueInWei string) *big.Int {
    decodedValue, err := hexutil.DecodeBig(hexValueInWei)
    if err != nil {
        log.Fatalf("error decoding value: %s\n", hexValueInWei)
    }

    f := new(big.Float).SetInt(decodedValue)
    gweiFloat := new(big.Float).Quo(f, big.NewFloat(GWEI))
    gweiInt, _ := gweiFloat.Int(nil)

    return gweiInt
}

func RedString(s string) string {
	return COLOR_RED + s + COLOR_RESET
}

func GreenString(s string) string {
	return COLOR_GREEN + s + COLOR_RESET
}

func YellowString(s string) string {
	return COLOR_YELLOW + s + COLOR_RESET
}

func BlueString(s string) string {
	return COLOR_BLUE + s + COLOR_BLUE
}
