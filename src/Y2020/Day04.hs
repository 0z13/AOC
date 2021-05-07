{-# LANGUAGE OverloadedStrings #-}

module Y2020.Day04 where

import Lib
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
main4 :: IO ()
main4 = do
  xs <- go . T.splitOn "\n"  <$> TIO.readFile "../data_2020/day04.txt"
  print $ head . tail $ fmap T.unpack xs

go :: [T.Text] -> [T.Text] 
go = f [] [] 

f :: [T.Text] -> [T.Text] -> [T.Text] -> [T.Text]
f res   _    []  =  res
f res acc (x:xs) 
  | x == "" = f (acc ++ res) [] xs
  | otherwise = f res (x : acc) xs
