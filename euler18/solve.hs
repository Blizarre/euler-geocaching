-- https://projecteuler.net/problem=67
-- https://projecteuler.net/problem=18

parsePyramid :: String -> [ [Integer] ]
parsePyramid text = reverse $ map pyramidRows $ lines text
    where pyramidRows = map read . words

solveLines :: [Integer] -> [Integer] -> [Integer]
solveLines (b:c:xc) (a:xa) =
    let newElement = a + max b c
        in newElement : solveLines (c:xc) xa
solveLines [_] [] = []


solvePyramid :: [ [Integer] ] -> [ [Integer] ]
solvePyramid (a : b : xs) = 
    let currentLine = solveLines a b
    in currentLine : solvePyramid (currentLine:xs)
solvePyramid (a : xs) = []

extractSolution :: [ [Integer] ] -> Integer
extractSolution pyramid = head . last $ pyramid


main :: IO ()
main = do
    text <- getContents
    print . extractSolution . solvePyramid . parsePyramid $ text

