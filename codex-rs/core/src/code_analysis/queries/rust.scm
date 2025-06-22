;; Functions
(function_item
  name: (identifier) @function.name
  parameters: (parameters) @function.parameters
  body: (block) @function.body) @function.definition

;; Methods
(impl_item
  (function_item
    name: (identifier) @method.name
    parameters: (parameters) @method.parameters
    body: (block) @method.body)) @method.definition

;; Structs
(struct_item
  name: (type_identifier) @struct.name
  body: (field_declaration_list)? @struct.body) @struct.definition

;; Enums
(enum_item
  name: (type_identifier) @enum.name
  body: (enum_variant_list) @enum.body) @enum.definition

;; Traits
(trait_item
  name: (type_identifier) @trait.name
  body: (declaration_list) @trait.body) @trait.definition

;; Implementations
(impl_item
  trait: (type_identifier)? @impl.trait
  type: (type_identifier) @impl.type) @impl.definition

;; Modules
(mod_item
  name: (identifier) @module.name
  body: (declaration_list)? @module.body) @module.definition

;; Use statements
(use_declaration) @use.declaration

;; Function calls
(call_expression
  function: [
    (identifier) @call.function
    (field_expression
      field: (field_identifier) @call.method)
  ]) @call.expression

;; Variable declarations
(let_declaration
  pattern: (identifier) @variable.name
  type: (type_identifier)? @variable.type
  value: (_)? @variable.value) @variable.declaration