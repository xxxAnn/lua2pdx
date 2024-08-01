/// Grammar module
/// List of Grammars
/// 
/// Building Blocks:
/// Ignore EOS: [<EOS>]*
/// Simple List: [<Name/Literal> <Comma> <{Ignore EOS}>]*
/// Assign List: [<Name> <Equal> <Name/Literal> <Comma> <{Ignore EOS}>]*
/// Function Call: <Name> <LeftParen> <{Simple List}> <RightParen>
/// Table Body: <LeftCurly> <{Assign List}> <RightCurly>
/// 
/// Compilable:
/// Assignment: <Name> <Equal> <Name/Literal/{Function Call}/{Table Body}>