<start> ::= <compilationUnit>;

<compilationUnit> ::= "void main() { " <topLevelDeclarations> " }";

<topLevelDeclarations> ::= <topLevelDeclaration> <topLevelDeclarations>
                         | <topLevelDeclaration>;

<topLevelDeclaration> ::= <classDeclaration>
                        | <functionDeclaration>
                        | <variableDeclaration> ";";

<classDeclaration> ::= "class " <identifier> "{" <classMembers> " };";

<classMembers> ::= <classMember> <classMembers>;

<classMember> ::= <variableDeclaration> ";"
                | <functionDeclaration>;

<functionDeclaration> ::= <type_> " " <identifier> "(" <parameterList> ") " <block>
                        | "void " <identifier> "(" <parameterList> ") " <block>;

<parameterList> ::= <parameter> ", " <parameterList>
                  | <parameter>;

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

<numericLiteral> ::= <digit>* "." <digit>* | <digit>*;

<stringLiteral> ::= "\"" <stringCharacter>* "\"" ;

<booleanLiteral> ::= "true" | "false";

<identifier> ::= <identifier_start> <identifier_parts>*
               | <identifier_start>;

<identifier_start> ::= <letter>*;

<identifier_parts> ::= <identifier_part> <identifier_parts>
                     | <identifier_part>;

<identifier_part> ::= <identifier_start>
                    | <digit>;

<stringCharacter> ::= <letter> | <digit> | "_" | " ";

<letter_lower> ::= "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z";

<letter_upper> ::= "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z";

<letter> ::= <letter_lower>
           | <letter_upper>;

<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";


