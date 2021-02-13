putStrLn :: String -> IO ()
print     :: Show a => a -> IO ()

main :: IO ()

main = do 
          putStrLen "Enter  a number greater than 3: "
          x <- readLn
          print(x > 3)
