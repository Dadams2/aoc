import System.IO
import Data.List
import Data.Char (isSpace)

-- Function to split a string by multiple spaces
splitBySpaces :: String -> [String]
splitBySpaces = filter (not . all isSpace) . groupBy (\a b -> isSpace a == isSpace b)

-- Function to count occurrences of an element in a list
count :: Eq a => a -> [a] -> Int
count x = length . filter (== x)

main :: IO ()
main = do
    -- Read the file
    contents <- readFile "day1.in"
    
    -- Parse the file
    let linesOfFiles = lines contents
        (l1, l2) = foldr (\line (acc1, acc2) -> 
                            let vals = splitBySpaces line
                            in (read (vals !! 0) : acc1, read (vals !! 1) : acc2)
                         ) ([], []) linesOfFiles
    
    -- Calculate the first sum
    let sortedL1 = sort l1
        sortedL2 = sort l2
        sum1 = sum [abs (a - b) | (a, b) <- zip sortedL1 sortedL2]
    
    -- Calculate the second sum
    let sum2 = sum [i * count i l2 | i <- l1]
    
    -- Print the results
    print sum1
    print sum2