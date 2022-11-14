-- Factorial function 
function fact(n)
  if n < 0 then
    return "No negative Factorial"
  end
  if n == 0 then
    return 1
  else
    return n * fact(n - 1)
  end
end

-- Norm 
function norm(x, y)
  return (x * x + y * y)^(0.5)
end 

-- For Loop
print("For loop!")
for i = 0, 5 do 
  print(i)
end

-- While Loop
print("While loop!")
i = 10
while i > 0 do 
  print(i)
  i = i - 1
end

-- Print Statements
print("Hello World!")
print(fact(5))
print(fact(-1))
print(norm(3, 4))

