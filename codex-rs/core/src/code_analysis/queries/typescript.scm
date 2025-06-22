;; Functions
(function_declaration
  name: (identifier) @function.name
  parameters: (formal_parameters) @function.parameters
  body: (statement_block) @function.body) @function.definition

(function
  parameters: (formal_parameters) @function.parameters
  body: (statement_block) @function.body) @function.expression

(arrow_function
  parameters: [(formal_parameters) @function.parameters (identifier) @function.parameter]
  body: [
    (statement_block) @function.body
    (_) @function.body
  ]) @function.arrow

;; Methods
(method_definition
  name: [(property_identifier) (computed_property_name)] @method.name
  parameters: (formal_parameters) @method.parameters
  body: (statement_block) @method.body) @method.definition

;; Classes
(class_declaration
  name: (identifier) @class.name
  body: (class_body) @class.body) @class.definition

;; Interfaces
(interface_declaration
  name: (identifier) @interface.name
  body: (object_type) @interface.body) @interface.definition

;; Types
(type_alias_declaration
  name: (identifier) @type.name
  value: (_) @type.value) @type.definition

;; Imports
(import_statement) @import.statement
(import_specifier) @import.specifier
(namespace_import) @import.namespace

;; Function calls
(call_expression
  function: [
    (identifier) @call.function
    (member_expression
      property: (property_identifier) @call.method)
  ]) @call.expression

;; Variable declarations
(variable_declaration
  (variable_declarator
    name: (identifier) @variable.name
    value: (_)? @variable.value)) @variable.declaration