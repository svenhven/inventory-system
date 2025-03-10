module Stock where

data Stock = Stock {
    itemId :: Int,
    quantity :: Int
} deriving (Show, Eq)

updateStock :: Stock -> Int -> Stock
updateStock stock newQuantity = stock { quantity = newQuantity }