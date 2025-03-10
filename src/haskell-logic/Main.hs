module Main where

import Stock
import Sync

main :: IO ()
main = do
    let localStocks = [Stock 1 100, Stock 2 200]
    let remoteStocks = [Stock 1 150, Stock 2 250]
    let syncedStocks = syncData localStocks remoteStocks
    print syncedStocks