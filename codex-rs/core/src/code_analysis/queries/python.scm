;; Class definitions
(class_definition
  name: (identifier) @name.definition.class) @definition.class

;; Function definitions
(function_definition
  name: (identifier) @name.definition.function) @definition.function

;; Method definitions (functions inside classes)
(class_definition
  body: (block
    (function_definition
      name: (identifier) @name.definition.method) @definition.method))

;; Variable assignments
(assignment
  left: (identifier) @name.definition.variable) @definition.variable

;; Import statements - from X import Y
(import_from_statement
  module_name: (dotted_name) @name.reference.import
  name: (dotted_name
    (identifier) @name.definition.import)) @definition.import

;; Import statements - import X
(import_statement
  name: (dotted_name
    (identifier) @name.definition.import)) @definition.import

;; Function calls
(call
  function: [
      (identifier) @name.reference.call
      (attribute
        attribute: (identifier) @name.reference.call)
  ]) @reference.call

;; Type annotations and class usage
(type
  (identifier) @name.reference.type) @reference.type

;; Class instantiation (constructor calls)
(call
  function: (identifier) @name.reference.class) @reference.class

;; Attribute access
(attribute
  object: (identifier) @name.reference.usage
  attribute: (identifier)) @reference.usage

;; Variable usage
(identifier) @name.reference.usage @reference.usage