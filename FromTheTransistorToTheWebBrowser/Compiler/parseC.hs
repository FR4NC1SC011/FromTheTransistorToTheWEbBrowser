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

data AExpr = Var String                       --Arithmetic Expressions--
           | IntConst Integer
           | Neg AExpr
           | ABinary ABinOp AExpr AExpr
           deriving(Show)

data ABinOp = Add                             --Arithmetic Operators--
            | Substract
            | Multiply
            | Divide
            deriving(Show)

data Stmt = Seq [Stmt]                        --Statements--
          | Assign String AExpr
          | If BExpr Stmt Stmt
          | While BExpr Stmt
          | Skip
          | LeftBrace
          | RightBrace
          deriving(Show)


-- LEXER --

languageDef = 
  emptyDef { Token.commentStart = "/*"
           , Token.commentEnd = "*/"
           , Token.commentLine = "//"
           , Token.identStart = letter
           , Token.identLetter = alphaNum
           , Token.reservedNames = [ "if"
                                   , "then"
                                   , "else"
                                   , "while"
                                   , "do"
                                   , "skip"
                                   , "true"
                                   , "false"
                                   , "not"
                                   , "and"
                                   , "or"
                                   , "{"
                                   , "}"
                                   ]

         , Token.reservedOpNames = [ "+", "-", "*", "/", "=", "<", ">"
                                   , "&&", "||", "!=", "and", "or", "not"
                                   ]
           }


lexer = Token.makeTokenParser languageDef

identifier = Token.identifier lexer
reserved = Token.reserved     lexer
reservedOp = Token.reservedOp lexer
parens = Token.parens         lexer
integer = Token.integer       lexer
semi = Token.semi             lexer
whiteSpace = Token.whiteSpace lexer


-- MAIN PARSER --

whileParser :: Parser Stmt
whileParser = whiteSpace >> statement

statement :: Parser Stmt
statement = parens statement
         <|> sequenceOfStmt

sequenceOfStmt = do 
    list <- (sepBy1 statement' semi)
    return $ if length list == 1 then head list else Seq list


statement' :: Parser Stmt
statement' = ifStmt
          <|> whileStmt
          <|> skipStmt
          <|> assignStmt
          <|> leftBraceStmt
          <|> rightBraceStmt



ifStmt :: Parser Stmt
ifStmt = do 
    reserved "if"
    cond <- bExpression
    reserved "{"
    stmt1 <- statement
    reserved "}"
    stmt2 <- statement
    return $ If cond stmt1 stmt2

whileStmt :: Parser Stmt
whileStmt = do 
  reserved "while"
  cond <- bExpression
  reserved "do"
  stmt <- statement
  return $ While cond stmt

assignStmt :: Parser Stmt
assignStmt = do 
  var <- identifier
  reservedOp "="
  expr <- aExpression
  return $ Assign var expr

skipStmt :: Parser Stmt
skipStmt = reserved "skip" >> return Skip

leftBraceStmt :: Parser Stmt
leftBraceStmt = reserved "{" >> return LeftBrace 

rightBraceStmt :: Parser Stmt
rightBraceStmt = reserved "}" >> return RightBrace



aExpression :: Parser AExpr
aExpression = buildExpressionParser aOperators aTerm

bExpression :: Parser BExpr
bExpression = buildExpressionParser bOperators bTerm


aOperators = [ [Prefix (reservedOp "-" >> return(Neg))] 
             , [Infix (reservedOp "*" >> return (ABinary Multiply)) AssocLeft,
                Infix (reservedOp "/" >> return (ABinary Divide)) AssocLeft]
             , [Infix (reservedOp "+" >> return (ABinary Add)) AssocLeft, 
                Infix (reservedOp "-" >> return (ABinary Substract)) AssocLeft]
             ]


bOperators = [ [Prefix (reservedOp "not" >> return (Not))] 
              ,[Infix (reservedOp "and" >> return (BBinary And)) AssocLeft
              ,Infix (reservedOp "or" >> return (BBinary Or)) AssocLeft]
              ,[Prefix (reservedOp "!=" >> return (Not))] 
              ,[Infix (reservedOp "&&" >> return (BBinary And)) AssocLeft
              ,Infix (reservedOp "||" >> return (BBinary Or)) AssocLeft]
             ]

aTerm = parens aExpression
     <|>liftM Var identifier
     <|>liftM IntConst integer

bTerm = parens bExpression
     <|>(reserved "true" >> return (BoolConst True))
     <|>(reserved "false" >> return (BoolConst False))
     <|> rExpression


rExpression = do
  a1 <- aExpression
  op <- relation
  a2 <- aExpression
  return $ RBinary op a1 a2


relation = (reservedOp ">" >> return Greater)
        <|>(reservedOp "<" >> return Less)





parseFile :: String -> IO Stmt
parseFile file = do
  program <- readFile file
  case parse whileParser "" program of 
    Left e -> print e >> fail "parse error"
    Right r -> return r
