function test(base)
    -- print("Hello, World!")
    return base .. "hi"
end

pdxlua.register_scripted_effect("test", test)