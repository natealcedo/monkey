package parser

import (
	"github.com/natealcedo/monkey/ast"
	"github.com/natealcedo/monkey/lexer"
	"testing"
)

func TestLetStatements(t *testing.T) {
	input := `
		let x = 5;
		let y = 10;
		let foobar = 838383`

	lex := lexer.New(input)
	parser := New(lex)

	program := parser.ParseProgram()

	if program == nil {
		t.Fatalf("ParseProgram() returned nil")
	}

	programStatementsLen := len(program.Statements)
	if programStatementsLen != 3 {
		t.Fatalf("program.Statements does not contain 3 statements. got=%d", programStatementsLen)
	}

	tests := []struct {
		expectedIdentifier string
	}{
		{"x"},
		{"y"},
		{"foobar"},
	}

	for i, tt := range tests {
		stmt := program.Statements[i]
		if !testLetStatement(t, stmt, tt.expectedIdentifier) {
			return
		}
	}
}

func testLetStatement(t *testing.T, stmt ast.Statement, name string) bool {
	if stmt.TokenLiteral() != "let" {
		t.Errorf("s.TokenLiteral not 'let'. got=%q", stmt.TokenLiteral())
		return false
	}

	letStmt, ok := stmt.(*ast.LetStatement)
	if !ok {
		t.Errorf("s not *ast.LetStatement. got=%s", stmt)
		return false
	}

	if letStmt.Name.TokenLiteral() != name {
		t.Errorf("s.Name not '%s'. got=%s", name, letStmt.Name)
		return false
	}

	return true
}
