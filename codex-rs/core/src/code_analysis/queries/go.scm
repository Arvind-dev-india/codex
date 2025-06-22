;; Functions
(function_declaration
  name: (identifier) @function.name
  parameters: (parameter_list) @function.parameters
  body: (block) @function.body) @function.definition

;; Methods
(method_declaration
  name: (field_identifier) @method.name
  parameters: (parameter_list) @method.parameters
  body: (block) @method.body) @method.definition

;; Structs
(type_declaration
  (type_spec
    name: (type_identifier) @struct.name
    type: (struct_type) @struct.type)) @struct.definition

;; Interfaces
(type_declaration
  (type_spec
    name: (type_identifier) @interface.name
    type: (interface_type) @interface.type)) @interface.definition

;; Packages
(package_clause
  (package_identifier) @package.name) @package.declaration

;; Imports
(import_declaration) @import.declaration
(import_spec
  path: (interpreted_string_literal) @import.path) @import.spec

;; Function calls
(call_expression
  function: [
    (identifier) @call.function
    (selector_expression
      field: (field_identifier) @call.method)
  ]) @call.expression

;; Variable declarations
(var_declaration
  (var_spec
    name: (identifier) @variable.name
    value: (_)? @variable.value)) @variable.declaration