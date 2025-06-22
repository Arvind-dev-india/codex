;; Methods
(method_declaration
  name: (identifier) @method.name
  parameters: (parameter_list) @method.parameters
  body: (block) @method.body) @method.definition

;; Classes
(class_declaration
  name: (identifier) @class.name
  body: (declaration_list) @class.body) @class.definition

;; Interfaces
(interface_declaration
  name: (identifier) @interface.name
  body: (declaration_list) @interface.body) @interface.definition

;; Structs
(struct_declaration
  name: (identifier) @struct.name
  body: (declaration_list) @struct.body) @struct.definition

;; Enums
(enum_declaration
  name: (identifier) @enum.name
  body: (enum_member_declaration_list) @enum.body) @enum.definition

;; Properties
(property_declaration
  name: (identifier) @property.name
  accessors: (accessor_list) @property.accessors) @property.definition

;; Fields
(field_declaration
  (variable_declaration
    (variable_declarator
      name: (identifier) @field.name))) @field.definition

;; Namespaces
(namespace_declaration
  name: (qualified_name) @namespace.name
  body: (declaration_list) @namespace.body) @namespace.definition

;; Using directives
(using_directive) @using.directive

;; Method invocations
(invocation_expression
  expression: [
    (identifier) @call.function
    (member_access_expression
      name: (identifier) @call.method)
  ]) @call.expression

;; Variable declarations
(variable_declaration
  (variable_declarator
    name: (identifier) @variable.name
    value: (_)? @variable.value)) @variable.declaration