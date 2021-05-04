module Day01 where

import Control.Monad
import Data.List
import Data.Maybe

main :: IO ()
main = do
  dat <-fmap (fmap (read :: String -> Int)) $ lines <$> readFile "../data_2020/day01.txt"
  print "First solution:" 
  print (fromJust $ solve1 dat) 
  putStrLn "Second solution:"
  print (fromJust $ solve2 dat)

solve1 :: [Int] -> Maybe Int 
solve1 xs = listToMaybe $ do 
  x:rest <- tails xs
  y    <- rest 
  guard (x + y == 2020)
  pure (x*y) 

solve2 :: [Int] -> Maybe Int 
solve2 xs = listToMaybe $ do 
  x:rest <- tails xs
  z:zs <- tails rest
  y    <- zs 
  guard (x + y + z == 2020)
  pure (x*y*z) 

