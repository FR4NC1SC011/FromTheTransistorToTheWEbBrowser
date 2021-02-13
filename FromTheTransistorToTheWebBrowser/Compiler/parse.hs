module ParseWhile where
import System.IO
import Control.Monad
import Text.ParserCombinators.Parsec
import Text.ParserCombinators.Parsec.Expr
import Text.ParserCombinators.Parsec.Language
import qualified Text.ParserCombinators.Parsec.Token as Token


data BExpr = BoolConst Bool                   --Boolean Expressions--
           | Not BExpr
           | BBinary BBinOp BExpr BExpr
           | RBinary RBinOp AExpr AExpr
           deriving(Show)


data BBinOp = And | Or deriving(Show)         --Binary Boolean Operators--

data RBinOp = Greater | Less deriving(Show)   --Relational Operators--
