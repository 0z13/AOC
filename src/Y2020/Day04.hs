{-# LANGUAGE OverloadedStrings #-}

module Y2020.Day04 where

import Lib
import Data.List.Split
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
main4 :: IO ()
main4 = do
  xs <-  map (\. splitOn " " . unlines . lines <$> readFile "../data_2020/day04.txt"
  print xs 


