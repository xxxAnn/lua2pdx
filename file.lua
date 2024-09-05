function hello(a, b) do
    c = a * 2
    if c > b then
      c = b
    end
    return c + b*a
end