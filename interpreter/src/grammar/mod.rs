/// Grammar module
/// List of Grammars
/// 
/// Building Blocks:
/// Ignore EOS: [<EOS>]*
/// Name List: [<Name> <Comma> <{Ignore EOS}>]*
/// Simple List: [<Name/Literal> <Comma> <{Ignore EOS}>]*
/// Assign List: [<Name> <Equal> <Name/Literal> <Comma> <{Ignore EOS}>]*
/// Function Call: <Name> <LeftParen> <{Simple List}> <RightParen>
/// Table Body: <LeftCurly> <{Assign List}> <RightCurly>
/// 
/// Special:
/// Compilables: [<{Compilable}>]*
/// 
/// Compilable:
/// Root: <{Compilables}>
/// Assignment: <Name> <Equal> <Name/Literal/{Function Call}/{Table Body}>
/// Function Definition: <Function> <Name> <LeftParen> <{Name List}> <RightParen> <{Compilables}> <End>