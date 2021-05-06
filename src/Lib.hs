module Lib (
        module Lib 
      , module Data.Maybe
      , module Data.Either
      , module Data.List
    ) where

import Data.Text (Text)
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import qualified Data.Map as M
import Data.List
import Data.Either
import Data.Maybe

someFunc :: IO ()
someFunc = putStrLn "someFunc"
