<start> ::= <compilationUnit>;

<compilationUnit> ::= <topLevelDeclarations>;

<topLevelDeclarations> ::= <topLevelDeclaration> <topLevelDeclarations>
                         | <topLevelDeclaration>;

<topLevelDeclaration> ::= <classDeclaration>
                        | <functionDeclaration>
                        | <variableDeclaration> ";";

<classDeclaration> ::= "class " <identifier> " { " <classMembers> " }";

<classMembers> ::= <classMember> <classMembers>
                 | "";

<classMember> ::= <variableDeclaration> ";"
                | <functionDeclaration>;

<functionDeclaration> ::= <type_> " " <identifier> "(" <parameterList> ") " <block>
                        | "void " <identifier> "(" <parameterList> ") " <block>;

<parameterList> ::= <parameter> ", " <parameterList>
                  | <parameter>
                  | "";

<parameter> ::= <type_> " " <identifier>;

<block> ::= "{ " <statements> " }";

<statements> ::= <statement> <statements>
               | "";

<statement> ::= <variableDeclaration> ";"
              | <ifStatement>
              | <whileStatement>
              | <returnStatement>
              | <expressionStatement>
              | <block>;

<variableDeclaration> ::= "var " <identifier> " = " <expr>
                        | <type_> " " <identifier> " = " <expr>
                        | <type_> " " <identifier>;

<type_> ::= "int" | "double" | "String" | "bool";

<ifStatement> ::= "if (" <expr> ") " <statement> " else " <statement>
                | "if (" <expr> ") " <statement>;

<whileStatement> ::= "while (" <expr> ") " <statement>;

<returnStatement> ::= "return " <expr> ";"
                    | "return;";

<expressionStatement> ::= <expr> ";";

<expr> ::= <identifier> " = " <expr>
         | <binaryExpr>
         | <primary>;

<binaryExpr> ::= <primary> " " <binaryOperator> " " <primary>;

<binaryOperator> ::= "+" | "-" | "*" | "/" | "==" | "!=" | "<" | ">";

<primary> ::= <identifier>
            | <literal>
            | <functionCall>
            | "(" <expr> ")";

<functionCall> ::= <identifier> "(" <argumentList> ")";

<argumentList> ::= <expr> ", " <argumentList>
                 | <expr>
                 | "";

<literal> ::= <numericLiteral> | <stringLiteral> | <booleanLiteral>;

<numericLiteral> ::= "42" | "3.14" | "0" | "1";

<stringLiteral> ::= "\"hello\"" | "\"world\"";

<booleanLiteral> ::= "true" | "false";

<identifier> ::= "x" | "y" | "z" | "main" | "myFunc" | "MyClass" | "print" | "value";
