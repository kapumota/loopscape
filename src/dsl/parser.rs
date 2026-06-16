use super::ast::{OrchestrationCommand, OrchestrationProgram};
use super::command::CommandKind;
use super::error::DslError;
use super::lexer::lex;
use super::token::{Token, TokenKind};

/// Parser minimo para convertir tokens del DSL en un programa de orquestacion.
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Crea un parser desde una lista de tokens ya generada por el lexer.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Convierte los tokens en un programa de orquestacion.
    pub fn parse_program(&mut self) -> Result<OrchestrationProgram, DslError> {
        let mut program = OrchestrationProgram::empty();

        loop {
            self.skip_separators();

            match self.current_kind() {
                Some(TokenKind::Eof) | None => break,
                Some(TokenKind::Command(_)) => {
                    let command = self.parse_command()?;
                    program.push(command);
                }
                _ => {
                    return Err(DslError::invalid_program(
                        "se esperaba un comando DSL al inicio de la linea",
                    ));
                }
            }
        }

        Ok(program)
    }

    fn parse_command(&mut self) -> Result<OrchestrationCommand, DslError> {
        let token = self.current().ok_or_else(|| {
            DslError::invalid_program("se esperaba un comando DSL antes del fin del archivo")
        })?;
        let source_line = token.span.line;
        let kind = match &token.kind {
            TokenKind::Command(kind) => *kind,
            _ => {
                return Err(DslError::invalid_program(
                    "se esperaba un comando DSL al inicio de la linea",
                ));
            }
        };

        self.advance();

        let arguments = match kind {
            CommandKind::Plan => self.parse_plan_arguments()?,
            _ => self.parse_regular_arguments()?,
        };

        let command = OrchestrationCommand::new(kind, arguments)?.with_source_line(source_line);
        Ok(command)
    }

    fn parse_regular_arguments(&mut self) -> Result<Vec<String>, DslError> {
        let mut arguments = Vec::new();

        while !self.is_line_end() {
            match self.current_kind() {
                Some(TokenKind::Identifier(value)) => {
                    arguments.push(value.clone());
                    self.advance();
                }
                Some(TokenKind::Number(value)) => {
                    arguments.push(value.to_string());
                    self.advance();
                }
                Some(TokenKind::StringLiteral(value)) => {
                    arguments.push(value.clone());
                    self.advance();
                }
                Some(TokenKind::Arrow) => {
                    return Err(DslError::invalid_program(
                        "la flecha solo se permite dentro de /plan",
                    ));
                }
                Some(TokenKind::Command(_)) => {
                    return Err(DslError::invalid_program(
                        "se encontro un comando antes de terminar la linea actual",
                    ));
                }
                _ => break,
            }
        }

        self.consume_line_end();
        Ok(arguments)
    }

    fn parse_plan_arguments(&mut self) -> Result<Vec<String>, DslError> {
        let mut arguments = Vec::new();
        let mut expecting_step = true;

        while !self.is_line_end() {
            match self.current_kind() {
                Some(TokenKind::Identifier(value)) => {
                    if !expecting_step {
                        return Err(DslError::invalid_program(
                            "se esperaba una flecha entre pasos de /plan",
                        ));
                    }
                    arguments.push(value.clone());
                    expecting_step = false;
                    self.advance();
                }
                Some(TokenKind::Number(value)) => {
                    if !expecting_step {
                        return Err(DslError::invalid_program(
                            "se esperaba una flecha entre pasos de /plan",
                        ));
                    }
                    arguments.push(value.to_string());
                    expecting_step = false;
                    self.advance();
                }
                Some(TokenKind::StringLiteral(value)) => {
                    if !expecting_step {
                        return Err(DslError::invalid_program(
                            "se esperaba una flecha entre pasos de /plan",
                        ));
                    }
                    arguments.push(value.clone());
                    expecting_step = false;
                    self.advance();
                }
                Some(TokenKind::Arrow) => {
                    if expecting_step {
                        return Err(DslError::invalid_program(
                            "se encontro una flecha sin paso previo en /plan",
                        ));
                    }
                    expecting_step = true;
                    self.advance();
                }
                Some(TokenKind::Command(_)) => {
                    return Err(DslError::invalid_program(
                        "se encontro un comando antes de terminar la linea actual",
                    ));
                }
                _ => break,
            }
        }

        if expecting_step && !arguments.is_empty() {
            return Err(DslError::invalid_program(
                "el comando /plan termina con una flecha sin paso final",
            ));
        }

        self.consume_line_end();
        Ok(arguments)
    }

    fn skip_separators(&mut self) {
        while matches!(
            self.current_kind(),
            Some(TokenKind::Newline | TokenKind::Comment(_))
        ) {
            self.advance();
        }
    }

    fn consume_line_end(&mut self) {
        if matches!(self.current_kind(), Some(TokenKind::Comment(_))) {
            self.advance();
        }

        if matches!(self.current_kind(), Some(TokenKind::Newline)) {
            self.advance();
        }
    }

    fn is_line_end(&self) -> bool {
        matches!(
            self.current_kind(),
            Some(TokenKind::Newline | TokenKind::Comment(_) | TokenKind::Eof) | None
        )
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn current_kind(&self) -> Option<&TokenKind> {
        self.current().map(|token| &token.kind)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }
}

