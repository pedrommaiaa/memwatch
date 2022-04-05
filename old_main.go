package main

import (
    "context"
    "log"
    "os"
    "fmt"

    "github.com/pedrommaiaa/memwatch/utils"

    "github.com/joho/godotenv"
    "github.com/ethereum/go-ethereum/common"
    "github.com/ethereum/go-ethereum/rpc"
    "github.com/ethereum/go-ethereum/ethclient/gethclient"
)

func main() {
    err := godotenv.Load(".env")
    utils.LoadThreshold()
    if err != nil {
        log.Fatalf("Some error occured. Err: %s", err);
    }

    nodeURL := os.Getenv("NODE_URL")
    if len(nodeURL) == 0 {
        log.Fatal("Node not set.")
    }

    // Dial creates a new client for the given URL.
    client, err := rpc.Dial(nodeURL)
    if err != nil {
        log.Fatal(err)
    }

    // ethclient provides a client for the Ethereum RPC API.
    geth := gethclient.New(client)
    defer client.Close()

    logs := make(chan common.Hash)

    sub, err := geth.SubscribePendingTransactions(context.Background(), logs)
    if err != nil {
        log.Fatal(err)
    }

    for {
        select {
            case err := <-sub.Err():
                log.Fatal(err)
            case tx := <-logs:
                go printTx(client, tx)
        }
    }
}

func printTx(client *rpc.Client, tx common.Hash) {
    var transaction map[string]interface{}
    client.Call(&transaction, "eth_getTransactionByHash", tx)

    if transaction == nil || transaction["value"] == "0x0" {
        return
    }

    if value, ok := transaction["value"].(string); ok {
        etherValue := utils.ValueInEth(value)
 
        // Transaction value filter
        //switch comparison := etherValue.Cmp(utils.Threshold); comparison {
		//    case 0:
		//	    return
		//    case -1:
		//	    return
		//}

		gasString, ok := transaction["gasPrice"].(string)
		if ok != true {
			fmt.Println(utils.RedString("Invalid gas price"))
		}
		gasPrice := utils.ValueInGwei(gasString)

		fmt.Printf(utils.GreenString("*** NEW TX DETECTED ***\n"))
		fmt.Printf(utils.YellowString("TX HASH: "))
		fmt.Println(transaction["hash"])
		fmt.Printf(utils.YellowString("FROM: "))
		fmt.Println(transaction["from"])
		fmt.Printf(utils.YellowString("TO: "))
		fmt.Println(transaction["to"])
		fmt.Printf(utils.YellowString("GAS PRICE: "))
		fmt.Printf("%s Gwei\n", gasPrice.String())
		fmt.Printf(utils.YellowString("ETH: "))
		fmt.Println(etherValue)
	} else {
		return
	}
}
