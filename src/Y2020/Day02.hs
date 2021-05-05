{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE RecordWildCards #-}

module Y2020.Day02 where

import qualified Text.Megaparsec.Char.Lexer as L
import Text.Megaparsec.Char
import Text.Megaparsec (Parsec, many, parseMaybe)
import Data.Text (Text) 
import Data.Void
import Data.Maybe
import qualified  Data.Vector as V
type Parser = Parsec Void String 

data Box = Box 
        { hi :: Int
        , lo :: Int
        , c :: Char 
        , s :: String 
        } 
  deriving (Show)

pInt :: Parser Int
pInt = L.decimal

p :: Parser Box
p  = do
  lo <- pInt 
  char '-'
  hi <- pInt 
  space
  c  <- letterChar
  char ':'
  space
  s <- many letterChar
  return Box {..}

g :: [Box] -> [Box]
g = filter helper 
  where helper x = let l  = lo x
                       h  = hi x
                       c' = c x 
                       s' = s x
                       in (s' !! l) == c' && (s' !! h) == c' 
f :: [Box] -> [Box]
f = filter helper 
  where helper x = let l  = lo x
                       h  = hi x
                       c' = c x 
                       s' = s x
                       in (l <= (length (filter (== c') s')) && h >= (length (filter (== c') s')))
inp :: IO ()
inp = do
  xs <- length . f . map (fromJust . parseMaybe p) . lines <$> readFile "../data_2020/day02.txt"
  ys <- length . g . map (fromJust . parseMaybe p) . lines <$> readFile "../data_2020/day02.txt"
  print "1:" 
  print xs
  putStrLn ""
  print "2:"
  print ys