/// Convierte texto DSL en un programa de orquestacion.
pub fn parse(input: &str) -> Result<OrchestrationProgram, DslError> {
    let tokens = lex(input)?;
    Parser::new(tokens).parse_program()
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::dsl::CommandKind;

    #[test]
    fn parser_builds_minimal_program() {
        let program = parse(
            r#"/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify checklist_final
/terminate when verified"#,
        )
        .expect("debe parsear flujo minimo");

        assert_eq!(program.commands.len(), 5);
        assert_eq!(program.command_count_by_kind(CommandKind::Goal), 1);
        assert_eq!(program.command_count_by_kind(CommandKind::Plan), 1);
        assert_eq!(program.command_count_by_kind(CommandKind::Delegate), 1);
        assert_eq!(program.command_count_by_kind(CommandKind::Verify), 1);
        assert_eq!(program.command_count_by_kind(CommandKind::Terminate), 1);
        assert!(program.has_termination());
    }

    #[test]
    fn parser_preserves_plan_steps() {
        let program =
            parse("/plan buscar -> clasificar -> asistir").expect("debe parsear pasos de plan");
        let command = &program.commands[0];

        assert_eq!(command.kind, CommandKind::Plan);
        assert_eq!(command.arguments, vec!["buscar", "clasificar", "asistir"]);
    }

    #[test]
    fn parser_accepts_strings_and_numbers_as_arguments() {
        let program = parse("/verify \"checklist final\"\n/terminate 10")
            .expect("debe aceptar cadenas y numeros");

        assert_eq!(program.commands[0].arguments, vec!["checklist final"]);
        assert_eq!(program.commands[1].arguments, vec!["10"]);
    }

    #[test]
    fn parser_ignores_blank_lines_and_comments() {
        let program =
            parse("\ncomentario libre no es valido").expect_err("debe rechazar texto sin comando");

        assert_eq!(
            program.to_string(),
            "se esperaba un comando DSL al inicio de la linea"
        );

        let program = parse("# comentario\n/goal rescatar\n\n# otro comentario\n/verify listo")
            .expect("debe ignorar comentarios y lineas vacias");

        assert_eq!(program.commands.len(), 2);
    }

    #[test]
    fn parser_rejects_arrow_outside_plan() {
        let error = parse("/delegate sector_a -> worker_1")
            .expect_err("debe rechazar flecha fuera de plan");

        assert_eq!(
            error.to_string(),
            "la flecha solo se permite dentro de /plan"
        );
    }

    #[test]
    fn parser_rejects_plan_without_arrow_between_steps() {
        let error = parse("/plan buscar clasificar").expect_err("debe exigir flecha entre pasos");

        assert_eq!(
            error.to_string(),
            "se esperaba una flecha entre pasos de /plan"
        );
    }

    #[test]
    fn parser_rejects_trailing_plan_arrow() {
        let error = parse("/plan buscar ->").expect_err("debe rechazar plan con flecha final");

        assert_eq!(
            error.to_string(),
            "el comando /plan termina con una flecha sin paso final"
        );
    }

    #[test]
    fn parser_stores_source_line() {
        let program = parse("\n\n/goal rescatar").expect("debe conservar linea de origen");

        assert_eq!(program.commands[0].source_line, Some(3));
    }
}
