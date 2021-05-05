{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE RecordWildCards #-}

module Dag2 where

import qualified Text.Megaparsec.Char.Lexer as L
import Text.Megaparsec.Char
import Text.Megaparsec (Parsec, many, parse)
import Data.Text (Text) 
import Data.Void

type Parser = Parsec Void Text

data Box = Box 
        { hi :: Integer
        , lo :: Integer
        , c :: Char 
        , s :: String 
        } 
  deriving (Show)

pInt :: Parser Integer
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

