;; Functions
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @function.name
    parameters: (parameter_list) @function.parameters)
  body: (compound_statement) @function.body) @function.definition

;; Methods
(function_definition
  declarator: (function_declarator
    declarator: (field_identifier) @method.name
    parameters: (parameter_list) @method.parameters)
  body: (compound_statement) @method.body) @method.definition

;; Classes
(class_specifier
  name: (type_identifier) @class.name
  body: (field_declaration_list) @class.body) @class.definition

;; Structs
(struct_specifier
  name: (type_identifier) @struct.name
  body: (field_declaration_list) @struct.body) @struct.definition

;; Enums
(enum_specifier
  name: (type_identifier) @enum.name
  body: (enumerator_list) @enum.body) @enum.definition

;; Namespaces
(namespace_definition
  name: (identifier) @namespace.name
  body: (declaration_list) @namespace.body) @namespace.definition

;; Includes
(preproc_include) @include.directive

;; Function calls
(call_expression
  function: [
    (identifier) @call.function
    (field_expression
      field: (field_identifier) @call.method)
  ]) @call.expression

;; Variable declarations
(declaration
  type: (_)
  declarator: (init_declarator
    declarator: (identifier) @variable.name
    value: (_)? @variable.value)) @variable.declaration