;; Functions
(function_definition
  name: (identifier) @function.name
  parameters: (parameters) @function.parameters
  body: (block) @function.body) @function.definition

;; Classes
(class_definition
  name: (identifier) @class.name
  body: (block) @class.body) @class.definition

;; Methods
(class_definition
  body: (block
    (function_definition
      name: (identifier) @method.name
      parameters: (parameters
        (identifier) @method.self .)
      body: (block) @method.body) @method.definition))

;; Imports
(import_statement) @import.statement
(import_from_statement) @import.from_statement

;; Function calls
(call
  function: [
    (identifier) @call.function
    (attribute
      attribute: (identifier) @call.method)
  ]) @call.expression

;; Variable assignments
(assignment
  left: (_) @variable.name
  right: (_) @variable.value) @variable.assignment