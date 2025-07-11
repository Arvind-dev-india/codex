; Class definitions
(class_specifier name: (type_identifier) @name.definition.class) @definition.class

; Struct definitions (treated as classes in C++)
(struct_specifier name: (type_identifier) @name.definition.class body:(_)) @definition.class

; Union definitions
(declaration type: (union_specifier name: (type_identifier) @name.definition.class)) @definition.class
(union_specifier name: (type_identifier) @name.definition.class) @definition.class

; Enum definitions
(enum_specifier name: (type_identifier) @name.definition.type) @definition.type

; Type definitions
(type_definition declarator: (type_identifier) @name.definition.type) @definition.type

; Function definitions
(function_definition 
  declarator: (function_declarator 
    declarator: (identifier) @name.definition.function)) @definition.function

; Function declarations
(function_declarator declarator: (identifier) @name.definition.function) @definition.function
(function_declarator declarator: (field_identifier) @name.definition.function) @definition.function

; Method definitions (functions within classes)
(function_definition 
  declarator: (function_declarator 
    declarator: (qualified_identifier 
      scope: (namespace_identifier) @scope 
      name: (identifier) @name.definition.method))) @definition.method

; Constructor definitions
(function_definition 
  declarator: (function_declarator 
    declarator: (qualified_identifier 
      name: (identifier) @name.definition.constructor))) @definition.constructor

; Template function definitions
(template_declaration 
  (function_definition 
    declarator: (function_declarator 
      declarator: (identifier) @name.definition.function))) @definition.function

; Template class definitions
(template_declaration 
  (class_specifier name: (type_identifier) @name.definition.class)) @definition.class

; Namespace definitions
(namespace_definition name: (namespace_identifier) @name.definition.module) @definition.module

; Method calls with member access (obj.method(), obj->method())
(call_expression 
  function: (field_expression 
    field: (field_identifier) @name.reference.call)) @reference.call

; Method calls with qualified names (Class::method())
(call_expression 
  function: (qualified_identifier 
    name: (identifier) @name.reference.call)) @reference.call

; Simple function calls (function())
(call_expression 
  function: (identifier) @name.reference.call) @reference.call

; Constructor calls with new
(new_expression 
  type: (type_identifier) @name.reference.constructor) @reference.constructor

; Type references in variable declarations
(declaration 
  type: (type_identifier) @name.reference.class) @reference.class

; Type references in parameter lists
(parameter_declaration 
  type: (type_identifier) @name.reference.class) @reference.class

; Template instantiations
(template_type 
  name: (type_identifier) @name.reference.class) @reference.class

; Base class references in inheritance
(base_class_clause 
  (type_identifier) @name.reference.class) @reference.class

; Namespace usage
(using_declaration 
  (qualified_identifier 
    scope: (namespace_identifier) @name.reference.module)) @reference.module

; Field access (obj.field, obj->field)
(field_expression 
  field: (field_identifier) @name.reference.field) @reference.field

; Qualified identifiers (namespace::identifier)
(qualified_identifier 
  scope: (namespace_identifier) @name.reference.module 
  name: (identifier) @name.reference.identifier) @reference.identifier