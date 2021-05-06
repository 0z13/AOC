module Day03 where

import Lib
import qualified Data.Text as T
import qualified Data.Text.IO as TIO


findTheThing :: [String] -> Int -> Int
findTheThing xs x = go xs x 0 
    

go :: [String] -> Int -> Int -> Int
go []     _ res = res
go (l:xs) x res = if cycle l !! x == '#' then go xs (x+3) (res+1)
                      else go xs (x+3) res


gos :: [String] -> Int -> Int -> Int
gos []     _ res = res
gos (l:_:xs) x res = if cycle l !! x == '#' then go xs (x+3) (res+1)
                      else go xs (x+3) res


readF :: IO ()
readF = do
  dat <- lines <$> readFile "../data_2020/day03.txt"
  let xs = findTheThing dat 3
  let ys = findTheThing dat 1
  let zs = findTheThing dat 5
  let ws = findTheThing dat 7
  let ty = gos (dat) 1 0
  print (xs * ys * zs * ws * ty)

readl :: IO ()
readl = do
  dat <- length . lines <$> readFile "../data_2020/day03.txt"
  print dat
 
