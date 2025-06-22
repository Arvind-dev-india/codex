;; Methods
(method_declaration
  name: (identifier) @method.name
  parameters: (formal_parameters) @method.parameters
  body: (block) @method.body) @method.definition

;; Classes
(class_declaration
  name: (identifier) @class.name
  body: (class_body) @class.body) @class.definition

;; Interfaces
(interface_declaration
  name: (identifier) @interface.name
  body: (interface_body) @interface.body) @interface.definition

;; Enums
(enum_declaration
  name: (identifier) @enum.name
  body: (enum_body) @enum.body) @enum.definition

;; Fields
(field_declaration
  declarator: (variable_declarator
    name: (identifier) @field.name
    value: (_)? @field.value)) @field.definition

;; Packages
(package_declaration
  name: (scoped_identifier) @package.name) @package.declaration

;; Imports
(import_declaration) @import.declaration

;; Method invocations
(method_invocation
  name: (identifier) @call.method
  arguments: (argument_list) @call.arguments) @call.expression

;; Variable declarations
(local_variable_declaration
  declarator: (variable_declarator
    name: (identifier) @variable.name
    value: (_)? @variable.value)) @variable.declaration