-- Fibonacci function 
function fibonacci(number)
  if number < 2 then
    return number 
  else 
    return fibonacci(number - 1) + fibonacci(number - 2)
  end
end

-- Print Function 
function print_fibonacci(number)
  for i = 0, number do
    print(fibonacci(i))
  end
end

-- Main Function
function main()
  print_fibonacci(10)
end

-- Call Main Function 
main()
